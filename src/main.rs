#[macro_use]
extern crate lazy_static;
extern crate goodgame_empire_import as gge;
use std::env;
use std::io;
use std::io::Write;

use gge::as_json;
use gge::packet::{ServerPacket, ClientPacket};
use gge::connection::{Connection, DUTCH_SERVER};
use gge::data::DATAMGR;

fn main() {
    io::stderr().write(b"Please login\n").unwrap();
    let un: String = env_or_ask("GGE_USERNAME", "Username: ");
    let pw: String = env_or_ask("GGE_PASSWORD", "Password: ");

    let mut con = Connection::new(*DUTCH_SERVER, &un, &pw);

    for pkt in con.read_packets() {
        process_packet(&mut con, pkt);
    }


    con.send_packet(ClientPacket::Gaa(r#"{"AY1":676,"AY2":688,"KID":0,"AX1":546,"AX2":558}"#.to_string()));
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":676,"AY2":688,"KID":0,"AX1":559,"AX2":571}"#.to_string()));
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":676,"AY2":688,"KID":0,"AX1":572,"AX2":584}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":689,"AY2":701,"KID":0,"AX1":546,"AX2":558}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":689,"AY2":701,"KID":0,"AX1":559,"AX2":571}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":689,"AY2":701,"KID":0,"AX1":572,"AX2":584}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":702,"AY2":714,"KID":0,"AX1":546,"AX2":558}"#.to_string()));
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":702,"AY2":714,"KID":0,"AX1":559,"AX2":571}"#.to_string()));
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":702,"AY2":714,"KID":0,"AX1":572,"AX2":584}"#.to_string()));
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":806,"AY2":818,"KID":0,"AX1":338,"AX2":350}"#.to_string()));
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":663,"AY2":675,"KID":0,"AX1":546,"AX2":558}"#.to_string()));
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":663,"AY2":675,"KID":0,"AX1":559,"AX2":571}"#.to_string()));
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":663,"AY2":675,"KID":0,"AX1":572,"AX2":584}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":650,"AY2":662,"KID":0,"AX1":559,"AX2":571}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":650,"AY2":662,"KID":0,"AX1":572,"AX2":584}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":650,"AY2":662,"KID":0,"AX1":585,"AX2":597}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":663,"AY2":675,"KID":0,"AX1":585,"AX2":597}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":676,"AY2":688,"KID":0,"AX1":585,"AX2":597}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":637,"AY2":649,"KID":0,"AX1":559,"AX2":571}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":637,"AY2":649,"KID":0,"AX1":572,"AX2":584}"#.to_string()));
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":637,"AY2":649,"KID":0,"AX1":585,"AX2":597}"#.to_string()));
    con.send_packet(ClientPacket::Gaa(r#"{"AX2":350,"KID":0,"AY1":806,"AY2":818,"AX1":338}"#.to_string()));

    for pkt in con.read_packets() {
        process_packet(&mut con, pkt);
    }

    println!("");

    for castle in DATAMGR.lock().unwrap().castles.values().take(40) {
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
}

fn process_packet(con: &mut Connection, pkt: ServerPacket) {
    match pkt {
        ServerPacket::Gbd(ref data) => {
            let data = &*data;
            let data = gge::gbd::Gbd::parse(data.to_owned()).unwrap();
            gge::read_castles(data.clone());

            let data_mgr = DATAMGR.lock().unwrap();
            let users = data_mgr.users.values().map(|user| user.clone()).collect::<Vec<_>>();
            for user in users {
                con.send_packet(ClientPacket::Gdi(user.id));
            }
        },
        ServerPacket::Gdi(data) => {
            gge::read_names(data);
        },
        ServerPacket::Gaa(data) => {
            println!("\n\n{}\n\n", data);
            let gaa = gge::map::Gaa::parse(data).unwrap();
            for castle in gaa.castles.iter() {
                DATAMGR.lock().unwrap().add_castle(castle.clone());
            }
            for castle in gaa.castle_names.iter() {
                DATAMGR.lock().unwrap().add_castle(castle.clone());
            }
            for user in gaa.users.iter() {
                DATAMGR.lock().unwrap().users.insert(user.id, user.clone());
            }
            println!("\n\n{:#?}\n\n", gaa);
        },
        _ => {}
    };
}

fn env_or_ask(env_name: &str, question: &str) -> String {
    env::var(env_name)
        .and_then(|data| {
            if data.len() < 2 {
                Err(env::VarError::NotPresent)
            } else {
                Ok(data)
            }
        }).or_else(|_| -> Result<String, io::Error> {
            let mut data = String::new();
            try!(io::stderr().write(question.as_bytes()));
            try!(io::stdin().read_line(&mut data));
            Ok(data.trim().to_owned())
        }).unwrap()
}

fn env_or_default(env_name: &str, default: &str) -> String {
    env::var(env_name)
        .and_then(|data| {
            if data.len() < 2 {
                Err(env::VarError::NotPresent)
            } else {
                Ok(data)
            }
        }).or_else(|_| -> Result<String, io::Error> {
            Ok(default.trim().to_owned())
        }).unwrap()
}
