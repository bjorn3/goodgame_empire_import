use std::io;
use std::borrow::Cow;

use slog::*;

use rustc_serialize::json;

error_chain!{
    types{
        Error, ErrorKind, ChainErr, Result;
    }

    foreign_links{
        io::Error, IoError;
        json::ParserError,  JsonParserError;
        json::DecoderError, JsonDecoderError;
        json::EncoderError, JsonEncoderError;
    }

    errors{
        InvalidFormat(descr: Cow<'static, str>){
            description("invalid format")
            display("The json returned from the server has a invalid format: {}", descr)
        }
    }
}

pub fn print_error(error: Error, logger: Logger) -> !{
    error!(logger, format!("{}\n{:?}\n{:?}", error, (error.1).0, (error.1).1).replace('\n', "\n    "));
    panic!()
}

pub trait ErrorExt<T>{
    fn unwrap_pretty(self, logger: Logger) -> T;
}

impl<T> ErrorExt<T> for Result<T>{
    fn unwrap_pretty(self, logger: Logger) -> T{
        self.unwrap_or_else(|err|print_error(err, logger))
    }
}
