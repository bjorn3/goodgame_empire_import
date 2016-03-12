use std::fmt;
use std::collections::HashMap;

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
    pub owner_name: Option<String>,
    pub name: Option<String>,
    pub x: Option<u64>,
    pub y: Option<u64>,
    pub world: Option<World>,
}

impl fmt::Display for Castle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let owner = self.owner_name.clone().unwrap_or(format!("{}", self.owner_id.unwrap_or(0)));
        let name = self.name.clone().unwrap_or(format!("{}_{}", owner, self.id));
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
        try!(write!(f, "\"owner\": \"{}\",", owner));
        try!(write!(f, "\"X\": {},", x));
        try!(write!(f, "\"Y\": {},", y));
        try!(write!(f, "\"wereld\": \"{}\" ", world));
        write!(f, "{}", "}")
    }
}

pub struct DataMgr{
    pub castles: HashMap<u64, Castle>
}

impl DataMgr{
    pub fn new() -> Self{
        DataMgr{ castles: HashMap::new() }
    }

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
    
    pub fn add_owner_name(&mut self, uid: u64, name: &str){
        for (_, castle) in self.castles.iter_mut(){
            if castle.owner_id == Some(uid){
                (*castle).owner_name = Some(name.to_string());
            }
        }
    }
}
