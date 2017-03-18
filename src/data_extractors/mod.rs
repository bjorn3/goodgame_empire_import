use error::{ErrorKind, Result};
use packet::ServerPacket;

/// Data reader
pub mod gbd;
/// Map reader
pub mod map;

pub fn extract(packet: ServerPacket, con: &mut ::connection::Connection, data_mgr: &mut ::data::DataMgr) -> Result<()>{
    match packet{
        ServerPacket::Gbd(data) => gbd::extract(data, con, data_mgr),
        ServerPacket::Gaa(data) => map::extract(::serde_json::de::from_str(&data).unwrap(), con, data_mgr),
        _ => Err(ErrorKind::InvalidFormat("invalid packet type".into()).into())
    }
}
