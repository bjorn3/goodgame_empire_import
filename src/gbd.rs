use serde_json::value::{Value, from_value};

use error::{ErrorKind, Result, ResultExt};
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
    fn parse(json: Value, owner_id: u64) -> Result<Self> where Self: Sized;
}

impl CastleParse for Castle {
    fn parse(json: Value, owner_id: u64) -> Result<Castle> {
        #[derive(Deserialize)]
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        /// ain A M [] AP/VP
        struct _FieldAinM__APVP(World, u64, u64, u64, u64);

        let obj: _FieldAinM__APVP = from_value(json.clone())
            .chain_err(|| "Cant deserialize gdb ain A M [] AP/VP")?;

        Ok(Castle {
            id: obj.1, // ain A M [] AP/VP [1] (id)
            owner_id: Some(owner_id),
            name: None,
            x: Some(obj.2), // ain A M [] AP/VP [2] (x)
            y: Some(obj.3), // ain A M [] AP/VP [3] (y)
            world: Some(obj.0), // ain A M [] AP/VP [0] (world)
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
    pub fn parse(json: &Value) -> Result<Vec<FieldAinM>> {
        #[derive(Deserialize)]
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        /// ain A M []
        struct _FieldAinM__ {
            OID: u64,
            N: String,
            AP: Vec<Value>,
            VP: Vec<Value>,
        }

        let json: &Vec<Value> = json.as_array()
            .ok_or(ErrorKind::InvalidFormat("gbd ain A M not an array".into()))?;
        return json.into_iter()
            .map(|row| {
                let obj: _FieldAinM__ = from_value(row.clone())
                    .chain_err(|| "Cant deserialize gdb ain A M []")?;

                let oid = obj.OID; // ain A M [] OID
                let n = obj.N; // ain A M [] N (username)

                DATAMGR.lock().unwrap().add_owner_name(oid, &n, true);

                let ap = obj.AP
                //           ^^ ain A M [] AP (base castles)
                    .into_iter()
                    .map(|cell| Castle::parse(cell, oid))
                    .collect::<Result<Vec<Castle>>>()?;

                let vp = obj.VP
                //           ^^ ain A M [] VP (support castles)
                    .into_iter()
                    .map(|cell| Castle::parse(cell, oid))
                    .collect::<Result<Vec<Castle>>>()?;

                Ok(FieldAinM {
                    oid: oid,
                    n: n,
                    ap: ap,
                    vp: vp,
                })
            })
            .collect::<Result<Vec<FieldAinM>>>();
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
    pub fn parse(data: String) -> Result<Self> {
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
        let ain = FieldAinM::parse(ain)?;
        let gbd = Gbd {
            gpi: gpi,
            ain: ain,
        };
        Ok(gbd)
    }
}
