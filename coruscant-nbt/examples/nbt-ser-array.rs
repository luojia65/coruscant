// ByteArray, IntArray and LongArray serialization
use serde::Serialize;
use coruscant_nbt::ser::as_nbt_array;

#[derive(Serialize)]
struct Wrap<'a> {
    #[serde(with = "as_nbt_array")]
    byte_array: &'a [i8]
}

fn main() {
    let byte_array = &[1, 2, 3];
    let data = Wrap { byte_array };
    let out = coruscant_nbt::to_string_transcript(&data).unwrap();
    println!("{}", out)
}
