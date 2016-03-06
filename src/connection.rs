use std::net::TcpStream;

use raw_connection::RawConnection;
use packet::Packet;

pub struct Connection{
    pub raw_con: RawConnection
}

impl Connection{
    pub fn new() -> Self {
        let stream = TcpStream::connect(("37.48.88.129",443)).unwrap();
        stream.set_read_timeout(Some(::std::time::Duration::new(1,0))).unwrap();
        let raw_con = RawConnection::new(stream);
        
        let mut con = Connection{ raw_con: raw_con };
        
        let header = "<msg t='sys'><body action='verChk' r='0'><ver v='166' /></body></msg>\0";
        let expected = "<msg t='sys'><body action='apiOK' r='0'></body></msg>";
        
        let _ = con.raw_con.send(header);
        
        let header = con.raw_con.recv(true);
        
        assert_eq!(header,expected);
        
        con
    }
    
    pub fn login(&mut self, un: &str, pw: &str){
        let login_header = r##"<msg t='sys'><body action='login' r='0'><login z='EmpireEx_11'><nick><![CDATA[]]></nick><pword><![CDATA[1455712286016%nl%]]></pword></login></body></msg>"##.to_string() + "\0";
        let login_code = r##"%xt%EmpireEx_11%lli%1%{"CONM":413,"KID":"","DID":"","ID":0,"PW":"{pw}","AID":"1456064275209394654","NOM":"{un}","RTM":129,"LANG":"nl"}%"##.to_string().replace("{pw}", pw).replace("{un}", un) + "\0";
        
        self.raw_con.send(&login_header);
        
        self.raw_con.recv(true);
        
        
        self.raw_con.send(&login_code);
        
        self.raw_con.recv(true);
    }
    
    pub fn read_data(&mut self, print: bool) -> Vec<String>{
        let mut data = String::new();
        
        loop{
            let new_data = self.raw_con.recv_var(false);
            if new_data.is_empty(){
                break;
            }
            data = data + &*new_data;
        }
        
        let data = data.split('\0').collect::<Vec<&str>>();
        
        if print{ println!("{}", data.join("\n\n")) };
        
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
