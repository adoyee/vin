use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    InvalidData,
    Unimplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidData => write!(f, "invalid data"),
            Error::Unimplemented => write!(f, "unimplemented")
        }
    }
}

impl std::error::Error for Error {}
