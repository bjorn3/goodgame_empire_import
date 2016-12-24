use std::fmt;

use error::{Result, ChainErr};

/// A server returned packet of data.
#[derive(Clone, Eq, PartialEq)]
pub enum ServerPacket{
    /// Unrecognized data
    Data(String, String),

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

    /// Server info
    Nfo(String),

    /// Get player info
    CoreGpi(String),
    
    /// Map info
    Gaa(String),

    /// Empty packet.
    None
}

impl ServerPacket{
    /// Create a packet from text.
    /// Returns ServerPacket::Data when it does not recognize the data.
    pub fn new(original_data: String) -> Result<Self>{
        use regex::Regex;

        if original_data.is_empty(){
            return Ok(ServerPacket::None);
        }

        let regex = Regex::new(r"^%xt%([:word:]+)%1%0%(.*)$").expect("Invalid packet regex");

        Ok(if let Some(captures) = regex.captures(&original_data){
            let name = captures.at(1).unwrap();
            let data = captures.at(2).unwrap();
            assert!(captures.at(3).is_none());
            match &*name{
                "kpi"      => ServerPacket::Kpi    (data.to_string()),
                "gam"      => ServerPacket::Gam    (data.to_string()),
                "gbd"      => ServerPacket::Gbd    (data.to_string()),
                "gdi"      => ServerPacket::Gdi    (data.to_string()),
                "irc"      => ServerPacket::Irc    (data.to_string()),
                "sei"      => ServerPacket::Sei    (data.to_string()),
                "nfo"      => ServerPacket::Nfo    (data.to_string()),
                "core_gpi" => ServerPacket::CoreGpi(data.to_string()),
                "gaa"      => ServerPacket::Gaa    (data.to_string()),
                _          => ServerPacket::Data   (name.to_string(), data.to_string())
            }
        }else{
            ServerPacket::Data("".to_string(), original_data.to_string())
        })
    }
}

impl fmt::Debug for ServerPacket{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let (description, name, _data): (&'static str, &str, &str) = match *self{
            ServerPacket::Data   (ref name, ref data) => ("unknown type"  , name      , data),
            ServerPacket::Kpi    (ref data)           => (""              , "kpi"     , data),
            ServerPacket::Gam    (ref data)           => (""              , "gam"     , data),
	        ServerPacket::Gbd    (ref data)           => (""              , "gbd"     , data),
            ServerPacket::Gdi    (ref data)           => (""              , "gdi"     , data),
            ServerPacket::Sei    (ref data)           => (""              , "sei"     , data),
            ServerPacket::Irc    (ref data)           => (""              , "irc"     , data),
            ServerPacket::Nfo    (ref data)           => ("serverinfo"    , "nfo"     , data),
            ServerPacket::CoreGpi(ref data)           => ("getplayerinfo" , "core_gpi", data),
            ServerPacket::Gaa    (ref data)           => ("mapinfo"       , "gaa"     , data),
            ServerPacket::None                        => ("none"          , ""        , ""  )
        };
        write!(f, "{:13} ({:9}) ( {} ... )", description, name, _data.chars().zip(0..64).map(|c|c.0).collect::<String>())
    }
}

/// A client send packet of data
#[derive(Debug)]
pub enum ClientPacket{
    /// Ask for user castles
    Gdi(u64),

    /// Ask for world map
    Gaa(String),

    None
}

impl ClientPacket{
    pub fn to_raw_data(&self) -> String{
        match *self{
            ClientPacket::None => String::new(),
            ClientPacket::Gdi(uid) => {
                format!("%xt%EmpireEx_11%gdi%1%{{\"PID\":{}}}%", uid)
            },
            ClientPacket::Gaa(ref data) => {
                format!("%xt%EmpireEx_11%gaa%1%{}%", data)
            }
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn parse_server_packet(){
        assert_eq!(ServerPacket::new("%xt%lli%1%0%".to_string()), ServerPacket::Data("lli".to_string(), "".to_string()));
        assert_eq!(ServerPacket::new("%xt%gbd%1%0%{\\\"gpi\\\":{\\\"UID\\\"".to_string()), ServerPacket::Gbd("{\\\"gpi\\\":{\\\"UID\\\"".to_string()));
    }

    #[test]
    fn parse_invalid_server_packet(){
        assert_eq!(ServerPacket::new("efroniveioej54549945wj9awjoawoiwa2322131298489439834#@*($&*($(*(*$@))))".to_string()), ServerPacket::Data("".to_string(), "efroniveioej54549945wj9awjoawoiwa2322131298489439834#@*($&*($(*(*$@))))".to_string()))
    }

    #[test]
    fn display_server_packet(){
        assert_eq!(format!("{:?}", ServerPacket::Irc("dsimoreoib".to_string())), "              (irc      ) ( dsimoreoib ... )".to_string());
    }

    #[test]
    fn serialize_client_packet(){
        assert_eq!(ClientPacket::Gdi(10).to_raw_data(), "%xt%EmpireEx_11%gdi%1%{\"PID\":10}%".to_string());
        assert_eq!(ClientPacket::Gaa("agreverebcd".to_string()).to_raw_data(), "%xt%EmpireEx_11%gaa%1%agreverebcd%".to_string());
    }
}
