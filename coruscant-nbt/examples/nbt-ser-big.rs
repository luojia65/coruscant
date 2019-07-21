/// Test case taken from https://wiki.vg/NBT
use coruscant_nbt::{to_string_transcript, to_vec, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct TestStruct {
    #[serde(rename = "byteTest")]
    byte_test: i8,
    #[serde(rename = "shortTest")]
    short_test: i16,
    #[serde(rename = "intTest")]
    int_test: i32,
    #[serde(rename = "longTest")]
    long_test: i64,
    #[serde(rename = "floatTest")]
    float_test: f32,
    #[serde(rename = "doubleTest")]
    double_test: f64,
    #[serde(rename = "stringTest")]
    string_test: &'static str,
    #[serde(rename = "listTest (long)")]
    list_long_test: [i64; 5],
    #[serde(rename = "listTest (compound)")]
    list_compound_test: Vec<NestedCompound>,
    #[serde(
        rename = "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))"
    )]
    byte_array_test: Box<[i8]>,
    #[serde(rename = "nested compound test")]
    nested: Nested,
}

#[derive(Serialize)]
pub struct Nested {
    egg: Food,
    ham: Food,
}

#[derive(Serialize)]
pub struct Food {
    name: &'static str,
    value: f32,
}

#[derive(Serialize)]
pub struct NestedCompound {
    #[serde(rename = "created-on")]
    created_on: i64,
    name: &'static str,
}

fn main() -> Result<()> {
    let mut byte_array_test = Vec::new();
    for i in 0i32..1000 {
        let value = (i * i * 255 + i * 7) % 100;
        byte_array_test.push(value as i8)
    }
    let byte_array_test = byte_array_test.into_boxed_slice();
    let value = TestStruct {
        nested: Nested {
            egg: Food {
                name: "Eggbert",
                value: 0.5,
            },
            ham: Food {
                name: "Hampus",
                value: 0.75,
            },
        },
        byte_test: 127,
        short_test: 32767,
        int_test: 2147483647,
        long_test: 9223372036854775807,
        double_test: 0.49312871321823148,
        float_test: 0.49823147058486938,
        string_test: "HELLO WORLD THIS IS A TEST STRING!",
        list_long_test: [11, 12, 13, 14, 15],
        list_compound_test: vec![
            NestedCompound {
                created_on: 1264099775885,
                name: "Compound tag #0",
            },
            NestedCompound {
                created_on: 1264099775885,
                name: "Compound tag #1",
            },
        ],
        byte_array_test,
    };
    println!("{}", to_vec(&value)?.len());
    println!("{}", to_string_transcript(("Level", &value))?);
    Ok(())
}
