pub use error::{Error, Result};
pub use root::Root;

#[cfg(feature = "gzip")]
#[doc(inline)]
pub use ser::to_gzip_writer;
#[cfg(feature = "zlib")]
#[doc(inline)]
pub use ser::to_zlib_writer;
#[doc(inline)]
pub use ser::{to_string_transcript, to_vec, to_writer};

pub use value::{to_value, Value};

#[macro_use]
mod macros;
mod consts;
pub mod de;
pub mod error;
mod map;
pub mod root;
pub mod ser;
pub mod value;
