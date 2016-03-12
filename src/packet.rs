macro_rules! try_packet{
    ($data: expr, $reg: expr, $variant: expr) => {
        if $data.find($reg) == Some(0){
            return $variant($data.replace($reg, ""));
        }
    };
}

///A server returned packet of data
#[derive(Debug)]
pub enum Packet{
    Data(String),
    Kpi(String),
    Gam(String),
	Gbd(String),
    None
}

impl Packet{
    ///Create a packet from text
    pub fn new(data: String) -> Self{
        if data.is_empty(){
            return Packet::None;
        }
        try_packet!(data, "%xt%kpi%1%0%", Packet::Kpi);
        try_packet!(data, "%xt%gam%1%0%", Packet::Gam);
		try_packet!(data, "%xt%gbd%1%0%", Packet::Gbd);
        Packet::Data(data)
    }
}
