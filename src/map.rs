use slog::*;

use serde_json::value::Value;

use error::{Error, ErrorKind};
use data::{User, Castle, World};

macro_rules! try_field{
    ($data: expr, $field: expr) => {
        try!(::serde_json::ser::to_string(&$data.get($field)))
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

    /// Metadata
    pub uap: String,

    pub oi: String,
    pub ai: String,

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

        let data = data.as_object().unwrap().clone();

        let world = World::from_int(data.get("KID").unwrap().as_u64().unwrap());
        let uap = try_field!(data, "uap");
        let oi = try_field!(data, "OI");
        let ai = try_field!(data, "AI");

        let mut users = Vec::new();
        let mut castles = Vec::new();
        let mut castle_names = Vec::new();

        for user in data.get("OI").unwrap().as_array().unwrap() {
            let user = user.as_object().unwrap();
            let user_id = user.get("OID").unwrap().as_u64().unwrap();
            users.push(User {
                id: user_id,
                username: user.get("N").unwrap().as_str().map(ToString::to_string),
                own_alliance: false,
            });

            let ap = user.get("AP").unwrap().as_array().unwrap();
            let vp = user.get("VP").unwrap().as_array().unwrap();

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

        let ai_ = data.get("AI").unwrap().as_array().unwrap();
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
            uap: uap,

            oi: oi,
            ai: ai,

            users: users,
            castles: castles,
            castle_names: castle_names,
        };
        Ok(gaa)
    }
}
