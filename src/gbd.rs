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
pub struct Gbd{
    gpi: String,
    upi: String,
    dcl: Json,
    rest: Json,
    rest_json: String
}

impl Gbd{
    pub fn parse(data: String) -> Result<Self, Error>{
        let data = data.trim_matches('%');
        let data = try!(Json::from_str(&data));
        if !data.is_object(){
            return Err(Error::InvalidFormat);
        }
        let mut data = data.as_object().unwrap().clone();
        data.remove("acl"); // remove chat from output
        let gpi = try_field!(data, "gpi");
        let upi = try_field!(data, "upi");
        //let dcl = try_field!(data_obj, "dcl");
        let dcl = data.get("dcl");
        let gbd = Gbd{gpi: gpi.unwrap(), upi: upi.unwrap(), dcl: dcl.unwrap().clone(), rest: Json::Object(data.clone()), rest_json: json::encode(&Json::Object(data.clone())).unwrap()};
        Ok(gbd)
    }
}