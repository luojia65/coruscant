use coruscant_nbt::{from_slice, from_reader, to_vec, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WrapRef<'a> {
    inner: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct WrapOwned {
    inner: String,
}

fn main() -> Result<()> {
    let value = WrapRef { inner: "hello" };
    let vec = to_vec(&value)?;
    println!("{:?}", vec);

    let ans: WrapRef = from_slice(&vec)?;
    println!("{:?}", ans);

    let read = std::io::Cursor::new(vec);
    let ans: WrapOwned = from_reader(read)?;
    println!("{:?}", ans);
    Ok(())
}
