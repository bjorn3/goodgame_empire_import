use slog::*;

use serde_json::value::{Value, from_value};

use error::{Error, ErrorKind, ResultExt};
use data::{User, Castle, World};

macro_rules! try_field{
    ($data: expr, $field: expr) => {
        try!(::serde_json::ser::to_string(&$data.get($field)))
    };
}

macro_rules! get{
    ($a:ident.$field:ident as u64) => {
        $a.as_object()
            .ok_or(ErrorKind::InvalidFormat("Not a object".into()))?
            .get(stringify!($field))
            .ok_or(ErrorKind::InvalidFormat("Not a field".into()))?
            .as_u64()
            .ok_or(ErrorKind::InvalidFormat("Not an u64".into()))?;
    };

    ($a:ident.$field:ident as arr) => {
        $a.as_object()
            .ok_or(ErrorKind::InvalidFormat("Not a object".into()))?
            .get(stringify!($field))
            .ok_or(ErrorKind::InvalidFormat("Not a field".into()))?
            .as_array()
            .ok_or(ErrorKind::InvalidFormat("Not an array".into()))?;
    };

    ($a:ident.$field:ident as str) => {
        $a.as_object()
            .ok_or(ErrorKind::InvalidFormat("Not a object".into()))?
            .get(stringify!($field))
            .ok_or(ErrorKind::InvalidFormat("Not a field".into()))?
            .as_str()
            .ok_or(ErrorKind::InvalidFormat("Not a str".into()))?;
    };
    ($a:ident.$field:ident as Value) => {
        $a.as_object()
            .ok_or(ErrorKind::InvalidFormat("Not a object".into()))?
            .get(stringify!($field))
            .ok_or(ErrorKind::InvalidFormat("Not a field".into()))?
    };
}

trait Flatten<T> {
    fn flatten(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}

/// Map data
#[derive(Debug, Clone)]
pub struct Gaa {
    /// World
    pub kid: World,

    // Parsed
    pub users: Vec<User>,
    pub castles: Vec<Castle>,
    pub castle_names: Vec<Castle>,
}

impl Gaa {
    /// Parse text returned from the server
    pub fn parse(data: String, logger: Logger) -> Result<Self, Error> {
        let data = data.trim_matches('%');
        let data: Value = try!(::serde_json::de::from_str(&data));
        if !data.is_object() {
            return Err(ErrorKind::InvalidFormat("gaa not an object".into()).into());
        }

        //let data = data.as_object().unwrap().clone();

        let world: World = from_value(get!(data.KID as Value).clone()).chain_err(||"gaa KID is not a world id")?;

        let mut users = Vec::new();
        let mut castles = Vec::new();
        let mut castle_names = Vec::new();

        for user in get!(data.OI as arr) {
            //let user = user.as_object().unwrap();
            let user_id = get!(user.OID as u64);
            users.push(User {
                id: user_id,
                username: user.as_object().unwrap().get("N").unwrap().as_str().map(ToString::to_string),
                own_alliance: false,
            });

            let ap = get!(user.AP as arr);
            let vp = get!(user.VP as arr);

            castles.extend_from_slice(&ap.iter()
                .chain(vp.iter())
                .filter_map(|castle| {
                    let castle = castle.as_array().unwrap();
                    if castle.len() < 3 {
                        return None;
                    }
                    Some(Castle {
                        id: castle[1].as_u64().unwrap(),
                        owner_id: Some(user_id),
                        name: None,
                        x: Some(castle[2].as_u64().unwrap()),
                        y: Some(castle[3].as_u64().unwrap()),
                        world: Some(world),
                    })
                }).collect::<Vec<_>>());
        }

        let ai_ = get!(data.AI as arr);
        for castle in ai_.iter() {
            let castle = castle.as_array().unwrap();
            if castle.len() < 10 {
                continue;
            }
            let id = castle[3].as_u64();
            if id.is_none() {
                continue;
            }
            let id = id.unwrap();
            let name = castle.get(10).map(Value::as_str).map(Option::unwrap).map(ToString::to_string);

            trace!(logger, "  process castle"; "castle" => ::serde_json::ser::to_string(castle).unwrap_or_else(|err|format!("{:?}", err)), "id" => id, "name" => name);
            castle_names.push(Castle {
                id: id,
                owner_id: None,
                name: name,
                x: Some(castle[1].as_u64().unwrap()),
                y: Some(castle[2].as_u64().unwrap()),
                world: None,
            });
        }

        let gaa = Gaa {
            kid: world,

            users: users,
            castles: castles,
            castle_names: castle_names,
        };
        Ok(gaa)
    }
}
