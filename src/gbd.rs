use slog::*;

use serde_json::value::Value;

use error::{Error, ErrorKind, ResultExt};
use data::Castle;
use data::World;
use data::DATAMGR;

macro_rules! try_field{
    ($data: expr, $field: expr) => {
        try!(::serde_json::ser::to_string(&$data.get($field)))
    };
}

/// Can parse castles
pub trait CastleParse {
    /// Parse castle
    fn parse(json: &Value, owner_id: u64, logger: Logger) -> Result<Self, Error> where Self: Sized;
}

impl CastleParse for Castle {
    fn parse(json: &Value, owner_id: u64, logger: Logger) -> Result<Castle, Error> {
        let json_array: &Vec<Value> = json.as_array().ok_or(ErrorKind::InvalidFormat("Castle json not an array".into()))?;

        if json_array.len() < 4 {
            // HACK to be able to run when there are special events
            error!(logger, "Parse error occured: json.len() < 4"; "json.len()" => json_array.len(), "owner_id" => owner_id, "json" => ::serde_json::ser::to_string(json).unwrap_or_else(|e|format!("{:?}",e)));
            return Err(ErrorKind::InvalidFormat(format!("Parse error: json.len() < 4, json.len == {}", json_array.len()).into()).into());
        }
        
        let world = json_array[0].as_u64().and_then(|world| Some(World::from_int(world)) ); // ain A M [] AP/VP [0] (world)

        let id = json_array[1].as_u64().ok_or(ErrorKind::InvalidFormat("field ain A M [] AP/VP [1] (id) not a positive number".into()))?; // ain A M [] AP/VP [1] (id)
        let x = json_array[2].as_u64(); // ain A M [] AP/VP [2] (x)
        let y = json_array[3].as_u64(); // ain A M [] AP/VP [3] (y)

        Ok(Castle {
            id: id,
            owner_id: Some(owner_id),
            name: None,
            x: x,
            y: y,
            world: world,
        })
    }
}

/// The alliance member data
#[derive(Debug, Clone)]
pub struct FieldAinM {
    /// Internal id
    pub oid: u64,
    /// Username
    pub n: String,
    /// Base castles
    pub ap: Vec<Castle>,
    /// Support castles
    pub vp: Vec<Castle>,
}

impl FieldAinM {
    /// Parse json data
    pub fn parse(json: &Value, logger: Logger) -> Result<Vec<FieldAinM>, Error> {
        let json: &Vec<Value> = json.as_array().ok_or(ErrorKind::InvalidFormat("gbd::ain::m not an array".into()))?;
        return json.into_iter().map(|row|{
            let row = row.as_object().unwrap();
            
            let oid = row.get("OID").unwrap().as_u64().unwrap(); // ain A M [] OID
            let n = row.get("N").unwrap().as_str().unwrap().to_string(); // ain A M [] N (username)
            
            DATAMGR.lock().unwrap().add_owner_name(oid, &n, true);
            
            let ap = row.get("AP").unwrap().as_array().unwrap(); // ain A M [] AP (base castles)
            let ap = ap.into_iter().map(|cell|Castle::parse(cell, oid, logger.clone())).collect::<Result<Vec<Castle>, Error>>()?;
            
            let vp = row.get("VP").unwrap().as_array().unwrap(); // ain A M [] VP (support castles)
            let vp = vp.into_iter().map(|cell|Castle::parse(cell, oid, logger.clone())).collect::<Result<Vec<Castle>, Error>>()?;
            
            Ok(FieldAinM{ oid: oid, n: n, ap: ap, vp: vp })
        }).collect::<Result<Vec<FieldAinM>, Error>>();
    }
}

/// Main data
#[derive(Debug, Clone)]
pub struct Gbd {
    /// User data
    pub gpi: String,
    /// Alliance member castles
    pub ain: Vec<FieldAinM>,
}

impl Gbd {
    /// Parse text returned from the server
    pub fn parse(data: String, logger: Logger) -> Result<Self, Error> {
        let data = data.trim_matches('%');
        let data: Value = try!(::serde_json::de::from_str(&data));
        if !data.is_object() {
            return Err(ErrorKind::InvalidFormat("gbd not an object".into()).into());
        }
        let json_data = data.clone();
        let mut data = data.as_object().unwrap().clone();
        data.remove("acl"); // remove chat from output
        let gpi = try_field!(data, "gpi");
        let ain = json_data.pointer("/ain/A/M").unwrap(); // ain A M
        let ain = FieldAinM::parse(ain, logger)?;
        let gbd = Gbd {
            gpi: gpi,
            ain: ain,
        };
        Ok(gbd)
    }
}
