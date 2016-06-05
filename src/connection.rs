use std::str;
use std::io::prelude::*;
use std::net::TcpStream;

use packet::{ServerPacket, ClientPacket};

///Goodgame empire connection
pub struct Connection{
    stream: TcpStream
}

impl Connection{
    ///Create a new connection
    pub fn new() -> Self {
        let stream = TcpStream::connect(("37.48.88.129",443)).unwrap();
        stream.set_read_timeout(Some(::std::time::Duration::new(2,0))).unwrap();
        
        let mut con = Connection{ stream: stream };
        
        let _ = con.send("<msg t='sys'><body action='verChk' r='0'><ver v='166' /></body></msg>\0");
        let header = con.recv(true);
        
        if header != "<msg t='sys'><body action='apiOK' r='0'></body></msg>"{
            panic!("Prelogin error: received unexpected result: {}", header);
        }

        con
    }

    // raw connection
    fn send(&mut self, data: &str){
        self.stream.write(data.as_bytes()).unwrap();
        println!("Data sent:     {}", data);
    }

    fn recv(&mut self, print: bool) -> String{
        let mut data = [0;8192];
        
        self.stream.read(&mut data).unwrap();
        
        let data = str::from_utf8(&data).expect("Malformed utf8 data provided by the server").trim_matches('\0');
        
        if print{ println!("Data received: {}", data) };

        String::from(data)
    }

    // clean connection
    
    ///Login
    pub fn login(&mut self, un: &str, pw: &str){
        let login_header = r##"<msg t='sys'><body action='login' r='0'><login z='EmpireEx_11'><nick><![CDATA[]]></nick><pword><![CDATA[1455712286016%nl%]]></pword></login></body></msg>"##.to_string() + "\0";
        let login_code = r##"%xt%EmpireEx_11%lli%1%{"CONM":413,"KID":"","DID":"","ID":0,"PW":"{pw}","AID":"1456064275209394654","NOM":"{un}","RTM":129,"LANG":"nl"}%"##.to_string().replace("{pw}", pw).replace("{un}", un) + "\0";
        
        self.send(&login_header);
        self.read_packets(true);
        
        self.send(&login_code);
    }
    
    pub fn send_packet(&mut self, packet: ClientPacket){
        self.send(&packet.to_raw_data());
    }
    
    ///Read packets
    pub fn read_packets(&mut self, print: bool) -> Box<Iterator<Item=ServerPacket>>{
        static SPLIT: &'static [u8] = &[0x00];
        let buf_reader = Box::new(::std::io::BufReader::new(self.stream.try_clone().unwrap()));
        let splitter = ::byte_stream_splitter::ByteStreamSplitter::new(buf_reader, SPLIT);
        
        let data = splitter.map(|splited|String::from_utf8(splited.unwrap()).expect("Malformed utf8 data provided by the server"));
        
        let data = data.map(ServerPacket::new).filter(|packet|{
            if let &ServerPacket::Kpi(_) = packet{
                false
            }else{
                true
            }
        });

        let data = data.map(move |packet|{
            if print{
                println!("Packet received: {:?}\n", packet);
            }
            packet
        });

        Box::new(data)
    }
}
