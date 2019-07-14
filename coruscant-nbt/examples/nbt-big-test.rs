use coruscant_nbt::{to_string_transcript, Result};
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
    // list_test
    // list_compound_test
    // byte_array_test
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

fn main() -> Result<()> {
    let value = TestStruct {
        nested: Nested {
            egg: Food { name: "Eggbert", value: 0.5 },
            ham: Food { name: "Hampus", value: 0.75 },
        },
        int_test: 2147483647,
        byte_test: 127,
        string_test: "HELLO WORLD THIS IS A TEST STRING!",
        // list_test
        double_test: 0.49312871321823148,
        float_test: 0.49823147058486938,
        long_test: 9223372036854775807,
        // list_compound_test
        // byte_array_test
        short_test: 32767,
    };
    println!("{}", to_string_transcript(("Level", &value))?);
    Ok(())
}