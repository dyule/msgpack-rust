#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

#![cfg(feature = "serde_macros")]

extern crate serde;
extern crate rmp;
extern crate rmp_serde;

use serde::Serialize;
use rmp_serde::Serializer;

#[test]
fn pass_struct() {
    let mut buf = [0x00; 16];

    #[derive(Serialize)]
    struct Decoded { id: u32, value: u32 }

    let val = Decoded { id: 42, value: 100500 };
    val.serialize(&mut Serializer::new(&mut &mut buf[..])).ok().unwrap();
    let out = [
        0x82, // Map Size (2)
        0xa2, 0x69, 0x64, // id
        0x2a, // 42
        0xa5, 0x76, 0x61, 0x6c,  0x75, 0x65, // value
        0xce, 0x0, 0x1, 0x88, 0x94 // 100500
    ];
    assert_eq!(out, buf);
}

#[test]
fn pass_empty_struct() {
    // Must be encoded as [].
    let mut buf = vec![];

    #[derive(Serialize)]
    struct Struct;

    let val = Struct;
    val.serialize(&mut Serializer::new(&mut buf)).ok().unwrap();

    assert_eq!(vec![0x80], buf);
}

#[test]
fn pass_struct_map() {
    #[derive(Debug, PartialEq, Serialize)]
    struct Custom<'a> {
        et: &'a str,
        le: u8,
        shit: u8,
    }

    let mut buf = [0x00; 20];

    let val = Custom { et: "voila", le: 0, shit: 1 };
    val.serialize(&mut Serializer::new(&mut &mut buf[..])).ok().unwrap();

    let out = [
        0x83, // 3 (size)
        0xa2, 0x65, 0x74, // "et"
        0xa5, 0x76, 0x6f, 0x69, 0x6c, 0x61, // "voila"
        0xa2, 0x6c, 0x65, // "le"
        0x00, // 0
        0xa4, 0x73, 0x68, 0x69, 0x74, // "shit"
        0x01, // 1
    ];
    assert_eq!(out, buf);
}

#[test]
fn pass_enum() {
    // We encode enum types as [id, [args...]].

    #[allow(unused)]
    #[derive(Debug, PartialEq, Serialize)]
    enum Custom {
        First,
        Second,
    }

    let mut buf = [0x00; 3];

    let val = Custom::Second;
    val.serialize(&mut Serializer::new(&mut &mut buf[..])).ok().unwrap();

    let out = [0x92, 0x01, 0x90];
    assert_eq!(out, buf);
}

#[test]
fn pass_tuple_enum_with_arg() {
    #[allow(unused)]
    #[derive(Debug, PartialEq, Serialize)]
    enum Custom {
        First,
        Second(u32),
    }

    let mut buf = [0x00; 4];

    let val = Custom::Second(42);
    val.serialize(&mut Serializer::new(&mut &mut buf[..])).ok().unwrap();

    let out = [0x92, 0x01, 0x91, 0x2a];
    assert_eq!(out, buf);
}

#[test]
fn encode_struct_with_string_using_vec() {
    #[derive(Debug, PartialEq, Serialize)]
    struct Custom {
        data: String,
    }

    let mut buf = Vec::new();

    let val = Custom { data: "le message".to_string() };
    val.serialize(&mut Serializer::new(&mut buf)).ok().unwrap();

    let out = vec![129, 164, 100, 97, 116, 97, 170, 108, 101, 32, 109, 101, 115, 115, 97, 103, 101];
    assert_eq!(out, buf);
}

#[test]
fn pass_struct_enum() {
    #[allow(unused)]
    #[derive(Debug, PartialEq, Serialize)]
    enum Custom {
        First,
        Second(u32),
        Third{data:u32,val:u32}
    }

    let mut buf = [0x00; 14];

    let val = Custom::Third{data:42, val:105};
    val.serialize(&mut Serializer::new(&mut &mut buf[..])).ok().unwrap();

    let out = [
        0x92, // Variant length (2)
        0x02, // Variant ID (2)
        0x82, // Map Size (2)
        0xa4, 0x64, 0x61, 0x74, 0x61, // data
        0x2a, // 42
        0xa3, 0x76, 0x61, 0x6c, // val
        0x69 // 105
    ];
    assert_eq!(out, buf);
}
