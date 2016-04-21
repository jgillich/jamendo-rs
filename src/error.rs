use std;
use serde_json;
use hyper;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
    Client(ErrorKind),
}


impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Hyper(ref e) => e.description(),
            &Error::Io(ref e) => e.description(),
            &Error::Json(ref e) => e.description(),
            &Error::Client(ref e) => match e {
                &ErrorKind::InvalidQuery => "The query is not valid for the resource",
                &ErrorKind::ResourceNotFound => "The resource does not exist",
                &ErrorKind::Api((_, ref message)) => &message,
            }
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            _ => None,
        }
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", std::error::Error::description(self))
    }
}


impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<ErrorKind> for Error {
    fn from(err: ErrorKind) -> Error {
        Error::Client(err)
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub enum ErrorKind {
    InvalidQuery,
    ResourceNotFound,
    Api((u32, String))
}
