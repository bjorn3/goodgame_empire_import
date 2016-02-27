use std::fmt;

use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ParserError;
use rustc_serialize::json::DecoderError;
use rustc_serialize::json::EncoderError;

macro_rules! try_field{
    ($data: expr, $field: expr) => {
        ::rustc_serialize::json::encode(&$data.get($field))
    };
}


#[derive(Debug)]
pub enum Error{
    InvalidFormat,
    ParserError(ParserError),
    DecoderError(DecoderError),
    EncoderError(EncoderError)
}

impl From<ParserError> for Error{
    fn from(err: ParserError) -> Error{
        Error::ParserError(err)
    }
}


impl From<DecoderError> for Error{
    fn from(err: DecoderError) -> Error{
        Error::DecoderError(err)
    }
}

impl From<EncoderError> for Error{
    fn from(err: EncoderError) -> Error{
        Error::EncoderError(err)
    }
}

#[derive(Debug, Clone)]
pub struct CastleData{
    name: Option<String>,
    x: u64,
    y: u64,
}

impl CastleData{
    pub fn parse(json: &Json, gcl: bool) -> Option<CastleData>{
        if !json.is_array(){
            return None;
        }
        let json: &Vec<Json> = json.as_array().unwrap();
        
        let (name, x, y) = if gcl{
            (
                Some(json[10].as_string().unwrap().to_owned()),
                json[1].as_u64().unwrap(),
                json[2].as_u64().unwrap()
            )
        }else{
            (
                None,
                json[2].as_u64().unwrap(),
                json[3].as_u64().unwrap()
            )
        };
        
        Some(CastleData{ name: name, x: x, y: y })
    }
}

impl fmt::Display for CastleData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", "{");
        if let Some(ref name) = self.name{
            write!(f, " name: {},", name);
        }
        write!(f, " x: {}, y: {}", self.x, self.y);
        write!(f, " {}", "}")
    }
}

#[derive(Debug, Clone)]
pub struct FieldAinM{
    OID: u64,
    N: String,
    AP: Vec<CastleData>,
    VP: Vec<CastleData>,
}

impl FieldAinM{
    pub fn parse(json: &Json) -> Option<Vec<FieldAinM>>{
        if !json.is_array(){
            return None;
        }
        let json: &Vec<Json> = json.as_array().unwrap();
        return Some(json.into_iter().map(|row|{
            let row = row.as_object().unwrap();
            
            let oid = row.get("OID").unwrap().as_u64().unwrap();
            let n = row.get("N").unwrap().as_string().unwrap().to_owned();
            
            let ap = row.get("AP").unwrap().as_array().unwrap();
            let ap = ap.into_iter().map(|cell|CastleData::parse(cell, false).unwrap()).collect::<Vec<CastleData>>();
            
            let vp = row.get("VP").unwrap().as_array().unwrap();
            let vp = vp.into_iter().map(|cell|CastleData::parse(cell, false).unwrap()).collect::<Vec<CastleData>>();
            
            FieldAinM{ OID: oid, N: n, AP: ap, VP: vp }
        }).collect::<Vec<FieldAinM>>());
    }
}

impl fmt::Display for FieldAinM{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} ", "{");
        write!(f, "oid: {}, ", self.OID);
        write!(f, "n: \"{}\", ", self.N);
        write!(f, "ap: [");
        for row in &self.AP{
            write!(f, "{},\n", row);
        }
        write!(f, "],\n");
        write!(f, "vp: [");
        for row in &self.VP{
            write!(f, "{},\n", row);
        }
        write!(f, "]\n")
    }
}


#[derive(Debug, Clone)]
pub struct Gbd{
    gpi: String,
    dcl: Json, //own castles
    ain: Vec<FieldAinM>, //alliance player castles
}

impl Gbd{
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
        let dcl = data.get("dcl");
        let ain = json_data.find_path(&["ain", "A", "M"]).unwrap(); // ain A M
        let ain = FieldAinM::parse(ain).unwrap();
        let gbd = Gbd{gpi: gpi.unwrap(), dcl: dcl.unwrap().clone(), ain: ain};
        Ok(gbd)
    }
}

impl fmt::Display for Gbd{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} ", "{");
        write!(f, "gpi: {}, ", self.gpi);
        write!(f, "dcl: {}, ", self.dcl);
        write!(f, "ain: [");
        for row in &self.ain{
            write!(f, "{}, \n", row);
        }
        write!(f, "]");
        write!(f, "{}", "}")
    }
}