extern crate vin;

use serde::Serialize;

use vin::packet::body::Body;
use vin::packet::types::Time;

#[derive(Debug, Serialize)]
struct StructA {
    a: u8,
    b: u8,
    c: String,
}

#[test]
fn serialize_struct_a() {
    let body = Body::VehicleLogin {
        at: Time {
            year: 21,
            month: 1,
            day: 2,
            hour: 1,
            minute: 1,
            second: 1,
        },
        sn: 1234,
        iccid: String::from("1234567ICCID"),
        subsys_num: 0,
        subsys_len: 0,
        subsys_sn: String::from("subsys-sn"),
    };

    let packet = vin::packet::Packet {
        begin: 0x2323,
        command: vin::packet::Command::VehicleLogin,
        response: vin::packet::Response::Success,
        vin: String::from("vin"),
        encrypt: vin::packet::types::Encrypt::Rsa,
        body_len: 123,
        body,
        bcc: 0xff,
    };

    let msg = vin::to_string(&packet).unwrap();
    println!("{}", msg)
}
// 2323010076696E0000000003007B15010201010104D23132333435363749434349440000007375627379732D736E00FF
