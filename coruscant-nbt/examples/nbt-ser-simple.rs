use coruscant_nbt::{to_vec, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Wrap<'a> {
    inner: &'a str,
}

fn main() -> Result<()> {
    let value = Wrap { inner: "Spicy" };
    let ans = to_vec(&value)?;
    println!("{:?}", ans);
    Ok(())
}
