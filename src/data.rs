use std::fmt;
use std::collections::HashMap;

use rustc_serialize::{Encoder, Encodable};
use rustc_serialize::json::as_json;

///World
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, RustcEncodable)]
pub enum World{
    ///Fire Peaks
    Fire,
    ///Burning Sands
    Sand,
    ///Green
    Grass,
    ///EW
    Ice,
    ///Special Event
    SpecialEvent
}

impl World{
    ///Get world from internal integer
    pub fn from_int(num: u64) -> Self{
        match num{
            0 => World::Grass,
            1 => World::Sand,
            2 => World::Ice,
            3 => World::Fire,
            4 => World::SpecialEvent,
            _ => panic!("Unrecognized world number {}", num)
        }
    }
}

///Castle data
#[derive(Debug, Hash, Eq, PartialEq, Clone, RustcEncodable)]
pub struct Castle{
    ///Internal id
    pub id: u64,
    ///Internal owner id
    pub owner_id: Option<u64>,
    ///Owner name
    pub owner_name: Option<String>,
    ///Castle name
    pub name: Option<String>,
    ///X position
    pub x: Option<u64>,
    ///Y position
    pub y: Option<u64>,
    ///World
    pub world: Option<World>,
}

///User data
#[derive(Debug, Hash, Eq, PartialEq, Clone, RustcEncodable)]
pub struct User{
    ///Internal id
    pub id: u64,
    ///Username
    pub username: Option<String>
}

impl fmt::Display for User{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", as_json(self))
    }
}

///Data manager
#[derive(RustcEncodable)]
pub struct DataMgr{
    ///List of castles
    pub castles: HashMap<u64, Castle>,
    pub users: HashMap<u64, User>,
}

impl DataMgr{
    ///Create new data manager
    pub fn new() -> Self{
        DataMgr{
            castles: HashMap::new(),
            users: HashMap::new(),
        }
    }
    
    ///Add the data of the specified castle
    pub fn add_castle(&mut self, castle: Castle) -> Castle{
        let mut castle = castle;
        let old_castle = self.castles.remove(&castle.id);
        match old_castle{
            Some(old_castle) => {
                castle.owner_id = castle.owner_id.or(old_castle.owner_id);
                castle.owner_name = castle.owner_name.or(old_castle.owner_name);
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
    
    ///Add the name of the specified user
    pub fn add_owner_name(&mut self, uid: u64, name: &str){
        self.users.insert(uid, User{
            id: uid,
            username: Some(name.to_owned())
        });
        
        for (_, castle) in self.castles.iter_mut(){
            if castle.owner_id == Some(uid){
                (*castle).owner_name = Some(name.to_string());
            }
        }
    }
}
