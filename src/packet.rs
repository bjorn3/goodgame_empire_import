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

    /// Unknown kind of data
    Sei(String),

    /// Some kind of keepalive data.
    Irc(String),

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
        let data = data.trim_left_matches("%xt%").to_string();

        try_packet!(data, "kpi%1%0%", ServerPacket::Kpi);
        try_packet!(data, "gam%1%0%", ServerPacket::Gam);
		try_packet!(data, "gbd%1%0%", ServerPacket::Gbd);
		try_packet!(data, "gdi%1%0%", ServerPacket::Gdi);
        try_packet!(data, "irc%1%0%", ServerPacket::Irc);
        try_packet!(data, "sei%1%0%", ServerPacket::Sei);
        ServerPacket::Data(data)
    }
}

impl fmt::Debug for ServerPacket{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let (name, _data): (&'static str, &str) = match *self{
            ServerPacket::Data(ref data) => ("data", data),
            ServerPacket::Kpi(ref data) => ("kpi ", data),
            ServerPacket::Gam(ref data) => ("gam ", data),
	        ServerPacket::Gbd(ref data) => ("gbd ", data),
            ServerPacket::Gdi(ref data) => ("gdi ", data),
            ServerPacket::Sei(ref data) => ("sei ", data),
            ServerPacket::Irc(ref data) => ("irc ", data),
            ServerPacket::None => ("none", "")
        };
        write!(f, "{} ( {} ... )", name, _data.chars().zip(0..64).map(|c|c.0).collect::<String>())
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
                format!("%xt%EmpireEx_11%gdi%1%{{\"PID\":{}}}%", uid)
            }
        }
    }
}
