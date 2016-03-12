extern crate rustc_serialize;
extern crate byte_stream_splitter;

pub mod error;
pub mod packet;
pub mod data;
pub mod gbd;
pub mod connection;

pub fn read_castles(data_mgr: &mut DataMgr, user: &str, data: gbd::Gbd){
    let pid = data.gcl.find("PID").unwrap().as_u64().unwrap();
    let owner_name = user;
    let dcl = data.gcl.find("C").unwrap().as_array().unwrap();
    for world in dcl{
        let world_name = World::from_int(world.find("KID").unwrap().as_u64().unwrap());
        let castles = world.find("AI").expect("No world AI").as_array().expect("AI not a array");
        for castle in castles{
            if castle.find("AI") == None{
                println!("{:?}", castle);
                continue;
            }
            let castle = castle.find("AI").expect("No castle AI").as_array().expect("AI not a array");
            let castle = Castle{
                id: castle[3].as_u64().unwrap(),
                owner_id: Some(pid),
                owner_name: Some(owner_name.to_string()),
                name: Some(castle[10].as_string().unwrap().to_owned()),
                x: castle[1].as_u64(),
                y: castle[2].as_u64(),
                world: Some(world_name)
            };
            data_mgr.add_castle(castle);
        }
    }

    for ain in data.ain{
        for castle in ain.ap{
            data_mgr.add_castle(castle);
        }
        for castle in ain.vp{
            data_mgr.add_castle(castle);
        }
    }
}