extern crate goodgame_empire_import as gge;
use std::env;
use std::io;
use std::io::Write;

use gge::as_json;
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
    
    for pkt in con.read_packets(true){
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
    
    let file_name = env_or_default("GGE_FILENAME", "data2.json");
    
    let mut f = std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .unwrap();
    
    write!(f, "{}", as_json(&data_mgr)).unwrap();
    
    if !found_gbd_packet{
        io::stderr().write(b"Login failed\n").unwrap();
    }
}

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

fn env_or_default(env_name: &str, default: &str) -> String{
    env::var(env_name).and_then(|data|{
        if data.len() < 2{
            Err(env::VarError::NotPresent)
        }else{
            Ok(data)
        }
    }).or_else(|_| -> Result<String,io::Error>{
        Ok(default.trim().to_owned())
    }).unwrap()
}
