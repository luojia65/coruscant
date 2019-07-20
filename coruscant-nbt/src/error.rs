use core::fmt;
use std::io;

use serde::{ser, de};

// derive nothing here by now
pub struct Error {
    err: Box<ErrorImpl>,
}

pub type Result<T> = core::result::Result<T, Error>;

struct ErrorImpl {
    code: ErrorCode,
    line: usize, // line == 0: line and column is not necessary
    column: usize,
}

pub(crate) enum ErrorCode {
    Message(Box<str>),
    Io(io::Error),
    UnsupportedType,
    UnsupportedListInnerType,
    InvalidStringLength,
    KeyMustBeAString,
    SequenceSizeUnknown,
    SequenceDifferentType,
}

impl Error {
    pub(crate) fn syntax(code: ErrorCode, line: usize, column: usize) -> Self {
        Self::from_inner(code, line, column)
    }

    pub(crate) fn io(error: io::Error) -> Self {
        let code = ErrorCode::Io(error);
        Self::from_inner(code, 0, 0)
    }

    #[inline]
    fn from_inner(code: ErrorCode, line: usize, column: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl { code, line, column }),
        }
    }
}

impl fmt::Debug for Error {
    // todo: improve error message format
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error at line {}, column {}",
            self.err.line, self.err.column
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&*self.err, f)
    }
}

impl fmt::Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.line == 0 {
            fmt::Display::fmt(&self.code, f)
        } else {
            // todo: improve message format
            write!(
                f,
                "{} at line {} column {}",
                self.code, self.line, self.column
            )
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ErrorCode::Message(ref msg) => f.write_str(msg),
            ErrorCode::Io(ref err) => fmt::Display::fmt(err, f),
            ErrorCode::UnsupportedType => f.write_str("unsupported type"),
            ErrorCode::UnsupportedListInnerType => f.write_str("unsupported list-wrapped type"),
            ErrorCode::InvalidStringLength => f.write_str("invalid string length"),
            ErrorCode::KeyMustBeAString => f.write_str("key must be a string"),
            ErrorCode::SequenceSizeUnknown => f.write_str("size of sequence is unknown"),
            ErrorCode::SequenceDifferentType => {
                f.write_str("elements of one sequence do not have the same type")
            } // exhaustive
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        let string = msg.to_string();
        let code = ErrorCode::Message(string.into_boxed_str());
        Self::from_inner(code, 0, 0)
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        let string = msg.to_string();
        let code = ErrorCode::Message(string.into_boxed_str());
        Self::from_inner(code, 0, 0)
    }
}
