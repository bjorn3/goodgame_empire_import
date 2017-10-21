extern crate regex;

extern crate futures;
extern crate tokio_core;
extern crate tokio_service;

extern crate smartfox;

use futures::{future, Future, BoxFuture, Stream, Sink};
use tokio_core::reactor::Core;
use tokio_core::io::{copy, Io, Framed};
use tokio_core::net::TcpListener;
use tokio_service::{Service, NewService};

use smartfox::{SmartFoxCodec, SmartFoxService, Delegate};

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let listener = TcpListener::bind(&"127.0.0.1:8081".parse().unwrap(), &handle).unwrap();

    let connections = listener.incoming();
    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (writer, reader) = socket.framed(SmartFoxCodec).split();
        let service = (|| Ok(SmartFoxService::new(ServerDelegate { loggedin: false })))
            .new_service()?;

        let responses = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(responses).then(|_| Ok(()));
        handle.spawn(server);

        Ok(())
    });

    core.run(server).unwrap();
}


struct ServerDelegate {
    loggedin: bool,
}

impl Delegate for ServerDelegate {
    fn login(&mut self, _room: &str, _un: &str, _pw: &str) -> Vec<smartfox::packet::Packet> {
        //const RESPONSE: &'static str = include_str!("../../raw_data.bin");
        //let res = RESPONSE.split('\n').map(|pkt|pkt.parse::<smartfox::packet::Packet>().unwrap()).collect::<Vec<_>>();
        //println!("{:?}\n{:?}", res[0], res[10]);
        //res
        Vec::new()
    }

    fn request(&mut self, packet: smartfox::packet::Packet) -> Vec<smartfox::packet::Packet> {
        const RESPONSE: &'static str = include_str!("../../raw_data.bin");
        let res = RESPONSE
            .split('\n')
            .map(|pkt| pkt.parse::<smartfox::packet::Packet>().unwrap())
            .collect::<Vec<_>>();
        println!("{:?}", packet);
        if !self.loggedin {
            println!("{:#?}", res);
            self.loggedin = true;
            res
        } else {
            Vec::new()
        }
    }
}
