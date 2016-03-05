extern crate rustc_serialize;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::io;
use std::io::Write;

mod raw_connection;
mod packet;
mod data;
mod gbd;
mod connection;

use raw_connection::RawConnection;
use packet::Packet;
use connection::Connection;
use data::Castle;
use data::World;

fn main() {
    let mut con = Connection::new();
    
    io::stderr().write(b"Please login\n").unwrap();
    
    let un: String = env::var("GGE_USERNAME").and_then(|data|{
        if data.len() < 2{
            Err(env::VarError::NotPresent)
        }else{
            Ok(data)
        }
    }).or_else(|_| -> Result<String,io::Error>{
        let mut un = String::new();
        try!(io::stderr().write(b"Username: "));
        try!(io::stdin().read_line(&mut un));
        Ok(un.trim().to_owned())
    }).unwrap();
    
    let pw: String = env::var("GGE_PASSWORD").and_then(|data|{
        if data.len() < 2{
            Err(env::VarError::NotPresent)
        }else{
            Ok(data)
        }
    }).or_else(|_| -> Result<String,io::Error>{
        let mut pw = String::new();
        try!(io::stderr().write(b"Password: "));
        try!(io::stdin().read_line(&mut pw));
        Ok(pw.trim().to_owned())
    }).unwrap();

    con.login(&un, &pw);
    
    for pkt in &con.read_packets(true){
        match *pkt{
            Packet::Gbd(ref data) => {
                let data = &*data;
                let data = gbd::Gbd::parse(data.to_owned()).unwrap();
                read_castles(data.clone());
                println!("{}", data);
            },
            _ => continue
        };
    }

    for castle in data::CASTLES.lock().unwrap().iter(){
        println!("{:?}", castle);
    }
}

fn read_castles(data: gbd::Gbd){
    let pid = data.gcl.find("PID").unwrap().as_u64().unwrap();
    let dcl = data.gcl.find("C").unwrap().as_array().unwrap();
    for world in dcl{
        let world_name = World::from_int(world.find("KID").unwrap().as_u64().unwrap());
        let castles = world.find("AI").expect("No world AI").as_array().expect("AI not a array");
        println!("world");
        for castle in castles{
            if castle.find("AI") == None{
                println!("{:?}", castle);
                continue;
            }
            let castle = castle.find("AI").expect("No castle AI").as_array().expect("AI not a array");
            let castle = Castle{
                id: castle[3].as_u64().unwrap(),
                owner_id: Some(pid),
                name: Some(castle[10].as_string().unwrap().to_owned()),
                x: castle[1].as_u64(),
                y: castle[2].as_u64(),
                world: Some(world_name)
            };
            println!("{:?}", castle);
            data::CASTLES.lock().unwrap().add(castle);
        }
    }

    for ain in data.ain{
        for castle in ain.AP{
            data::CASTLES.lock().unwrap().add(castle);
        }
        for castle in ain.VP{
            data::CASTLES.lock().unwrap().add(castle);
        }
    }
}
