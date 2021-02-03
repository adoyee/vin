use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use error::{Error, Result};
pub use types::Encrypt;
pub use types::Iccid;
pub use types::Time;
pub use types::Vin;

pub mod body;
pub mod error;
pub mod parser;
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
    DupVin,
    Command = 0xFE,
}

#[derive(Debug)]
pub struct Packet {
    pub begin: u16,
    pub command: Command,
    pub response: Response,
    pub vin: Vin,
    pub encrypt: Encrypt,
    pub body_len: u16,
    pub body: Box<dyn body::Body>,
    pub bcc: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyCommand(u8);

#[derive(Debug, Deserialize)]
pub struct Header {
    pub begin: u16,
    pub command: MyCommand,
    pub response: Response,
    pub vin: Vin,
    pub encrypt: Encrypt,
    pub body_len: u16,
}
