use error::{ErrorKind, Result};
use packet::ServerPacket;

/// Data reader
pub mod gbd;
/// Map reader
pub mod map;

fn extract(packet: ServerPacket, con: &mut ::connection::Connection, data_mgr: &mut ::data::DataMgr) -> Result<()>{
    match packet{
        ServerPacket::Gbd(data) => gbd::extract(data, con, data_mgr),
        _ => Err(ErrorKind::InvalidFormat("invalid packet type".into()).into())
    }
}
