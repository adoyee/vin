use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::packet::error;
use crate::serde::gbk;

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
    None = 0x01,
    Rsa = 0x02,
    Aes128 = 0x03,
}

pub struct VinOpts {}

impl gbk::Options for VinOpts {
    const LENGTH: usize = 17;
}

pub type Vin = gbk::GBKString<VinOpts>;

pub struct IccidOpts {}

impl gbk::Options for IccidOpts {
    const LENGTH: usize = 20;
}

pub type Iccid = gbk::GBKString<IccidOpts>;

pub mod info {}
