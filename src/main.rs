extern crate goodgame_empire_import as gge;
use std::env;
use std::io;
use std::io::Write;

use gge::packet::Packet;
use gge::connection::Connection;
use gge::data::DataMgr;

fn main() {
    let mut data_mgr = DataMgr::new();

    let mut con = Connection::new();
    
    io::stderr().write(b"Please login\n").unwrap();
    let un: String = env_or_ask("GGE_USERNAME", "Username: ");
    let pw: String = env_or_ask("GGE_PASSWORD", "Password: ");
    con.login(&un, &pw);
    
    let mut found_gbd_packet = false;
    
    for pkt in con.read_packets(){
        match pkt{
            Packet::Gbd(ref data) => {
                found_gbd_packet = true;
                let data = &*data;
                let data = gge::gbd::Gbd::parse(data.to_owned()).unwrap();
                gge::read_castles(&mut data_mgr, &un, data.clone());
            },
            _ => continue
        };
    }
    
    for castle in data_mgr.castles.values().take(20){
        println!("{:?}", castle);
    }
    
    let file_name = env_or_ask("GGE_FILENAME", "Filename: ");
    
    let mut f = std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();
    
    write!(f, "[").unwrap();
    
    let mut iter = data_mgr.castles.values().peekable();
    loop{
        if let Some(castle) = iter.next(){
            let has_next = match iter.peek(){
                Some(_) => true,
                None => false
            };
            write!(f, "{}{}\n", castle, if has_next{
                ","
            }else{
                ""
            }).unwrap();
        }else{
            break;
        }
    }
    
    write!(f, "]").unwrap();
    
    if !found_gbd_packet{
        io::stderr().write(b"Login failed\n").unwrap();
    }
}

/*fn read_castles(data_mgr: &mut DataMgr, user: &str, data: gge::gbd::Gbd){
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
}*/

fn env_or_ask(env_name: &str, question: &str) -> String{
    env::var(env_name).and_then(|data|{
        if data.len() < 2{
            Err(env::VarError::NotPresent)
        }else{
            Ok(data)
        }
    }).or_else(|_| -> Result<String,io::Error>{
        let mut data = String::new();
        try!(io::stderr().write(question.as_bytes()));
        try!(io::stdin().read_line(&mut data));
        Ok(data.trim().to_owned())
    }).unwrap()
}
