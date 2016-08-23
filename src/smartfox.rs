use std::str;
use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct SmartFoxPacket{
    pub data: String
}

impl fmt::Debug for SmartFoxPacket{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{:?}", self.data)
    }
}

#[allow(non_snake_case)]
pub fn SmartFoxPacket<T>(data: T) -> SmartFoxPacket where T: Into<String>{
    SmartFoxPacket{
        data: data.into()
    }
}

///Goodgame empire connection
pub struct SmartFoxClient{
    pub stream: TcpStream
}

impl SmartFoxClient{
    ///Create a new connection
    pub fn new(stream: TcpStream, room: &str, un: &str, pw: &str) -> Self {
        stream.set_read_timeout(Some(::std::time::Duration::new(2,0))).unwrap();
        
        let mut con = SmartFoxClient{ stream: stream };
        
        let _ = con.send_packet(SmartFoxPacket("<msg t='sys'><body action='verChk' r='0'><ver v='166' /></body></msg>".to_string()));
        let header = con.recv();
        
        if header != "<msg t='sys'><body action='apiOK' r='0'></body></msg>"{
            panic!("Invalid server version: {}", header);
        }
        //                                                                            room               username                    password
        //                                                                            v                  v                           v
        let login_header = format!("<msg t='sys'><body action='login' r='0'><login z='{}'><nick><![CDATA[{}]]></nick><pword><![CDATA[{}]></pword></login></body></msg>", room, un, pw);
        
        con.send_packet(SmartFoxPacket(login_header));
        con.read_packets();

        con
    }

    // raw connection
    pub fn recv(&mut self) -> String{
        let mut data = [0;8192];
        
        self.stream.read(&mut data).unwrap();
        
        let data = str::from_utf8(&data).expect("Malformed utf8 data provided by the server").trim_matches('\0');
        
        println!("Data received: {}", data);

        String::from(data)
    }

    // clean connection

    pub fn send_packet(&mut self, packet: SmartFoxPacket){
        let data = packet.data + "\0";
        self.stream.write(data.as_bytes()).unwrap();
        println!("Data sent:     {}", data);
    }
    
    ///Read packets
    pub fn read_packets(&mut self) -> Box<Iterator<Item=SmartFoxPacket>>{
        static SPLIT: &'static [u8] = &[0x00];
        let buf_reader = Box::new(::std::io::BufReader::new(self.stream.try_clone().unwrap()));
        let splitter = ::byte_stream_splitter::ByteStreamSplitter::new(buf_reader, SPLIT);
        
        let data = splitter.map(|splited|String::from_utf8(splited.unwrap()).expect("Malformed utf8 data provided by the server"));
        
        Box::new(data.map(SmartFoxPacket))
    }
}
