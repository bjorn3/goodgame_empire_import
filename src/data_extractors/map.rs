use serde_json::value::Value;
use serde_json::de::from_str;

use error::{Result, ResultExt};
use data::{User, Castle, World};

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
    pub fn parse(data: String) -> Result<Self> {
        #[derive(Deserialize)]
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        /// self
        struct _Self {
            KID: World,
            OI: Vec<_OI__>,
            AI: Vec<Value>,
        }

        #[derive(Deserialize)]
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        /// OI []
        struct _OI__ {
            OID: u64,
            N: String,
            AP: Vec<Value>,
            VP: Vec<Value>,
        }

        let data = data.trim_matches('%');
        let obj: _Self = from_str(&data).chain_err(
            || "failed to deserialize gaa",
        )?;

        let world: World = obj.KID;

        let mut users = Vec::new();
        let mut castles = Vec::new();
        let mut castle_names = Vec::new();

        for user in obj.OI {
            users.push(User {
                id: user.OID,
                username: Some(user.N.clone()),
                own_alliance: false,
            });

            castles.extend_from_slice(&user.AP
                .iter()
                .chain(user.VP.iter())
                .filter_map(|castle| {
                    let castle = castle.as_array().unwrap();
                    if castle.len() < 3 {
                        return None;
                    }
                    Some(Castle {
                        id: castle[1].as_u64().unwrap(),
                        owner_id: Some(user.OID),
                        name: None,
                        x: Some(castle[2].as_u64().unwrap()),
                        y: Some(castle[3].as_u64().unwrap()),
                        world: Some(world),
                    })
                })
                .collect::<Vec<_>>());
        }

        for castle in obj.AI {
            let castle = castle.as_array().unwrap();
            if castle.len() < 10 {
                //warn!(::slog_scope::logger(), "ignoring to short castle {}", Value::Array(castle.to_owned()));
                continue;
            }
            let id = castle[3].as_u64();
            if id.is_none() {
                continue;
            }
            let id = id.unwrap();
            let name = ::get_name_from_slice(castle);

            trace!(::slog_scope::logger(), "  process castle";
                "castle" => ::serde_json::ser::to_string(castle).unwrap_or_else(|err|format!("{:?}", err)),
                "id" => id,
                "name" => name.clone());
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

pub fn extract(
    obj: String,
    _con: &mut ::connection::Connection,
    data_mgr: &mut ::data::DataMgr,
) -> Result<()> {
    let gaa = Gaa::parse(obj).chain_err(|| "Couldnt read gaa packet")?;
    for castle in gaa.castles.iter() {
        data_mgr.add_castle(castle.clone());
    }
    for castle in gaa.castle_names.iter() {
        data_mgr.add_castle(castle.clone());
    }
    for user in gaa.users.iter() {
        data_mgr.users.insert(user.id, user.clone());
    }
    Ok(())
}
