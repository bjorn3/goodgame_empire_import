use std::fmt;
use std::collections::HashMap;
use std::sync::Mutex;

use serde::de::{Deserialize, Deserializer, Visitor};

lazy_static!{
    pub static ref DATAMGR: Mutex<DataMgr> = {
        Mutex::new(DataMgr::new())
    };
}

/// World
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize)]
pub enum World {
    /// Fire Peaks
    Fire = 3,
    /// Burning Sands
    Sand = 1,
    /// Green
    Grass = 0,
    /// EW
    Ice = 2,
    /// Special Event
    SpecialEvent = 4,
}

impl Deserialize for World{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        struct WorldVisitor;

        impl Visitor for WorldVisitor {
            type Value = World;

            fn visit_u64<E: ::serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
                match v {
                    0 => Ok(World::Grass),
                    1 => Ok(World::Sand),
                    2 => Ok(World::Ice),
                    3 => Ok(World::Fire),
                    4 => Ok(World::SpecialEvent),
                    _ => {
                        Err(::serde::de::Error::custom(
                            format_args!("Unrecognized world number {}", v)
                        ))
                    }
                }
            }

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "an integer between 0 and 5")
            }
        }

        deserializer.deserialize_u8(WorldVisitor)
    }
}

/// Castle data
#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize)]
pub struct Castle {
    /// Internal id
    pub id: u64,
    /// Internal owner id
    pub owner_id: Option<u64>,
    /// Castle name
    pub name: Option<String>,
    /// X position
    pub x: Option<u64>,
    /// Y position
    pub y: Option<u64>,
    /// World
    pub world: Option<World>,
}

/// User data
#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize)]
pub struct User {
    /// Internal id
    pub id: u64,
    /// Username
    pub username: Option<String>,
    /// Is it from your own alliance?
    pub own_alliance: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::serde_json::ser::to_string(self).unwrap())
    }
}

/// Data manager
#[derive(Debug, Serialize)]
pub struct DataMgr {
    /// List of castles
    pub castles: HashMap<u64, Castle>,
    pub users: HashMap<u64, User>,
}

macro_rules! same{
    ($a:expr, $b:expr, $field:ident) => {
        if !$b.is_none(){
            if let (Some(a), Some(b)) = ($a.$field, $b.unwrap().$field){
                assert_eq!(a, b, "{:?}, {:?}", $a, $b);
            }
        }
    }
}

impl DataMgr {
    /// Create new data manager
    pub fn new() -> Self {
        DataMgr {
            castles: HashMap::new(),
            users: HashMap::new(),
        }
    }

    /// Add the data of the specified castle
    pub fn add_castle(&mut self, castle: Castle) -> Castle {
        let mut castle = castle;
        let old_castle = self.castles.remove(&castle.id);

        same!(castle.clone(), old_castle.clone(), owner_id);
        same!(castle.clone(), old_castle.clone(), name);
        same!(castle.clone(), old_castle.clone(), x);
        same!(castle.clone(), old_castle.clone(), y);
        same!(castle.clone(), old_castle.clone(), world);

        match old_castle {
            Some(old_castle) => {
                castle.owner_id = castle.owner_id.or(old_castle.owner_id);
                castle.name = castle.name.or(old_castle.name);
                castle.x = castle.x.or(old_castle.x);
                castle.y = castle.y.or(old_castle.y);
                castle.world = castle.world.or(old_castle.world);
            }
            None => {}
        }
        self.castles.insert(castle.id, castle.clone());
        return castle;
    }

    /// Add the name of the specified user
    pub fn add_owner_name(&mut self, uid: u64, name: &str, own_alliance: bool) {
        let user = self.users.entry(uid).or_insert(User {
            id: uid,
            username: Some(name.to_owned()),
            own_alliance: false,
        });
        if own_alliance {
            user.own_alliance = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_castle() {
        use std::collections::HashMap;

        let mut data_mgr = DataMgr::new();
        data_mgr.add_castle(Castle {
            id: 42,
            owner_id: Some(84),
            name: Some("some name".to_string()),
            x: Some(10),
            y: None,
            world: Some(World::Grass),
        });
        data_mgr.add_castle(Castle {
            id: 42,
            owner_id: None,
            name: None,
            x: None,
            y: Some(20),
            world: None,
        });

        let mut expected_castles = HashMap::new();
        expected_castles.insert(42, Castle {
            id: 42,
            owner_id: Some(84),
            name: Some("some name".to_string()),
            x: Some(10),
            y: Some(20),
            world: Some(World::Grass),
        });
        
        assert_eq!(data_mgr.castles, expected_castles);
    }

    #[test]
    #[should_panic]
    fn conflicting_castle_world(){
        let mut data_mgr = DataMgr::new();
        data_mgr.add_castle(Castle {
            id: 42,
            owner_id: Some(84),
            name: Some("some name".to_string()),
            x: Some(10),
            y: None,
            world: Some(World::Grass),
        });
        data_mgr.add_castle(Castle {
            id: 42,
            owner_id: None,
            name: None,
            x: None,
            y: Some(20),
            world: Some(World::Fire),
        });
    }

    #[test]
    #[should_panic]
    fn conflicting_castle_position(){
        let mut data_mgr = DataMgr::new();
        data_mgr.add_castle(Castle {
            id: 42,
            owner_id: Some(84),
            name: Some("some name".to_string()),
            x: Some(10),
            y: None,
            world: Some(World::Grass),
        });
        data_mgr.add_castle(Castle {
            id: 42,
            owner_id: None,
            name: None,
            x: Some(11),
            y: Some(20),
            world: None,
        });
    }
}
