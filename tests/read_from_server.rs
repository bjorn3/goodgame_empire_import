extern crate slog;
extern crate slog_term;

extern crate goodgame_empire_import as gge;
use std::io::Write;

use gge::as_json;
use gge::packet::{ServerPacket, ClientPacket};
use gge::connection::{Connection, DUTCH_SERVER};
use gge::data::DATAMGR;

#[test]
fn read_from_server() {
    let log_root = slog::Logger::root(
        slog::Fuse::new(
            slog::LevelFilter::new(slog_term::streamer().build(), slog::Level::Debug)
        ),
        o!("version" => env!("CARGO_PKG_VERSION"))
    );
    
    let un = std::env::var("GGE_USERNAME").unwrap();
    let pw = std::env::var("GGE_PASSWORD").unwrap();

    let mut con = Connection::new(*DUTCH_SERVER, &un, &pw, logger.clone());

    for pkt in con.read_packets() {
        process_packet(&mut con, pkt, logger.clone());
    }

    for _castle in DATAMGR.lock().unwrap().castles.values().take(20) {

    }

    let mut f = std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open("data2.json")
        .unwrap();

    write!(f, "{}", as_json(&*DATAMGR.lock().unwrap())).unwrap();
}

fn process_packet(con: &mut Connection, pkt: ServerPacket, logger: slog::Logger) {
    match pkt {
        ServerPacket::Gbd(ref data) => {
            let data = &*data;
            let data = gge::gbd::Gbd::parse(data.to_owned(), logger.clone()).unwrap();
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
        _ => {}
    };
}
