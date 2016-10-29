use std::fmt;
use std::collections::HashMap;
use std::sync::Mutex;

use rustc_serialize::{Encoder, Encodable};
use rustc_serialize::json::as_json;

lazy_static!{
    pub static ref DATAMGR: Mutex<DataMgr> = {
        Mutex::new(DataMgr::new())
    };
}

/// World
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, RustcEncodable)]
pub enum World {
    /// Fire Peaks
    Fire,
    /// Burning Sands
    Sand,
    /// Green
    Grass,
    /// EW
    Ice,
    /// Special Event
    SpecialEvent,
}

impl World {
    /// Get world from internal integer
    pub fn from_int(num: u64) -> Self {
        match num {
            0 => World::Grass,
            1 => World::Sand,
            2 => World::Ice,
            3 => World::Fire,
            4 => World::SpecialEvent,
            _ => panic!("Unrecognized world number {}", num),
        }
    }
}

/// Castle data
#[derive(Debug, Hash, Eq, PartialEq, Clone, RustcEncodable)]
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
#[derive(Debug, Hash, Eq, PartialEq, Clone, RustcEncodable)]
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
        write!(f, "{}", as_json(self))
    }
}

/// Data manager
#[derive(Debug, RustcEncodable)]
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
            },
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
