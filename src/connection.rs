use std::str;
use std::io::prelude::*;
use std::net::TcpStream;

use packet::Packet;

pub struct Connection{
    stream: TcpStream
}

impl Connection{
    pub fn new() -> Self {
        let stream = TcpStream::connect(("37.48.88.129",443)).unwrap();
        stream.set_read_timeout(Some(::std::time::Duration::new(1,0))).unwrap();
        
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

    #[allow(unused_must_use)] //recv_var MUST NOT panic when there is a timeout error
    fn recv_var(&mut self, print:bool) -> String{
        let mut string = String::new();
        self.stream.read_to_string(&mut string);
        if print{ println!("Data received: {}", string) };
        string
    }
    

    // clean connection
    pub fn login(&mut self, un: &str, pw: &str){
        let login_header = r##"<msg t='sys'><body action='login' r='0'><login z='EmpireEx_11'><nick><![CDATA[]]></nick><pword><![CDATA[1455712286016%nl%]]></pword></login></body></msg>"##.to_string() + "\0";
        let login_code = r##"%xt%EmpireEx_11%lli%1%{"CONM":413,"KID":"","DID":"","ID":0,"PW":"{pw}","AID":"1456064275209394654","NOM":"{un}","RTM":129,"LANG":"nl"}%"##.to_string().replace("{pw}", pw).replace("{un}", un) + "\0";
        
        self.send(&login_header);
        self.recv(true);
        
        self.send(&login_code);
        self.recv(true);
    }
    
    pub fn read_data(&mut self, print: bool) -> Vec<String>{
        let mut data = String::new();
        
        loop{
            let new_data = self.recv_var(false);
            if new_data.is_empty(){
                break;
            }
            data = data + &*new_data;
        }
        
        let data = data.split('\0').collect::<Vec<&str>>();
        
        if print{
            println!("{}", data.join("\n\n"))
        };
        
        return data.iter().map(|d| d.to_string()).collect::<Vec<String>>();
    }
    
    pub fn read_packets(&mut self, print: bool) -> Vec<Packet>{
        let data = self.read_data(false);
        let packets = data.iter().map(|d|{
            let packet = Packet::new(d.to_string());
            if let Packet::Kpi(_) = packet{
                return packet;
            }

            if print{
                println!("{:#?}\n", packet);
            }

            packet
        }).collect::<Vec<Packet>>();
        packets
    }
}
