use std::fmt;

use serde_json::{Value, from_str, to_string};

use error::{Result, ResultExt};

/// A server returned packet of data.
#[derive(Clone, PartialEq)]
pub enum ServerPacket {
    /// Unrecognized data
    Data(String, String),

    /// Kpi packet
    Kpi(String),

    /// Gam packet
    Gam(String),

    /// Main data source.
    /// Send by the server when you login.
	Gbd(Value),

    /// Castle information of a specific user.
    Gdi(String),

    /// Unknown kind of data
    Sei(String),

    /// Some kind of keepalive data.
    Irc(String),

    /// Server info
    Nfo(String),

    /// Get player info
    CoreGpi(String),
    
    /// Map info
    Gaa(String),

    /// Empty packet.
    None,
}

impl ServerPacket {
    /// Create a packet from text.
    /// Returns ServerPacket::Data when it does not recognize the data.
    pub fn new(original_data: String) -> Result<Self> {
        use regex::Regex;
        
        lazy_static! {
            static ref PACKET_REGEX: Regex = Regex::new(r"^%xt%([[:word:]]+)%1%0%(.*)$").expect("Invalid packet regex");
        }

        if original_data.is_empty() {
            return Ok(ServerPacket::None);
        }

        Ok(if let Some(captures) = PACKET_REGEX.captures(&original_data) {
            let name = captures.get(1).unwrap().as_str();
            let data = captures.get(2).unwrap().as_str().trim_right_matches('%');
            assert!(captures.get(3).is_none());
            println!("{:?}", data);
            match &*name {
                "kpi"      => ServerPacket::Kpi    (data.to_string()),
                "gam"      => ServerPacket::Gam    (data.to_string()),
                "gbd"      => ServerPacket::Gbd    (from_str(data).chain_err(|| "Failed to parse gbd packet")?),
                "gdi"      => ServerPacket::Gdi    (data.to_string()),
                "irc"      => ServerPacket::Irc    (data.to_string()),
                "sei"      => ServerPacket::Sei    (data.to_string()),
                "nfo"      => ServerPacket::Nfo    (data.to_string()),
                "core_gpi" => ServerPacket::CoreGpi(data.to_string()),
                "gaa"      => ServerPacket::Gaa    (data.to_string()),
                _          => ServerPacket::Data   (name.to_string(), data.to_string())
            }
        } else {
            ServerPacket::Data("".to_string(), original_data.to_string())
        })
    }
}

impl fmt::Debug for ServerPacket{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let (description, name, data): (&'static str, String, String) = match self.clone(){
            ServerPacket::Data   (name, data) => ("unknown type"  , name      , data),
            ServerPacket::Kpi    (data)       => (""              , "kpi".to_string()      , data),
            ServerPacket::Gam    (data)       => (""              , "gam".to_string()      , data),
	        ServerPacket::Gbd    (data)       => (""              , "gbd".to_string()      , to_string(&data).unwrap()),
            ServerPacket::Gdi    (data)       => (""              , "gdi".to_string()      , data),
            ServerPacket::Sei    (data)       => (""              , "sei".to_string()      , data),
            ServerPacket::Irc    (data)       => (""              , "irc".to_string()      , data),
            ServerPacket::Nfo    (data)       => ("serverinfo"    , "nfo".to_string()      , data),
            ServerPacket::CoreGpi(data)       => ("getplayerinfo" , "core_gpi".to_string() , data),
            ServerPacket::Gaa    (data)       => ("mapinfo"       , "gaa".to_string()      , data),
            ServerPacket::None                => ("none"          , "".to_string()         , "".to_string()),
        };
        write!(f,
               "{:13} ({:9}) ( {} ... )",
               description,
               name,
               data.chars().take(64).collect::<String>())
    }
}

/// A client send packet of data
#[derive(Debug)]
pub enum ClientPacket {
    /// Ask for user castles
    Gdi(u64),

    /// Ask for world map
    Gaa(String),

    None,
}

impl ClientPacket {
    pub fn to_raw_data(&self) -> String {
        match *self {
            ClientPacket::None => String::new(),
            ClientPacket::Gdi(uid) => format!("%xt%EmpireEx_11%gdi%1%{{\"PID\":{}}}%", uid),
            ClientPacket::Gaa(ref data) => format!("%xt%EmpireEx_11%gaa%1%{}%", data),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_server_packet() {
        assert_eq!(ServerPacket::new("%xt%lli%1%0%".to_string()).unwrap(),
                   ServerPacket::Data("lli".to_string(), "".to_string()));
        assert_eq!(ServerPacket::new(r#"%xt%gbd%1%0%{"gpi":{"UID":0}}%"#.to_string()).unwrap(),
                   ServerPacket::Gbd(from_str(r#"{"gpi":{"UID":0}}"#).unwrap()));
    }

    #[test]
    fn parse_invalid_server_packet() {
        assert_eq!(ServerPacket::new("efroniveioej54549945wj9awjoawoiwa2322131298489439834#@*($&*($(*(*$@))))".to_string()).unwrap(), ServerPacket::Data("".to_string(), "efroniveioej54549945wj9awjoawoiwa2322131298489439834#@*($&*($(*(*$@))))".to_string()))
    }

    #[test]
    fn display_server_packet() {
        assert_eq!(format!("{:?}", ServerPacket::Irc("dsimoreoib".to_string())),
                   "              (irc      ) ( dsimoreoib ... )".to_string());
    }

    #[test]
    fn serialize_client_packet() {
        assert_eq!(ClientPacket::Gdi(10).to_raw_data(),
                   "%xt%EmpireEx_11%gdi%1%{\"PID\":10}%".to_string());
        assert_eq!(ClientPacket::Gaa("agreverebcd".to_string()).to_raw_data(),
                   "%xt%EmpireEx_11%gaa%1%agreverebcd%".to_string());
    }
}
