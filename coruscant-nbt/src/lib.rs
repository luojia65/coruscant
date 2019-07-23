#[cfg(any(feature = "gzip", feature = "zlib"))]
pub use flate2::Compression;

#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use root::Root;

#[cfg(feature = "gzip")]
#[doc(inline)]
pub use ser::to_gzip_writer;
#[cfg(feature = "zlib")]
#[doc(inline)]
pub use ser::to_zlib_writer;
#[doc(inline)]
pub use ser::{to_string_transcript, to_vec, to_writer, Serializer};

#[doc(inline)]
pub use de::{from_reader, Deserializer};

#[doc(inline)]
pub use value::{to_value, Value};

#[doc(inline)]
pub use map::Map;

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

pub mod as_nbt_array;
