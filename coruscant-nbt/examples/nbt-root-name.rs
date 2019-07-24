use serde::Serialize;

#[derive(Serialize)]
struct MyStruct {}

fn main() -> coruscant_nbt::Result<()> {
    let struct_data = MyStruct {};
    let int_data: i32 = 31415926;
    let s = coruscant_nbt::to_string_transcript(&struct_data)?;
    println!("{}", s);
    let s = coruscant_nbt::to_string_transcript(("Renamed", &struct_data))?;
    println!("{}", s);
    let s = coruscant_nbt::to_string_transcript(&int_data)?;
    println!("{}", s);
    let s = coruscant_nbt::to_string_transcript(("RenamedInt", &int_data))?;
    println!("{}", s);
    Ok(())
}
