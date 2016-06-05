use std::fmt;

use rustc_serialize::json::Json;

macro_rules! try_packet{
    ($data: expr, $reg: expr, $variant: expr) => {
        if $data.find($reg) == Some(0){
            return $variant($data.replace($reg, ""));
        }
    };
}

///A server returned packet of data
pub enum ServerPacket{
    Data(String),
    Kpi(String),
    Gam(String),
	Gbd(String),
    Gdi(String),
    None
}

impl ServerPacket{
    ///Create a packet from text
    pub fn new(data: String) -> Self{
        if data.is_empty(){
            return ServerPacket::None;
        }
        try_packet!(data, "%xt%kpi%1%0%", ServerPacket::Kpi);
        try_packet!(data, "%xt%gam%1%0%", ServerPacket::Gam);
		try_packet!(data, "%xt%gbd%1%0%", ServerPacket::Gbd);
		try_packet!(data, "%xt%gdi%1%0%", ServerPacket::Gdi);
        ServerPacket::Data(data)
    }
}

impl fmt::Debug for ServerPacket{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let (name, data): (&'static str, &str) = match *self{
            ServerPacket::Data(ref data) => ("data", data),
            ServerPacket::Kpi(ref data) => ("kpi", data),
            ServerPacket::Gam(ref data) => ("gam", data),
	        ServerPacket::Gbd(ref data) => ("gbd", data),
            ServerPacket::Gdi(ref data) => ("gdi", data),
            ServerPacket::None => ("none", "")
        };
        try!(write!(f, "{}: {}\n", name, data));
        if name == "gdi"{
            write!(f, "{:#?}", Json::from_str(data.trim_right_matches('%')).unwrap_or(Json::String(data.to_string())))
        }else{
            Ok(())
        }
    }
}

///A client send packet of data
#[derive(Debug)]
pub enum ClientPacket{
    //Ask for user castles
    Gdi(u64),
    None
}

impl ClientPacket{
    pub fn to_raw_data(&self) -> String{
        match *self{
            ClientPacket::None => String::new(),
            ClientPacket::Gdi(uid) => {
                format!("%xt%EmpireEx_11%gdi%1%{{\"PID\":{}}}%\0", uid)
            }
        }
    }
}
