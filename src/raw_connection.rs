use std::io::prelude::*;
use std::net::TcpStream;

const RAW_PRINT: bool = false;

pub struct RawConnection{
    stream: TcpStream
}

impl RawConnection{
    pub fn new(stream: TcpStream) -> Self{
        RawConnection{ stream: stream }
    }
    
    pub fn send(&mut self, data: &str){
        let _ = self.stream.write(data.as_bytes());
        println!("Data sent:     {}", data);
    }
    
    pub fn recv(&mut self, print: bool) -> String{
        let mut data = [0;8192];
        let _ = self.stream.read(&mut data);
        
        let header = ::std::str::from_utf8(&data).unwrap().trim_matches('\0');
        
        if print && RAW_PRINT{
            println!("Raw data received:");
            for char in header.bytes(){
                print!("{:x} ", char);
            }
            println!("");
        }
        if print{ println!("Data received: {}", header) };
        
        String::from(header)
    }
    
    pub fn recv_var(&mut self, print:bool) -> String{
        let mut str = String::new();
        self.stream.read_to_string(&mut str);
        if print{ println!("Data received: {}", str) };
        str
    }
}

impl ::std::ops::Deref for RawConnection{
    type Target = TcpStream;
    fn deref(&self) -> &TcpStream{
        &self.stream
    }
}

impl ::std::ops::DerefMut for RawConnection{
    fn deref_mut(&mut self) -> &mut TcpStream{
        &mut self.stream
    }
}
