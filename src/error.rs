use crate::client::ClientError;
use std::time::SystemTimeError;
use std::{fmt, io};

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self {
            message: format!("io error: {}", err),
        }
    }
}

impl From<SystemTimeError> for Error {
    fn from(err: SystemTimeError) -> Self {
        Self {
            message: format!("system time error: {}", err),
        }
    }
}

impl From<ClientError> for Error {
    fn from(err: ClientError) -> Self {
        Self {
            message: format!("client error: {}", err),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self {
            message: format!("serde json error: {}", err),
        }
    }
}
