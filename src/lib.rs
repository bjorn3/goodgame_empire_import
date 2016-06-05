extern crate rustc_serialize;
extern crate byte_stream_splitter;
#[macro_use]
extern crate lazy_static;

use data::DATAMGR;

pub use rustc_serialize::json::as_json;
use rustc_serialize::json::Json;

///Error
pub mod error;
///Packet
pub mod packet;
///Data
pub mod data;
///Data reader
pub mod gbd;
///Goodgame empire connection
pub mod connection;

///Read castles
pub fn read_castles(data: gbd::Gbd){
    for ain in data.ain{
        for castle in ain.ap{
            DATAMGR.lock().unwrap().add_castle(castle);
        }
        for castle in ain.vp{
            DATAMGR.lock().unwrap().add_castle(castle);
        }
    }
}

pub fn read_names(data: String){
    let data = data.trim_right_matches('%');
    let data = Json::from_str(data).unwrap();
    let data = data.as_object().unwrap();
    let gcl = data.get("gcl").unwrap().as_object().unwrap();
    println!("gcl: {:?}\n", gcl);
    let c = gcl.get("C").unwrap().as_array().unwrap();
    println!("C: {:?}", c);
    for world in c.iter(){
        let world = world.as_object().unwrap();
        let world_name = world.get("KID").unwrap().as_u64().unwrap();
        let world_name = data::World::from_int(world_name);
        println!("world: {:?}", world_name);
        let world = world.get("AI").unwrap().as_array().unwrap();
        for castle in world{
            let castle = castle.as_object().unwrap().get("AI").unwrap().as_array().unwrap();
            println!("castle: {:?}", castle);
            let castle_id = castle[3].as_u64().unwrap();
            let castle_name = castle[10].as_string().unwrap();
            let castle = data::Castle{
                id: castle_id,
                owner_id: None,
                name: Some(castle_name.to_string()),
                x: None,
                y: None,
                world: None
            };
            println!("castle: {:?}\n", castle);
            DATAMGR.lock().unwrap().add_castle(castle);
        }
    }
}