#[macro_use] extern crate lazy_static;
extern crate goodgame_empire_import as gge;
use std::env;
use std::io;
use std::io::Write;
use std::sync::Mutex;

use gge::as_json;
use gge::packet::{ServerPacket, ClientPacket};
use gge::connection::{Connection, DUTCH_SERVER};
use gge::data::DATAMGR;

lazy_static!{
    static ref FOUNDGBDPACKET: Mutex<bool> = Mutex::new(false);
}

fn main() {
    io::stderr().write(b"Please login\n").unwrap();
    let un: String = env_or_ask("GGE_USERNAME", "Username: ");
    let pw: String = env_or_ask("GGE_PASSWORD", "Password: ");

    let mut con = Connection::new(*DUTCH_SERVER, &un, &pw);
    
    for pkt in con.read_packets(true){
        process_packet(&mut con, pkt);
    }
    
    for castle in DATAMGR.lock().unwrap().castles.values().take(20){
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
    
    write!(f, "{}", as_json(&*DATAMGR.lock().unwrap())).unwrap();
    
    if !*FOUNDGBDPACKET.lock().unwrap(){
        io::stderr().write(b"Login failed\n").unwrap();
    }
}

fn process_packet(con: &mut Connection, pkt: ServerPacket){
    match pkt{
        ServerPacket::Gbd(ref data) => {
            let data = &*data;
            let data = gge::gbd::Gbd::parse(data.to_owned()).unwrap();
            gge::read_castles(data.clone());
            *FOUNDGBDPACKET.lock().unwrap() = true;

            let data_mgr = DATAMGR.lock().unwrap();
            let users = data_mgr.users.values().map(|user|user.clone()).collect::<Vec<_>>();
            for user in users{
                con.send_packet(ClientPacket::Gdi(user.id));
            }
        },
        ServerPacket::Gdi(data) => {
            gge::read_names(data);
        },
        _ => {}
    };
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
