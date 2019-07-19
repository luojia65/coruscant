pub use error::{Error, Result};
pub use root::Root;
pub use ser::{to_string_transcript, to_vec, to_writer};
pub use value::{to_value, Value};

#[cfg(gzip)]
pub use ser::to_gzip_writer;
#[cfg(zlib)]
pub use ser::to_zlib_writer;

#[macro_use]
mod macros;
mod consts;
pub mod error;
mod map;
pub mod root;
pub mod ser;
pub mod value;
