use coruscant_nbt::to_string_transcript;
use serde::Serialize;

#[derive(Serialize)]
struct Book {
    resolved: Option<i8>,
}

fn main() -> coruscant_nbt::Result<()> {
    let b1 = Book { resolved: None };
    let b2 = Book { resolved: Some(1) };

    println!("== Unresolved: should not contain `resolved` key");
    println!("{}", to_string_transcript(&b1)?);

    println!("== Resolved: should contain `resolved` key");
    println!("{}", to_string_transcript(&b2)?);
    Ok(())
}
