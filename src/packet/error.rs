use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unimplemented")]
    Unimplemented,

    #[error(transparent)]
    HexString(#[from] hex::FromHexError),

    #[error(transparent)]
    Serde(#[from] crate::serde::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
