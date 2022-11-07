use std::error;
use std::fmt;

use serde_json;

use crate::client;

#[derive(Debug)]
pub enum TodorstError {
    Status(client::Response),
    Network(client::Error),
    Parse(serde_json::Error),
}

impl From<client::Error> for TodorstError {
    fn from(e: client::Error) -> Self {
        TodorstError::Network(e)
    }
}

impl From<serde_json::Error> for TodorstError {
    fn from(e: serde_json::Error) -> Self {
        TodorstError::Parse(e)
    }
}

impl fmt::Display for TodorstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TodorstError::Status(ref err) => write!(f, "Status error: {}", err.status()),
            TodorstError::Network(ref err) => err.fmt(f),
            TodorstError::Parse(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for TodorstError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            TodorstError::Status(_) => None,
            TodorstError::Network(ref err) => Some(err),
            TodorstError::Parse(ref err) => Some(err),
        }
    }
}
