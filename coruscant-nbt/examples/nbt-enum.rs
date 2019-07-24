// 这个example暂时跑不了，过会儿修

// // Example taken from https://serde.rs/json.html
// use serde::Serialize;
// use coruscant_nbt::to_string_transcript;

// #[derive(Serialize)]
// struct Wrap {
//     inner: E,
// }

// #[derive(Serialize)]
// enum E {
//     W { a: i32, b: i32 },
//     X(i32, i32),
//     Y(i32),
//     Z,
// }

// /*
// Compound ''
//     Compound 'inner'
//         Int a 0
//         Int b 0
//     EndCompound
// EndCompound

// Compound ''
//     Int inner 0
// EndCompound
//  */
// fn main () -> coruscant_nbt::Result<()> {
//     let w = E::W { a: 0, b: 0 };
//     let x = E::X(0, 0);
//     let y = E::Y(0);
//     let z = E::Z;
//     let (w, x, y, z) = (
//         Wrap { inner: w }, Wrap { inner: x },
//         Wrap { inner: y }, Wrap { inner: z }
//     );
//     println!("w => {}", to_string_transcript(&w)?);
//     // println!("x => {}", to_string_transcript(&x)?);
//     println!("y => {}", to_string_transcript(&y)?);
//     println!("z => {}", to_string_transcript(&z)?);
//     Ok(())
// }
fn main () {}
