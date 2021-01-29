use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub mod body;
pub mod error;
pub mod types;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Command {
    VehicleLogin = 0x01,
    RealTimeReport = 0x02,
    ReissueReport = 0x03,
    VehicleLogout = 0x04,
    PlatformLogin = 0x5,
    PlatformLogout = 0x6,
    HeartBeat = 0x7,
    Time = 0x8,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Response {
    Success,
    Fail,
    RepeatedVin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub begin: u16,
    pub command: Command,
    pub response: Response,
    pub vin: String,
    pub encrypt: types::Encrypt,
    pub body_len: u16,
    pub body: body::Body,
    pub bcc: u8,
}
