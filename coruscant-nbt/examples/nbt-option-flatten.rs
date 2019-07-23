use coruscant_nbt::to_string_transcript;
use serde::Serialize;

// there is a (probable) serde bug here!
// https://github.com/serde-rs/serde/issues/1584

#[derive(Serialize)]
#[serde(rename = "book")]
struct Book {
    resolved: Option<i8>,
    #[serde(flatten)]
    extra: Option<Extra>,
}

#[derive(Serialize)]
struct Extra {
    generation: i32,
    author: &'static str,
    title: &'static str,
}

fn main() -> coruscant_nbt::Result<()> {
    let b1 = Book {
        resolved: None,
        extra: None,
    };
    let b2 = Book {
        resolved: Some(1),
        extra: None,
    };
    let e1 = Extra {
        generation: 0,
        author: "luojia65",
        title: "hello",
    };
    let b3 = Book {
        resolved: Some(1),
        extra: Some(e1),
    };

    println!("== Unresolved: does not contain `resolved` key");
    println!("{}", to_string_transcript(&b1)?);

    println!("== Resolved: contains `resolved` key");
    println!("{}", to_string_transcript(&b2)?);

    println!("== With extra: contains flattened `generation` key, etc.");
    println!("{}", to_string_transcript(&b3)?);

    Ok(())
}
