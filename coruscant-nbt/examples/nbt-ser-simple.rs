use coruscant_nbt::{to_string_transcript, Result};
use serde::Serialize;

// #[derive(Serialize)]
// struct Wrap<'a> {
//     inner: &'a str,
// }

// fn main() -> Result<()> {
//     let value = Wrap { inner: "Spicy" };
//     let ans = to_vec(&value)?;
//     println!("{:?}", ans);
//     Ok(())
// }

#[derive(Serialize)]
#[serde(rename = "wrap")]
struct Wrap {
    #[serde(rename = "inner")]
    inner: Inner,
}

#[derive(Serialize)]
struct Inner {
    a: i8
}

fn main() -> Result<()> {
    let value = Wrap { inner: Inner { a: -15 } };
    let ans = to_string_transcript(("Outer", &value))?;
    println!("{}", ans);
    Ok(())
}
