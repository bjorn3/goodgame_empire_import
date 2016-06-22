use std::fmt;

macro_rules! try_packet{
    ($data: expr, $reg: expr, $variant: expr) => {
        if $data.find($reg) == Some(0){
            return $variant($data.replace($reg, ""));
        }
    };
}

/// A server returned packet of data.
pub enum ServerPacket{
    /// Unrecognized data
    Data(String),

    /// Kpi packet
    Kpi(String),

    /// Gam packet
    Gam(String),

    /// Main data source.
    /// Send by the server when you login.
	Gbd(String),

    /// Castle information of a specific user.
    Gdi(String),

    /// Empty packet.
    None
}

impl ServerPacket{
    /// Create a packet from text.
    /// Returns ServerPacket::Data when it does not recognize the data.
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
        let (name, _data): (&'static str, &str) = match *self{
            ServerPacket::Data(ref data) => ("data", data),
            ServerPacket::Kpi(ref data) => ("kpi", data),
            ServerPacket::Gam(ref data) => ("gam", data),
	        ServerPacket::Gbd(ref data) => ("gbd", data),
            ServerPacket::Gdi(ref data) => ("gdi", data),
            ServerPacket::None => ("none", "")
        };
        //write!(f, "{} ( {} )\n\n", name, _data)
        write!(f, "{}", name)
    }
}

///A client send packet of data
#[derive(Debug)]
pub enum ClientPacket{
    ///Ask for user castles
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
