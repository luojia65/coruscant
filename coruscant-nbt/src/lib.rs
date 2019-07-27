#![doc(html_root_url = "https://docs.coruscant.rs/coruscant_nbt/")]
// #![warn(missing_docs)]

#[cfg(any(feature = "gzip", feature = "zlib"))]
pub use flate2::Compression;

#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use root::Root;

#[doc(inline)]
pub use ser::{to_string_transcript, to_vec, to_writer, Serializer};
#[cfg(feature = "gzip")]
#[doc(inline)]
pub use {de::from_gzip_reader, ser::to_gzip_writer};
#[cfg(feature = "zlib")]
#[doc(inline)]
pub use {de::from_zlib_reader, ser::to_zlib_writer};

#[doc(inline)]
pub use de::{from_reader, from_slice, Deserializer};

#[doc(inline)]
pub use value::{to_value, Value};

#[doc(inline)]
pub use map::Map;

#[doc(inline)]
pub use to_array::serialize as to_array;

#[macro_use]
mod macros;
mod consts;
pub mod de;
pub mod error;
pub mod map;
mod read;
pub mod root;
pub mod ser;
pub mod value;

mod to_array;
