use std::io::prelude::*;

use raw_connection::RawConnection;

pub fn send_header(stream: &mut RawConnection){
    let header = "<msg t='sys'><body action='verChk' r='0'><ver v='166' /></body></msg>\0";
    
    let _ = stream.send(header);
    
    let header = stream.recv();
    
    assert_eq!(header,"<msg t='sys'><body action='apiOK' r='0'></body></msg>");
}
