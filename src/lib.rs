#[macro_use]
extern crate slog;
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;
extern crate regex;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use slog::*;

pub use serde_json::ser::to_string as to_json;
use serde_json::from_str;
use serde_json::value::Value;

use data::DATAMGR;
use error::ResultExt;

/// Error
pub mod error;
/// Packet
pub mod packet;
/// Data
pub mod data;
/// Data reader
pub mod gbd;
/// Map reader
pub mod map;
/// Smartfoxserver client
pub mod smartfox;
/// Goodgame empire connection
pub mod connection;

mod byte_stream_splitter;

/// Read castles
pub fn read_castles(data: gbd::Gbd) {
    for ain in data.ain {
        for castle in ain.ap {
            DATAMGR.lock().unwrap().add_castle(castle);
        }
        for castle in ain.vp {
            DATAMGR.lock().unwrap().add_castle(castle);
        }
    }
}

pub fn read_names(data: String, logger: Logger) -> error::Result<()> {
    let data = data.trim_right_matches('%');
    let data: Value = from_str(data).chain_err(||"Cant parse json in gge::read_names")?;
    let data = data.as_object().ok_or(error::ErrorKind::InvalidFormat("Root not a object in gge::read_names".into()))?;
    let gcl = data.get("gcl").unwrap().as_object().unwrap(); // gcl
    let c = gcl.get("C").unwrap().as_array().unwrap(); // gcl C
    for world in c.iter() {
        let world = world.as_object().unwrap();
        let world_name = world.get("KID").unwrap().as_u64().unwrap(); // gcl C [] KID
        let world_name = data::World::from_int(world_name);
        let world = world.get("AI").unwrap().as_array().unwrap(); // gcl C [] AI
        for castle in world {
            let castle = castle.as_object().unwrap().get("AI").unwrap().as_array().unwrap(); // gcl C [] AI [] AI (castle)
            let castle_id = castle[3].as_u64().unwrap(); // gcl C [] AI [] AI [3] (id)
            let castle_name = if castle.len() == 18{
                castle[10].as_str().unwrap() // gcl C [] AI [] AI [10] (name)
            }else if castle.len() == 10{
                castle[6].as_str().unwrap()  // gcl C [] AI [] AI [6]  (name)
            }else{
                panic!("Invalid gcl C [] AI [] AI length {}", castle.len())
            };

            let castle = data::Castle {
                id: castle_id,
                owner_id: None,
                name: Some(castle_name.to_string()),
                x: None,
                y: None,
                world: Some(world_name),
            };
            debug!(logger, "processed castle";  "castle" => format!("{:?}", castle));
            DATAMGR.lock().unwrap().add_castle(castle);
        }
    }
    Ok(())
}
