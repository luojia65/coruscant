pub mod de;
pub mod error;
mod mux;

pub use de::Deserializer;
pub use error::{Error, Result, ErrorKind};
