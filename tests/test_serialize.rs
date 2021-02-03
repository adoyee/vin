extern crate vin;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use vin::packet::body::Body;
use vin::packet::types::Time;

#[test]
fn test_vehicle_login() {
    let text = "232301fe4c5a595442474257364a3130313431393401001e120a1e14233600fd383938363034303231303137303031373937373901005c" ;
    vin::packet::parser::pares_hex(text).unwrap();
}
