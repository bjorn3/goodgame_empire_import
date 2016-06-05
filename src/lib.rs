extern crate rustc_serialize;
extern crate byte_stream_splitter;
#[macro_use]
extern crate lazy_static;

use data::DATAMGR;

pub use rustc_serialize::json::as_json;

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
pub fn read_castles(user: &str, data: gbd::Gbd){
    let pid = data.gcl.find("PID").unwrap().as_u64().unwrap();
    let owner_name = user;
    let dcl = data.gcl.find("C").unwrap().as_array().unwrap();
    for world in dcl{
        let world_name = data::World::from_int(world.find("KID").unwrap().as_u64().unwrap());
        let castles = world.find("AI").expect("No world AI").as_array().expect("AI not a array");
        for castle in castles{
            if castle.find("AI") == None{
                println!("{:?}", castle);
                continue;
            }
            let castle = castle.find("AI").expect("No castle AI").as_array().expect("AI not a array");
            let castle = data::Castle{
                id: castle[3].as_u64().unwrap(),
                owner_id: Some(pid),
                owner_name: Some(owner_name.to_string()),
                name: Some(castle[10].as_string().unwrap().to_owned()),
                x: castle[1].as_u64(),
                y: castle[2].as_u64(),
                world: Some(world_name)
            };
            DATAMGR.lock().unwrap().add_castle(castle);
        }
    }

    for ain in data.ain{
        for castle in ain.ap{
            DATAMGR.lock().unwrap().add_castle(castle);
        }
        for castle in ain.vp{
            DATAMGR.lock().unwrap().add_castle(castle);
        }
    }
}
