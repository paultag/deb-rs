#![no_main]

use deb::control::de::from_reader;
use libfuzzer_sys::fuzz_target;
use serde::Deserialize;
use std::io::{BufReader, Cursor};

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct TestStruct {
    key: String,
    ni1: i8,
    ni16: i16,
    ni32: i32,
    ni64: i64,
    ni128: i128,
    nu1: u8,
    nu16: u16,
    nu32: u32,
    nu64: u64,
    nu128: u128,
    boolean: bool,

    #[serde(flatten)]
    flattend_inside: FlattenedInside,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct FlattenedInside {
    test_key: String,
}

fuzz_target!(|data: &str| {
    let _ = from_reader::<TestStruct, _>(&mut BufReader::new(Cursor::new(data)));
});
