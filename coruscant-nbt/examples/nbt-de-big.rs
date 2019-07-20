use coruscant_nbt::{from_reader, to_vec, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TestStruct {
    #[serde(rename = "byteTest")]
    byte_test: i8,
}

fn main() -> Result<()> {
    let vec = to_vec(&TestStruct { byte_test: 127 })?;
    let ans: TestStruct = from_reader(std::io::Cursor::new(vec))?;
    println!("{}", ans.byte_test);
    Ok(())
}
