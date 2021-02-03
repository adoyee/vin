use std::fmt::Debug;
use std::io::Read;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::packet::error::{Error, Result};
use crate::packet::{types, Time};
use crate::serde::{Deserializer, Serializer};

pub trait Body: Debug {}

#[derive(Debug)]
pub struct VehicleLogin {
    pub at: types::Time,
    pub sn: u16,
    pub iccid: types::Iccid,
    pub subsys_num: u8,
    pub subsys_len: u8,
    pub subsys_sn: Vec<u8>,
}

impl Body for VehicleLogin {}

impl VehicleLogin {
    pub fn deserialize<R: Read>(de: &mut Deserializer<R>) -> Result<Self> {
        let mut subsys_sn: Vec<u8> = Vec::new();
        let at: types::Time = serde::Deserialize::deserialize(&mut *de)?;
        let sn = de.deserialize_u16()?;
        let iccid: types::Iccid = serde::Deserialize::deserialize(&mut *de)?;
        let subsys_num = de.deserialize_u8()?;
        let subsys_len = de.deserialize_u8()?;
        let len = (subsys_num * subsys_len) as usize;
        if len > 0 {
            subsys_sn = de.read_bytes(len)?;
        }
        Ok(Self {
            at,
            sn,
            iccid,
            subsys_num,
            subsys_len,
            subsys_sn,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct VehicleLogout {
    pub at: Time,
    pub sn: u16,
}

impl VehicleLogout {
    pub fn deserialize<R: Read>(de: &mut Deserializer<R>) -> Result<Self> {
        ::serde::Deserialize::deserialize(&mut *de).map_err(Error::from)
    }
}
