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
    fn parse(json: &Json, owner_id: u64) -> Result<Self, Error> where Self: Sized;
}

impl CastleParse for Castle{
    fn parse(json: &Json, owner_id: u64) -> Result<Castle, Error>{
        if !json.is_array(){
            return Err(Error::InvalidFormat);
        }
        let json: &Vec<Json> = json.as_array().unwrap();
        
        if json.len() < 4{
            // HACK to be able to run when there are special events
            println!("Parse error occured: json.len() < 4");
            return Err(Error::InvalidFormat);
        }
        
        let world = json[0].as_u64().and_then(|world| Some(World::from_int(world)) );
        
        let id = json[1].as_u64().unwrap();
        let x = json[2].as_u64();
        let y = json[3].as_u64();
        
        Ok(Castle{ id: id, owner_id: Some(owner_id), name: None, x: x, y: y, world: world })
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
            let ap = ap.into_iter().map(|cell|Castle::parse(cell, oid)).filter_map(|castle|castle.map_err(|err|{
                println!("{}", err);
                err
            }).ok()).collect::<Vec<Castle>>();
            
            let vp = row.get("VP").unwrap().as_array().unwrap();
            let vp = vp.into_iter().map(|cell|Castle::parse(cell, oid)).filter_map(|castle|castle.map_err(|err|{
                println!("{}", err);
                err
            }).ok()).collect::<Vec<Castle>>();
            
            FieldAinM{ oid: oid, n: n, ap: ap, vp: vp }
        }).collect::<Vec<FieldAinM>>());
    }
}

///Main data
#[derive(Debug, Clone)]
pub struct Gbd{
    ///User data
    pub gpi: String,
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
        let ain = json_data.find_path(&["ain", "A", "M"]).unwrap(); // ain A M
        let ain = FieldAinM::parse(ain).unwrap();
        let gbd = Gbd{gpi: gpi.unwrap(), ain: ain};
        Ok(gbd)
    }
}