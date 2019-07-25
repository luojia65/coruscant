//! Mark that a field should be serialized as an NBT array instread of a list of
//! elements.
//!
//! You may use this module by adding `#[serde(with = "as_nbt_array")]` onto fields
//! that need to be serialized into arrays, for example ByteArray instead of a ListTag
//! of bytes. By adding it onto a serde sequence of `i8`, `i32` or `i64`, the serializer
//! guarantees to procceed them into ByteArray, IntArray or LongArray.
//!
//! # Example
//!
//! ```
//! use coruscant_nbt::as_nbt_array;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Wrap<'a> {
//!     list_of_bytes: &'a [i8], // no attribute => ListTag of bytes
//!     #[serde(with = "as_nbt_array")]
//!     byte_array: &'a [i8], // with special attribute => ByteArray
//! }
//!
//! fn main() {
//!     let bytes = &[-1, 2, -3];
//!     let data = Wrap {
//!         list_of_bytes: bytes,
//!         byte_array: bytes,
//!     };
//!
//!     // You may also use other to-functions. For human readability, this example
//!     // prints informal NBT transctiption for reference.
//!     let out = coruscant_nbt::to_string_transcript(&data)
//!             .expect("generate string transcript");
//!
//!     // Should firstly print a ListTag of bytes, and secondly a special ByteArray.
//!     // IntArray and LongArray are also supported.
//!     println!("{}", out);
//! }
//! ```
//!
//! # Notes
//!
//! Although this module seems empty in docs, it actually consists of a doc hidden
//! `serialize` method and a special wrapper struct. This enables serde to serialize
//! marked fields using this method, in which your field are wrapped by the struct
//! and procceeded differently in `coruscant::Serializer`, resulting in switching
//! lists into arrays.

use serde::Serialize;

#[doc(hidden)]
pub fn serialize<'a, T, S>(value: &T, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    T: serde::Serialize,
    S: serde::Serializer,
{
    let value = __WrapAsArray(value);
    value.serialize(serializer)
}

#[doc(hidden)]
#[derive(serde::Serialize)]
#[serde(rename = "$coruscant_nbt::private::__WrapAsArray")]
pub struct __WrapAsArray<T>(pub T);

pub(crate) const TOKEN_AS_ARRAY: &'static str = "$coruscant_nbt::private::__WrapAsArray";
