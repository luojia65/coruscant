#![feature(test)]
extern crate test;
use test::Bencher;

use coruscant_nbt::to_writer;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename = "wrap")]
struct Wrap {
    #[serde(rename = "inner")]
    inner: Inner,
}

#[derive(Serialize)]
struct Inner {
    map: HashMap<&'static str, f32>,
}

#[bench]
fn nbt_ser_simple(b: &mut Bencher) {
    let mut map = HashMap::new();
    map.insert("123", 123.456);
    map.insert("456", 789.012);
    let value = Wrap {
        inner: Inner { map },
    };
    // 44 bytes should be filled
    let mut vec = Vec::with_capacity(128);
    b.iter(|| {
        let _ = to_writer(&mut vec, &value).unwrap();
        vec.clear();
    });
}
