// ByteArray, IntArray and LongArray serialization
use coruscant_nbt::to_array;
use serde::Serialize;

#[derive(Serialize)]
struct Wrap<'a> {
    list_of_bytes: &'a [i8], // no attributes, do it as ListTag of bytes .
    #[serde(serialize_with = "to_array")] // special attribute used, regard as ByteArray.
    byte_array: &'a [i8],
    #[serde(serialize_with = "to_array")] // same attribute, but for seq of i32 values,
    int_array: &'a [i32], // we serialize it as IntArray instead.
    #[serde(serialize_with = "to_array")]
    long_array: &'a [i64], // for the same reason, LongArray here.
}

fn main() {
    let bytes = &[-1, 2, -3];
    let data = Wrap {
        list_of_bytes: bytes,
        byte_array: bytes,
        int_array: &[19132, -25565],
        long_array: &[-31415926, 299792458],
    };

    // You may also use other to-functions. For human readability, this example
    // prints informal NBT transctiption for reference.
    let out = coruscant_nbt::to_string_transcript(&data).expect("generate string transcript");

    // Should firstly print a ListTag of bytes, and secondly a special ByteArray.
    // IntArray and LongArray are also supported.
    println!("{}", out);
}
