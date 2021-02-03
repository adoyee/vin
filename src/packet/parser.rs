use std::io::Read;

use crate::packet::body;
use crate::packet::error::{Error, Result};
use crate::packet::types;
use crate::packet::{Command, Header, Packet};
use crate::serde;

pub fn parse_header<R: Read>(de: &mut serde::Deserializer<R>) -> Result<Header> {
    let h: Header = ::serde::Deserialize::deserialize(de).unwrap();
    Ok(h)
}

pub fn pares_hex(text: &str) -> Result<()> {
    let data = hex::decode(text)?;
    parse_bytes(data.as_slice())
}

pub fn parse_bytes(data: &[u8]) -> Result<()> {
    let mut de = serde::Deserializer::new(data);
    let header: Header = ::serde::Deserialize::deserialize(&mut de)?;
    println!("{:?}", header);
    let body = body::VehicleLogin::deserialize(&mut de)?;
    println!("{:?}", body);
    Ok(())
}
