use std::fmt;

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
