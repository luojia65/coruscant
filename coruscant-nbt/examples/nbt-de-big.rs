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
}

fn main() -> Result<()> {
    let data = TestStruct { 
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
