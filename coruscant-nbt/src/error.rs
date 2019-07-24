//! When serializing or deserializing NBT goes wrong.

use core::fmt;
use std::io;

use serde::{de, ser};

/// This type represents all possible errors that can occur when serializing or
/// deserializing NBT data.
pub struct Error {
    err: Box<ErrorImpl>,
}

/// Alias for a `Result` with the error type `coruscant_nbt::Error`.
pub type Result<T> = core::result::Result<T, Error>;

struct ErrorImpl {
    code: ErrorCode,
    index: usize, // index == 0: index is not necessary
}

pub(crate) enum ErrorCode {
    Message(Box<str>),
    Io(io::Error),
    UnsupportedType,
    UnsupportedListInnerType,
    UnsupportedArrayType,
    UnsupportedArrayInnerType,
    InvalidStringLength,
    KeyMustBeAString,
    SequenceSizeUnknown,
    ListDifferentType,
    ArrayDifferentType,
    InvalidBoolByte(i8),
    InvalidUtf8String,
    TypeIdMismatch(u8, u8),
    TypeIdInvalid(u8),
    InvalidLength(i16),
    SliceUnexpectedEof,
}

impl Error {
    pub(crate) fn syntax(code: ErrorCode, index: usize) -> Self {
        Self::from_inner(code, index)
    }

    pub(crate) fn io(error: io::Error) -> Self {
        let code = ErrorCode::Io(error);
        Self::from_inner(code, 0)
    }

    pub(crate) fn io_at(error: io::Error, index: usize) -> Self {
        let code = ErrorCode::Io(error);
        Self::from_inner(code, index)
    }

    pub(crate) fn utf8_at(index: usize) -> Self {
        let code = ErrorCode::InvalidUtf8String;
        Self::from_inner(code, index)
    }

    pub(crate) fn mismatch_at(mismatch: u8, expected: u8, index: usize) -> Self {
        let code = ErrorCode::TypeIdMismatch(mismatch, expected);
        Self::from_inner(code, index)
    }

    pub(crate) fn invalid_id_at(invalid_id: u8, index: usize) -> Self {
        let code = ErrorCode::TypeIdInvalid(invalid_id);
        Self::from_inner(code, index)
    }

    pub(crate) fn bool_at(invalid: i8, index: usize) -> Self {
        let code = ErrorCode::InvalidBoolByte(invalid);
        Self::from_inner(code, index)
    }

    pub(crate) fn invalid_len_at(invalid: i16, index: usize) -> Self {
        let code = ErrorCode::InvalidLength(invalid);
        Self::from_inner(code, index)
    }

    pub(crate) fn slice_eof() -> Self {
        let code = ErrorCode::SliceUnexpectedEof;
        Self::from_inner(code, 0)
    }

    #[inline]
    fn from_inner(code: ErrorCode, index: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl { code, index }),
        }
    }
}

impl fmt::Debug for Error {
    // todo: improve error message format
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.err.index == 0 {
            fmt::Display::fmt(&self.err.code, f)
        } else {
            write!(f, "(NBT input:{}) {}", self.err.index, self.err.code)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&*self.err, f)
    }
}

impl fmt::Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.index == 0 {
            fmt::Display::fmt(&self.code, f)
        } else {
            // todo: improve message format
            write!(f, "(NBT input:{}) {}", self.index, self.code)
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ErrorCode::Message(ref msg) => f.write_str(msg),
            ErrorCode::Io(ref err) => fmt::Display::fmt(err, f),
            ErrorCode::UnsupportedType => f.write_str("unsupported NBT type"),
            ErrorCode::UnsupportedListInnerType => f.write_str("unsupported NBT list-wrapped type"),
            ErrorCode::UnsupportedArrayType => {
                f.write_str("unsupported type for NBT arrays; a sequence is required")
            }
            ErrorCode::UnsupportedArrayInnerType => f.write_str("unsupported NBT array inner type"),
            ErrorCode::InvalidStringLength => f.write_str("invalid NBT string length"),
            ErrorCode::KeyMustBeAString => f.write_str("NBT key must be a string"),
            ErrorCode::SequenceSizeUnknown => f.write_str("size of NBT sequence is unknown"),
            ErrorCode::ListDifferentType => {
                f.write_str("elements of one NBT list do not have the same type")
            }
            ErrorCode::ArrayDifferentType => {
                f.write_str("elements of one NBT array do not have the same type")
            }
            ErrorCode::InvalidBoolByte(invalid) => f.write_fmt(format_args!(
                "invalid NBT boolean byte {} (0x{:02X}), 0 or 1 expected",
                invalid, invalid
            )),
            ErrorCode::InvalidUtf8String => f.write_str("invalid utf-8 NBT string"),
            ErrorCode::TypeIdMismatch(invalid, expected) => f.write_fmt(format_args!(
                "mismatched NBT type id {} (0x{:02X}), expected {} (0x{:02X})",
                invalid, invalid, expected, expected
            )),
            ErrorCode::TypeIdInvalid(invalid) => f.write_fmt(format_args!(
                "invalid type id {} (0x{:02X})",
                invalid, invalid
            )),
            ErrorCode::InvalidLength(invalid) => f.write_fmt(format_args!(
                "invalid length {} (0x{:04X}); length must be positive for NBT",
                invalid, invalid
            )),
            ErrorCode::SliceUnexpectedEof => {
                f.write_str("unexpected EOF when reading NBT source slice")
            }
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        let string = msg.to_string();
        let code = ErrorCode::Message(string.into_boxed_str());
        Self::from_inner(code, 0)
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        let string = msg.to_string();
        let code = ErrorCode::Message(string.into_boxed_str());
        Self::from_inner(code, 0)
    }
}

impl From<io::Error> for Error {
    fn from(src: io::Error) -> Error {
        Error::io(src)
    }
}
