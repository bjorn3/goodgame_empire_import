use std::str;
use std::net::{TcpStream, SocketAddr};

use slog::*;

use error::{Result, ResultExt};
use smartfox::{SmartFoxClient, SmartFoxPacket};
use packet::{ServerPacket, ClientPacket};

/// Goodgame empire connection
pub struct Connection {
    smartfox: SmartFoxClient,
    logger: Logger,
}

lazy_static!{
    /// The Dutch server (37.48.88.129)
    pub static ref DUTCH_SERVER: SocketAddr = "37.48.88.129:443".parse().unwrap();
    /// Local server (127.0.0.1:8081)
    pub static ref LOCAL_SERVER: SocketAddr = "127.0.0.1:8081".parse().unwrap();
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
    pub fn new(server: SocketAddr, un: &str, pw: &str, logger: Logger) -> Result<Self> {
        let stream = try!(TcpStream::connect(server)
            .chain_err(|| "Can't connect to server"));
        try!(stream.set_read_timeout(Some(::std::time::Duration::new(2, 0)))
            .chain_err(|| "Can't set server connection timeout"));

        //                                          room               02/17/2016 @ 12:31pm (UTC) unix timestamp with millisecond precision
        //                                          v                  v
        let smartfox = SmartFoxClient::new(stream, "EmpireEx_11", "", "1455712286016%nl%", logger.clone())?;
        let mut con = Connection {
            smartfox: smartfox,
            logger: logger
        };

        let login_code = r##"%xt%EmpireEx_11%lli%1%{"RTM":32,"FID":null,"ID":0,"PW":"{pw}","FTK":null,"REF":"http://empire.goodgamestudios.com","FAID":null,"KID":"","LANG":"nl","NOM":"{un}","AID":"1433061122034798333","CONM":182,"DID":""}%"##.to_string().replace("{pw}", pw).replace("{un}", un);

        con.smartfox.send_packet(SmartFoxPacket(login_code))?;

        Ok(con)
    }

    // clean connection

    /// Send gge packet
    pub fn send_packet(&mut self, packet: ClientPacket) -> Result<()> {
        self.smartfox.send_packet(SmartFoxPacket(packet.to_raw_data()))?;
        debug!(self.logger, "     send packet"; "packet" => format!("{:?}", packet));
        Ok(())
    }

    /// Read gge packets
    ///
    /// Ignores kpi and irc packets
    pub fn read_packets(&mut self, logger: Logger) -> Result<Box<Iterator<Item = ServerPacket>>> {
        let data = self.smartfox
            .read_packets(logger.clone())
            .chain_err(|| "Couldnt read packets")?
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
                trace!(logger, " received packet"; "packet" => format!("{:?}", packet));
                packet
            });

        Ok(Box::new(data))
    }
}
