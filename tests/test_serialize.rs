extern crate vin;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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

#[test]
fn de_enum_8() {
    #[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
    #[repr(u16)]
    enum EnumA {
        A = 1,
        B = 2,
    }
    let a = EnumA::B;
    let ser = vin::to_string(&a).unwrap();
    let de: EnumA = vin::from_str(ser.as_str()).unwrap();
    assert_eq!(de, a)
}

#[test]
fn de_string() {
    let msg = String::from("hello world 你们好");
    let ser = vin::to_string(&msg).unwrap();
    let de: String = vin::from_str(ser.as_str()).unwrap();
    assert_eq!(msg, de)
}

#[test]
fn test_gbk_string() {
    #[derive(Debug, Eq, PartialEq)]
    struct Opt {}
    impl vin::serde::gbk::Options for Opt {
        const LENGTH: usize = 20;
    }
    type S20 = vin::serde::gbk::GBKString<Opt>;

    #[derive(Serialize, Deserialize, Debug)]
    struct MyStruct {
        s20: S20,
    }

    let s20 = MyStruct {
        s20: S20::from(String::from("hello world")),
    };
    let ser = vin::to_string(&s20).unwrap();
    println!("{:?}", ser);
    let der: MyStruct = vin::from_str(ser.as_str()).unwrap();
    println!("{:?}", der);
}
