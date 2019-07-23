// ByteArray, IntArray and LongArray serialization
use serde::Serialize;
use coruscant_nbt::as_nbt_array;

#[derive(Serialize)]
struct Wrap<'a> {
    list_of_bytes: &'a [i8],
    #[serde(with = "as_nbt_array")]
    byte_array: &'a [i8],
    #[serde(with = "as_nbt_array")]
    int_array: &'a [i32],
    #[serde(with = "as_nbt_array")]
    long_array: &'a [i64],
}

fn main() {
    let bytes = &[1, 2, 3];
    let data = Wrap { 
        list_of_bytes: bytes,
        byte_array: bytes,
        int_array: &[19132, 25565],
        long_array: &[314159364, 299792458],
    };
    let out = coruscant_nbt::to_string_transcript(&data).unwrap();

    // Should firstly print a ListTag of bytes, and secondly a special ByteArray.
    // IntArray and LongArray are also supported.
    println!("{}", out);
}
