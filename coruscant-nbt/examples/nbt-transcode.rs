use std::io::{Cursor, Write};

fn main() {
    let str_input = "{\"testByte\":127}";
    let vec_output = Vec::new();
    let write = Cursor::new(vec_output);

    let mut deserializer = serde_json::Deserializer::from_str(str_input);
    let mut serializer = coruscant_nbt::Serializer::transcript(write, "");

    serde_transcode::transcode(&mut deserializer, &mut serializer)
        .expect("serde transcode");
    
    let mut write = serializer.into_inner();
    write.flush().unwrap();
    let string_output = String::from_utf8(write.into_inner()).unwrap();

    println!("{}", string_output);
}
