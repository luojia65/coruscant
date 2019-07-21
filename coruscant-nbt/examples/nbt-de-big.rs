use coruscant_nbt::{from_reader, to_vec, Result};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct TestStruct {
//     #[serde(rename = "byteTest")]
//     byte_test: i8,
// }

fn main() -> Result<()> {
    let vec = to_vec(&127i8)?;
    println!("{:?}", vec);
    let ans: bool = from_reader(std::io::Cursor::new(vec))?;
    println!("{}", ans);
    Ok(())
}
