// 20 lines of code that converts your JSON string into NBT format.
//
// This piece of code performs the transcode through `serde_transcode` crate.
// It prints NBT transcript for human readbility using `Serializer::transcript`;
// if you want NBT bytes, consider using `Serializer::binary` instead.
//
// Due to the loss of number accuracy in JSON, this program only support limited
// input data type from JSON, for example objects and strings. However, if we do
// contrarily from NBT to JSON, full transcode support is possible as the former
// does record all the data types for numbers.

use std::io::{Cursor, Write};

fn main() {
    let str_input = "{\"nick\":\"luojia65\", 
        \"school\":{\"name\":\"hust\",\"985\":true}}";
    let vec_output = Vec::new();
    let write = Cursor::new(vec_output);

    let mut deserializer = serde_json::Deserializer::from_str(str_input);
    let mut serializer = coruscant_nbt::Serializer::transcript(write, "person");

    serde_transcode::transcode(&mut deserializer, &mut serializer).expect("serde transcode");

    let mut write = serializer.into_inner();
    write.flush().unwrap(); // Cursor flush never fails. If other `io::Write`
                            // output is used, handle flush result properly.
    let string_output = String::from_utf8(write.into_inner()).unwrap();
    // Transcript byte output from `coruscant_nbt` is always valid for utf8.
    println!("{}", string_output);
}
