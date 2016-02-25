extern crate rustc_serialize;

mod raw_connection;
mod packet;
mod gbd;
mod connection;

use raw_connection::RawConnection;
use packet::Packet;
use connection::Connection;

fn main() {
    let mut con = Connection::new();
    
    let stdin = std::io::stdin();
    let mut un = String::new();
    let mut pw = String::new();
    
    println!("Username: ");
    stdin.read_line(&mut un).unwrap();
    println!("Password: ");
    stdin.read_line(&mut pw).unwrap();
    let un = un.trim();
    let pw = pw.trim();
    
    con.login(un, pw);
    
    for pkt in &con.read_packets(true){
        match *pkt{
            Packet::Gbd(ref data) => {
                let data = &*data;
                println!("{:#?}", gbd::Gbd::parse(data.to_owned()).unwrap());
            },
            _ => continue
        };
    }
}
