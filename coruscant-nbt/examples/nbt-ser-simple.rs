use coruscant_nbt::{to_string_transcript, Result};
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

fn main() -> Result<()> {
    let mut map = HashMap::new();
    map.insert("123", 123.456);
    map.insert("456", 789.012);
    let value = Wrap { inner: Inner { map } };
    let ans = to_string_transcript(("Outer", &value))?;
    println!("{}", ans);
    Ok(())
}
