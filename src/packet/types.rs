use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::packet::error;

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Encrypt {
    Plain = 0x01,
    Rsa = 0x02,
    Aes128 = 0x03,
}

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait Parser {
    fn parse<P>(buff: &[u8]) -> Result<P>;
}
