use std::fmt;

use rustc_serialize::json::Json;

use error::Error;
use data::Castle;
use data::World;
use data::DATAMGR;

macro_rules! try_field{
    ($data: expr, $field: expr) => {
        ::rustc_serialize::json::encode(&$data.get($field))
    };
}

///Can parse castles
pub trait CastleParse{
    ///Parse castle
    fn parse(json: &Json, gcl: bool, owner_id: u64, world: Option<World>) -> Result<Self, Error> where Self: Sized;
}

impl CastleParse for Castle{
    fn parse(json: &Json, gcl: bool, owner_id: u64, world: Option<World>) -> Result<Castle, Error>{
        if !json.is_array(){
            return Err(Error::InvalidFormat);
        }
        let json: &Vec<Json> = json.as_array().unwrap();
        
        if json.len() < 4{
            // HACK to be able to run when there are special events
            println!("Parse error occured: json.len() < 4");
            return Err(Error::InvalidFormat);
        }
        
        let world = if !gcl && world == None{
            json[0].as_u64().and_then(|world| Some(World::from_int(world)) )
        }else{
            world
        };
        
        let (id, name, x, y) = if gcl{
            (
                json[3].as_u64().unwrap(),
                Some(json[10].as_string().unwrap().to_owned()),
                json[1].as_u64(),
                json[2].as_u64()
            )
        }else{
            (
                json[1].as_u64().unwrap(),
                None,
                json[2].as_u64(),
                json[3].as_u64()
            )
        };
        
        Ok(Castle{ id: id, owner_id: Some(owner_id), name: name, x: x, y: y, world: world })
    }
}

///The alliance member data
#[derive(Debug, Clone)]
pub struct FieldAinM{
    ///Internal id
    pub oid: u64,
    ///Username
    pub n: String,
    ///Base castles
    pub ap: Vec<Castle>,
    ///Support castles
    pub vp: Vec<Castle>,
}

impl FieldAinM{
    ///Parse json data
    pub fn parse(json: &Json) -> Result<Vec<FieldAinM>, Error>{
        if !json.is_array(){
            return Err(Error::InvalidFormat);
        }
        let json: &Vec<Json> = json.as_array().unwrap();
        return Ok(json.into_iter().map(|row|{
            let row = row.as_object().unwrap();
            
            let oid = row.get("OID").unwrap().as_u64().unwrap();
            let n = row.get("N").unwrap().as_string().unwrap().to_owned();
            
            DATAMGR.lock().unwrap().add_owner_name(oid, &n);
            
            let ap = row.get("AP").unwrap().as_array().unwrap();
            let ap = ap.into_iter().map(|cell|Castle::parse(cell, false, oid, None)).filter_map(|castle|castle.map_err(|err|{
                println!("{}", err);
                err
            }).ok()).collect::<Vec<Castle>>();
            
            let vp = row.get("VP").unwrap().as_array().unwrap();
            let vp = vp.into_iter().map(|cell|Castle::parse(cell, false, oid, None)).filter_map(|castle|castle.map_err(|err|{
                println!("{}", err);
                err
            }).ok()).collect::<Vec<Castle>>();
            
            FieldAinM{ oid: oid, n: n, ap: ap, vp: vp }
        }).collect::<Vec<FieldAinM>>());
    }
}

impl fmt::Display for FieldAinM{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        try!(write!(f, "{} ", "{"));
        try!(write!(f, "oid: {}, ", self.oid));
        try!(write!(f, "n: \"{}\", ", self.n));
        try!(write!(f, "ap: ["));
        for row in &self.ap{
            try!(write!(f, "{:?},\n", row));
        }
        try!(write!(f, "],\n"));
        try!(write!(f, "vp: ["));
        for row in &self.vp{
            try!(write!(f, "{:?},\n", row));
        }
        write!(f, "]\n")
    }
}

///Main data
#[derive(Debug, Clone)]
pub struct Gbd{
    ///User data
    pub gpi: String,
    ///Own castles
    pub gcl: Json,
    ///Alliance member castles
    pub ain: Vec<FieldAinM>
}

impl Gbd{
    ///Parse text returned from the server
    pub fn parse(data: String) -> Result<Self, Error>{
        let data = data.trim_matches('%');
        let data = try!(Json::from_str(&data));
        if !data.is_object(){
            return Err(Error::InvalidFormat);
        }
        let json_data = data.clone();
        let mut data = data.as_object().unwrap().clone();
        data.remove("acl"); // remove chat from output
        let gpi = try_field!(data, "gpi");
        let gcl = data.get("gcl");
        let ain = json_data.find_path(&["ain", "A", "M"]).unwrap(); // ain A M
        let ain = FieldAinM::parse(ain).unwrap();
        let gbd = Gbd{gpi: gpi.unwrap(), gcl: gcl.unwrap().clone(), ain: ain};
        Ok(gbd)
    }
}

impl fmt::Display for Gbd{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        try!(write!(f, "{} ", "{"));
        try!(write!(f, "gpi: {}, ", self.gpi));
        try!(write!(f, "gcl: {}, ", self.gcl));
        try!(write!(f, "ain: ["));
        for row in &self.ain{
            try!(write!(f, "{}, \n", row));
        }
        try!(write!(f, "]"));
        write!(f, "{}", "}")
    }
}
