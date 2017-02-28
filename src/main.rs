#[macro_use]
extern crate slog;
extern crate slog_scope;
extern crate slog_term;
extern crate slog_stream;
extern crate slog_json;

extern crate gge;

use std::env;
use std::io;
use std::io::Write;

use gge::error::{self, ResultExt};
use gge::to_json;
use gge::packet::{ServerPacket, ClientPacket};
use gge::connection::{Connection, DUTCH_SERVER};
use gge::data::DATAMGR;

fn main() {
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("./log.json")
        .expect("Cant open log file log.json");
    let json_log_formatter = slog_json::new().set_newlines(true).add_default_keys().build();

    let logger = slog::Logger::root(
        slog::Fuse::new(
            slog::Duplicate::new(
                slog::LevelFilter::new(slog_term::streamer().compact().build(), slog::Level::Debug),
                slog_stream::stream(log_file, json_log_formatter)
            )
        ),
        o!("version" => env!("CARGO_PKG_VERSION"))
    );

    slog_scope::set_global_logger(logger.clone());

    if let Err(ref e) = run() {
        let logger = logger.new(o!("error" => ""));

        error!(logger, "error: {}", e);

        for e in e.iter().skip(1) {
            info!(logger, "caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            info!(logger, "backtrace: {:?}", backtrace);
        }

        crit!(logger, "Aborting due to previous error");

        ::std::process::exit(1);
    }
}

fn run() -> gge::error::Result<()> {
    let logger = slog_scope::logger();
    io::stderr().write(b"Please login\n").chain_err(|| "Cant write to stderr")?;
    let un: String = env_or_ask("GGE_USERNAME", "Username: ");
    let pw: String = env_or_ask("GGE_PASSWORD", "Password: ");

    let mut con = Connection::new(*DUTCH_SERVER, &un, &pw, logger.clone())?;

    for pkt in con.read_packets(logger.clone())? {
        slog_scope::scope(logger.new(o!("process"=>"pre map")),
                          || process_packet(&mut con, pkt))?;
    }

    debug!(logger.clone(), "");

    con.send_packet(ClientPacket::Gaa(r#"{"AY1":676,"AY2":688,"KID":0,"AX1":546,"AX2":558}"#.to_string()))?;
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":676,"AY2":688,"KID":0,"AX1":559,"AX2":571}"#.to_string()))?;
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":676,"AY2":688,"KID":0,"AX1":572,"AX2":584}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":689,"AY2":701,"KID":0,"AX1":546,"AX2":558}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":689,"AY2":701,"KID":0,"AX1":559,"AX2":571}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":689,"AY2":701,"KID":0,"AX1":572,"AX2":584}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":702,"AY2":714,"KID":0,"AX1":546,"AX2":558}"#.to_string()))?;
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":702,"AY2":714,"KID":0,"AX1":559,"AX2":571}"#.to_string()))?;
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":702,"AY2":714,"KID":0,"AX1":572,"AX2":584}"#.to_string()))?;
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":806,"AY2":818,"KID":0,"AX1":338,"AX2":350}"#.to_string()))?;
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":663,"AY2":675,"KID":0,"AX1":546,"AX2":558}"#.to_string()))?;
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":663,"AY2":675,"KID":0,"AX1":559,"AX2":571}"#.to_string()))?;
    con.send_packet(ClientPacket::Gaa(r#"{"AY1":663,"AY2":675,"KID":0,"AX1":572,"AX2":584}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":650,"AY2":662,"KID":0,"AX1":559,"AX2":571}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":650,"AY2":662,"KID":0,"AX1":572,"AX2":584}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":650,"AY2":662,"KID":0,"AX1":585,"AX2":597}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":663,"AY2":675,"KID":0,"AX1":585,"AX2":597}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":676,"AY2":688,"KID":0,"AX1":585,"AX2":597}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":637,"AY2":649,"KID":0,"AX1":559,"AX2":571}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":637,"AY2":649,"KID":0,"AX1":572,"AX2":584}"#.to_string()))?;
    //con.send_packet(ClientPacket::Gaa(r#"{"AY1":637,"AY2":649,"KID":0,"AX1":585,"AX2":597}"#.to_string()))?;
    con.send_packet(ClientPacket::Gaa(r#"{"AX2":350,"KID":0,"AY1":806,"AY2":818,"AX1":338}"#.to_string()))?;

    debug!(logger.clone(), "");

    for pkt in con.read_packets(logger.clone())? {
        slog_scope::scope(logger.new(o!("process"=>"post map")), || process_packet(&mut con, pkt))?;
    }

    debug!(logger.clone(), "");

    for castle in DATAMGR.lock().expect("Cant lock DATAMGR").castles.values().take(40) {
        info!(logger.clone(), "     read castle"; "castle" => format!("{:?}", castle));
    }

    let file_name = env_or_default("GGE_FILENAME", "data2.json");

    let mut f = std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .chain_err(||"Cant open data file")?;

    write!(f, "{}", to_json(&*DATAMGR.lock().expect("Cant lock DATAMGR")).chain_err(||"Cant serialize data")?).chain_err(||"Cant write data to file")
}

fn process_packet(con: &mut Connection, pkt: ServerPacket) -> error::Result<()> {
    let logger = slog_scope::logger();
    match pkt {
        ServerPacket::Gbd(ref data) => {
            let data = &*data;
            let data = slog_scope::scope(logger.new(o!("packet"=>"gdb")),
                                         || gge::data_extractors::gbd::Gbd::parse_val(data.to_owned())).chain_err(||"Couldnt read gdb packet")?;
            gge::read_castles(data.clone());

            let data_mgr = DATAMGR.lock().unwrap();
            let users = data_mgr.users.values().map(|user| user.clone()).collect::<Vec<_>>();
            for user in users {
                con.send_packet(ClientPacket::Gdi(user.id))?;
            }
        }
        ServerPacket::Gdi(data) => {
            slog_scope::scope(logger.new(o!("packet"=>"gdi")),
                              || gge::read_names(data))?;
        }
        ServerPacket::Gaa(data) => {
            trace!(logger, "gaa packet"; "data" => data);
            let gaa = slog_scope::scope(logger.new(o!("packet"=>"gaa")), || gge::data_extractors::map::Gaa::parse(data)).chain_err(||"Couldnt read gaa packet")?;
            for castle in gaa.castles.iter() {
                DATAMGR.lock().unwrap().add_castle(castle.clone());
            }
            for castle in gaa.castle_names.iter() {
                DATAMGR.lock().unwrap().add_castle(castle.clone());
            }
            for user in gaa.users.iter() {
                DATAMGR.lock().unwrap().users.insert(user.id, user.clone());
            }
            //trace!(logger, "gaa  data"; "parsed" => format!("{:?}", gaa));
        }
        _ => {}
    };
    Ok(())
}

fn env_or_ask(env_name: &str, question: &str) -> String {
    env::var(env_name)
        .and_then(|data| {
            if data.len() < 2 {
                Err(env::VarError::NotPresent)
            } else {
                Ok(data)
            }
        })
        .or_else(|_| -> error::Result<_> {
            let mut data = String::new();
            try!(io::stderr().write(question.as_bytes()));
            try!(io::stdin().read_line(&mut data));
            Ok(data.trim().to_string())
        })
        .unwrap()
}

fn env_or_default(env_name: &str, default: &str) -> String {
    env::var(env_name)
        .and_then(|data| {
            if data.len() < 2 {
                Err(env::VarError::NotPresent)
            } else {
                Ok(data)
            }
        })
        .unwrap_or_else(|_| default.trim().to_string() )
}
