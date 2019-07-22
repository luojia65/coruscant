use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type")]
enum Message {
    Request { id: &'static str, method: &'static str, params: i8 },
    Response { id: &'static str, result: i8 },
}

fn main() {
    let data = Message::Request { id: "...", method: "...", params: 1 };
    let out = coruscant_nbt::to_string_transcript(&data).unwrap();
    println!("{}", out);

    let data = Message::Response { id: "...", result: 2 };
    let out = coruscant_nbt::to_string_transcript(&data).unwrap();
    println!("{}", out);
}
