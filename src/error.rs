use rustc_serialize::json;

use std::fmt;
use std::error;

/// Generic error type
#[derive(Debug)]
pub enum Error {
    /// Can't parse the data returned from the server
    InvalidFormat,
    /// Can't parse the json data
    JsonError(JsonError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured:\n{}", error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidFormat => {
                "Can't parse the data returned from the server.\n\
                 This is likely a bug in this program.\n\
                 Please report this at \
                 https://github.com/bjorn3/goodgame_empire_import/issues."
            },
            Error::JsonError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidFormat => None,
            Error::JsonError(ref err) => err.cause(),
        }
    }
}

/// Cast json errors to Error
impl<T> From<T> for Error where JsonError: From<T> {
    fn from(err: T) -> Error {
        Error::JsonError(From::from(err))
    }
}

/// Generic json error
#[derive(Debug)]
pub enum JsonError {
    ParserError(json::ParserError),
    DecoderError(json::DecoderError),
    EncoderError(json::EncoderError),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured:\n{}", error::Error::description(self))
    }
}

impl error::Error for JsonError {
    fn description(&self) -> &str {
        match *self {
            JsonError::ParserError(ref err) => err.description(),
            JsonError::DecoderError(ref err) => err.description(),
            JsonError::EncoderError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            JsonError::ParserError(ref err) => err.cause(),
            JsonError::DecoderError(ref err) => err.cause(),
            JsonError::EncoderError(ref err) => err.cause(),
        }
    }
}

impl From<json::ParserError> for JsonError {
    fn from(err: json::ParserError) -> JsonError {
        JsonError::ParserError(err)
    }
}


impl From<json::DecoderError> for JsonError {
    fn from(err: json::DecoderError) -> JsonError {
        JsonError::DecoderError(err)
    }
}

impl From<json::EncoderError> for JsonError {
    fn from(err: json::EncoderError) -> JsonError {
        JsonError::EncoderError(err)
    }
}
