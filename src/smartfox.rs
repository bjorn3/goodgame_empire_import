use std::str;
use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream;

use slog::*;

use error::{Result, ResultExt};

pub struct SmartFoxPacket {
    pub data: String,
}

impl fmt::Debug for SmartFoxPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

/// Create a SmartFoxServer packet from a string
#[allow(non_snake_case)]
pub fn SmartFoxPacket<T>(data: T) -> SmartFoxPacket
where
    T: Into<String>,
{
    SmartFoxPacket { data: data.into() }
}

/// Goodgame empire connection
pub struct SmartFoxClient {
    pub stream: TcpStream,
    logger: Logger,
}

impl SmartFoxClient {
    /// Create a new connection
    ///
    /// # Sends and receives
    /// ## check version:
    ///
    /// ```xml
    /// send: <msg t='sys'><body action='verChk' r='0'><ver v='166' /></body></msg>
    /// recv: <msg t='sys'><body action='apiOK' r='0'></body></msg>
    /// ```
    ///
    /// ## login
    ///
    /// ```xml
    /// send: <msg t='sys'><body action='login' r='0'>
    ///           <login z='<#room#>'>
    ///               <nick><![CDATA[<#username#>]]></nick>
    ///               <pword><![CDATA[<#password#>]]></pword>
    ///           </login>
    ///       </body></msg>
    /// ```
    pub fn new(
        stream: TcpStream,
        room: &str,
        username: &str,
        password: &str,
        logger: Logger,
    ) -> Result<Self> {
        stream
            .set_read_timeout(Some(::std::time::Duration::new(2, 0)))
            .chain_err(|| "Couldnt set stream timeout")?;

        let mut con = SmartFoxClient {
            stream: stream,
            logger: logger.clone(),
        };

        let ver_chk_msg = "<msg t='sys'><body action='verChk' r='0'><ver v='166' /></body></msg>";
        let _ = con.send_packet(SmartFoxPacket(ver_chk_msg.to_string()));
        let header = con.recv().chain_err(|| "Couldn't connect to server")?;

        if header != "<msg t='sys'><body action='apiOK' r='0'></body></msg>" {
            panic!("Invalid server version: {}", header);
        }

        let login_header = format!(
            "<msg t='sys'><body action='login' r='0'><login z='{}'><nick><![CDATA[{}]]></nick><pword><![CDATA[{}]]></pword></login></body></msg>",
            room,
            username,
            password
        );

        con.send_packet(SmartFoxPacket(login_header)).chain_err(
            || "Couldn't connect to server",
        )?;
        con.read_packets(logger).chain_err(
            || "Couldn't connect to server",
        )?;

        Ok(con)
    }

    // raw connection
    fn recv(&mut self) -> Result<String> {
        let mut data = [0; 8192];

        self.stream.read(&mut data).chain_err(
            || "Couldnt read from stream",
        )?;

        let data = str::from_utf8(&data)
            .chain_err(|| "Malformed utf8 data provided by the server")?
            .trim_matches('\0')
            .to_string();

        trace!(self.logger.clone(), "   smartfox recv"; "data" => data.clone());

        Ok(data)
    }

    // clean connection

    /// Send a zero terminated packet
    pub fn send_packet(&mut self, packet: SmartFoxPacket) -> Result<()> {
        let data = packet.data + "\0";
        self.stream.write(data.as_bytes()).chain_err(
            || "Cant write to server stream",
        )?;
        Ok(())
    }

    /// Read zero terminated packets
    pub fn read_packets(&mut self, logger: Logger) -> Result<Box<Iterator<Item = SmartFoxPacket>>> {
        static SPLIT: &'static [u8] = &[0x00];
        let buf_reader = Box::new(::std::io::BufReader::new(
            self.stream.try_clone().chain_err(|| "Couldnt clone stream")?,
        ));
        let splitter = ::byte_stream_splitter::ByteStreamSplitter::new(buf_reader, SPLIT);

        let data = splitter
            .map(|splited| {
                String::from_utf8(splited.unwrap()).expect(
                    "Malformed utf8 data provided by the server",
                )
            })
            .map(move |data| {
                trace!(logger.clone(), "Received data"; "data" => data.clone());
                data
            });

        Ok(Box::new(data.map(SmartFoxPacket)))
    }
}
