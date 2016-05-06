use data::DataMgr;

///Format the data as json
pub fn format_json(data_mgr: &DataMgr) -> String{
    let mut json = "[".to_owned();
    
    let mut iter = data_mgr.castles.values().peekable();
    loop{
        if let Some(castle) = iter.next(){
            let has_next = match iter.peek(){
                Some(_) => true,
                None => false
            };
            json = json + &format!("{}{}\n", castle, if has_next{
                ","
            }else{
                ""
            });
        }else{
            break;
        }
    }
    
    json + "]"
}
