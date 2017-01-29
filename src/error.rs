use std::io;
use std::borrow::Cow;

use serde_json::error::Error as SerdeJsonError;

error_chain!{
    types{
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links{
        IoError(io::Error);
        SerdeJsonError(SerdeJsonError);
    }

    errors{
        InvalidFormat(descr: Cow<'static, str>){
            description("invalid format")
            display("The json returned from the server has a invalid format: {}", descr)
        }
    }
}
