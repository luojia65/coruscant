use core::fmt;
use std::io;

use serde::{de, ser};

// derive nothing here by now
pub struct Error {
    err: Box<ErrorImpl>,
}

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
    InvalidBoolByte,
}

impl Error {
    pub(crate) fn syntax(code: ErrorCode, index: usize) -> Self {
        Self::from_inner(code, index)
    }

    pub(crate) fn io(error: io::Error) -> Self {
        let code = ErrorCode::Io(error);
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
            write!(f, "{} at index {}", self.err.code, self.err.index)
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
            write!(f, "{} at index {}", self.code, self.index)
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ErrorCode::Message(ref msg) => f.write_str(msg),
            ErrorCode::Io(ref err) => fmt::Display::fmt(err, f),
            ErrorCode::UnsupportedType => f.write_str("unsupported nbt type"),
            ErrorCode::UnsupportedListInnerType => f.write_str("unsupported list-wrapped type"),
            ErrorCode::UnsupportedArrayType => {
                f.write_str("unsupported type for arrays; a sequence is required")
            }
            ErrorCode::UnsupportedArrayInnerType => f.write_str("unsupported array inner type"),
            ErrorCode::InvalidStringLength => f.write_str("invalid string length"),
            ErrorCode::KeyMustBeAString => f.write_str("key must be a string"),
            ErrorCode::SequenceSizeUnknown => f.write_str("size of sequence is unknown"),
            ErrorCode::ListDifferentType => {
                f.write_str("elements of one list do not have the same type")
            }
            ErrorCode::ArrayDifferentType => {
                f.write_str("elements of one array do not have the same type")
            }
            ErrorCode::InvalidBoolByte => f.write_str("invalid boolean byte"),
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
