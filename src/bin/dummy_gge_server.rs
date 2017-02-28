use std::io::{Read, Write, Cursor, copy};
use std::fs::File;
use std::net;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8081").unwrap();

    println!("Running");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut file = File::open("./raw_data.bin").unwrap();
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                buf = buf.replace("@@@NULL@@@", "\0");
                let amount = copy(&mut Cursor::new(buf.into_bytes()), &mut stream)
                    .unwrap_or_else(|e| {
                                        println!("{:?}", e);
                                        0
                                    });
                println!("Send {} bytes", amount);

                let mut buf = Vec::new();
                stream.read_to_end(&mut buf);
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

