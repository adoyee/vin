use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::packet::types;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct VehicleLogin {
//     pub at: types::Time,
//     pub sn: u16,
//     pub iccid: String,
//     pub subsys_num: u8,
//     pub subsys_len: u8,
//     pub subsys_sn: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum Body {
    VehicleLogin {
        at: types::Time,
        sn: u16,
        iccid: String,
        subsys_num: u8,
        subsys_len: u8,
        subsys_sn: String,
    },
}
