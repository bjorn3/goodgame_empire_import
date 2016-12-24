use std::str;
use std::net::{TcpStream, IpAddr, Ipv4Addr};

use slog::*;

use error::{Result, ChainErr};
use smartfox::{SmartFoxClient, SmartFoxPacket};
use packet::{ServerPacket, ClientPacket};

/// Goodgame empire connection
pub struct Connection {
    smartfox: SmartFoxClient,
    logger: Logger,
}

lazy_static!{
    /// The Dutch server (37.48.88.129)
    pub static ref DUTCH_SERVER: IpAddr = IpAddr::V4(Ipv4Addr::new(37, 48, 88, 129));
}

impl Connection {
    /// Create a new connection
    ///
    /// ## SmartFoxServer settings
    /// * room: "EmpireEx_11"
    /// * username: ""
    /// * password: "1455712286016%nl%" `02/17/2016 @ 12:31pm (UTC)` unix timestamp with millisecond precision
    /// # Sends and receives
    /// ## login
    ///
    /// ```xml
    /// send: %xt%EmpireEx_11%lli%1%{"CONM":413,"KID":"","DID":"","ID":0,"PW":"<#password#>","AID":"1456064275209394654","NOM":"<#username#>","RTM":129,"LANG":"nl"}%
    /// ```
    pub fn new(server: IpAddr, un: &str, pw: &str, logger: Logger) -> Result<Self> {
        let stream = try!(TcpStream::connect((server, 443)).chain_err(||"Can't connect to server"));
        try!(stream.set_read_timeout(Some(::std::time::Duration::new(2, 0))).chain_err(||"Can't set server connection timeout"));

        //                                          room               02/17/2016 @ 12:31pm (UTC) unix timestamp with millisecond precision
        //                                          v                  v
        let smartfox = SmartFoxClient::new(stream, "EmpireEx_11", "", "1455712286016%nl%", logger.clone())?;
        let mut con = Connection { smartfox: smartfox, logger: logger };

        let login_code = r##"%xt%EmpireEx_11%lli%1%{"CONM":413,"KID":"","DID":"","ID":0,"PW":"{pw}","AID":"1456064275209394654","NOM":"{un}","RTM":129,"LANG":"nl"}%"##.to_string().replace("{pw}", pw).replace("{un}", un);

        con.smartfox.send_packet(SmartFoxPacket(login_code))?;

        Ok(con)
    }

    // clean connection

    /// Send gge packet
    pub fn send_packet(&mut self, packet: ClientPacket) -> Result<()>{
        self.smartfox.send_packet(SmartFoxPacket(packet.to_raw_data()))?;
        debug!(self.logger, "     send packet"; "packet" => format!("{:?}", packet));
        Ok(())
    }

    /// Read gge packets
    ///
    /// Ignores kpi and irc packets
    pub fn read_packets(&mut self, logger: Logger) -> Box<Iterator<Item = ServerPacket>> {
        //let logger = self.logger.clone();
        let data = self.smartfox
            .read_packets(logger)
            .map(|p| ServerPacket::new(p.data).expect("Received invalid packet"))
            .filter(|packet| {
                // Ignore kpi and irc packets
                match *packet {
                    ServerPacket::Kpi(_) |
                    ServerPacket::Irc(_) => false,
                    _ => true,
                }
            })
            .map(move |packet| {
                //debug!(logger, " received packet"; "packet" => format!("{:?}", packet));
                packet
            });

        Box::new(data)
    }
}
