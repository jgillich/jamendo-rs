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
pub enum ErrorKind {
    ResourceNotFound,
    Api((i32, String))
}
