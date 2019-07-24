use coruscant_nbt::{from_reader, to_vec, Result};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug)]
struct TestStruct {
    bool_test: bool,
    #[serde(rename = "byteTest")]
    byte_test: i8,
    #[serde(rename = "shortTest")]
    short_test: i16,
    #[serde(rename = "intTest")]
    int_test: i32,
    #[serde(rename = "nested compound test")]
    nested: Nested,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nested {
    egg: Food,
    ham: Food,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Food {
    name: String,
    value: f32,
}

fn main() -> Result<()> {
    let data = TestStruct { 
        nested: Nested {
            egg: Food {
                name: "Eggbert".to_owned(),
                value: 0.5,
            },
            ham: Food {
                name: "Hampus".to_owned(),
                value: 0.75,
            },
        },
        bool_test: true,
        byte_test: 127,
        short_test: 19132,
        int_test: 0, 
    };
    let vec = to_vec(&data)?;
    println!("{:?}", vec);
    let read = Cursor::new(vec);
    let ans: TestStruct = from_reader(read)?;
    println!("{:?}", ans);
    Ok(())
}
