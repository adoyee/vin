use std::fmt::Display;
use std::{io, str, string};

use serde::{de, ser};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed with reason: {0}")]
    Custom(String),

    #[error("unsupported type for serde")]
    Unsupported,

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Utf8(#[from] str::Utf8Error),

    #[error(transparent)]
    FromUtf8(#[from] string::FromUtf8Error),
    #[error("GBK decode/encode")]
    GBK,
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
