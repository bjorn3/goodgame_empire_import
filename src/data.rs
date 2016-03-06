use std::fmt;
use std::sync::Mutex;
use std::collections::HashMap;
use std::collections::hash_map::Values;

lazy_static!{
    pub static ref CASTLES: Mutex<CastleMgr> = Mutex::new(CastleMgr::new());
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum World{
    Fire,
    Sand,
    Grass,
    Ice,
    SpecialEvent
}

impl World{
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

impl fmt::Display for World{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let data = match *self{
            World::Grass => "gras".to_string(),
            World::Sand => "zand".to_string(),
            World::Ice => "ijs".to_string(),
            World::Fire => "vuur".to_string(),
            _ => "Unknown world".to_string()
        };
        write!(f, "{}", data)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Castle{
    pub id: u64,
    pub owner_id: Option<u64>,
    pub name: Option<String>,
    pub x: Option<u64>,
    pub y: Option<u64>,
    pub world: Option<World>,
}

impl fmt::Display for Castle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let name = self.name.clone().unwrap_or(format!("{}_{}", self.owner_id.unwrap_or(0), self.id));
        let x = match self.x{
            Some(x) => x,
            None => return Err(fmt::Error)
        };
        let y = match self.y{
            Some(y) => y,
            None => return Err(fmt::Error)
        };
        let world = self.world.unwrap_or(World::SpecialEvent);
        try!(write!(f, "{}", "{"));
        try!(write!(f, "\"name\": \"{}\",", name));
        try!(write!(f, "\"X\": {},", x));
        try!(write!(f, "\"Y\": {},", y));
        try!(write!(f, "\"wereld\": \"{}\" ", world));
        write!(f, "{}", "}")
    }
}

// json format: {"name":"Ajaciedy","X":373,"Y":376,"wereld":"gras"}

pub struct CastleMgr{
    inner: HashMap<u64, Castle>
}

impl CastleMgr{
    pub fn new() -> Self{
        CastleMgr{ inner: HashMap::new() }
    }

    pub fn add(&mut self, castle: Castle) -> Castle{
        let mut castle = castle;
        let old_castle = self.inner.remove(&castle.id);
        match old_castle{
            Some(old_castle) => {
                if castle.owner_id == None{
                    castle.owner_id = old_castle.owner_id;
                }
                if castle.name == None{
                    castle.name = old_castle.name;
                }
                if castle.x == None{
                    castle.x = old_castle.x;
                }
                if castle.y == None{
                    castle.y = old_castle.y;
                }
                if castle.world == None{
                    castle.world = old_castle.world;
                }
            },
            None => {}
        }
        self.inner.insert(castle.id, castle.clone());
        return castle;
    }

    pub fn iter(&self) -> Values<u64, Castle>{
        self.inner.values()
    }
}
