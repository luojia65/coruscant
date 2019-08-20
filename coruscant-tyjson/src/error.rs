use core::fmt;

pub struct Error {
    repr: Repr,
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {

}
impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Error {
        Error {
            repr: Repr::Simple(kind)
        }
    }
}

enum Repr {
    Message(Box<str>),
    Simple(ErrorKind),
}

impl fmt::Debug for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Repr::Message(msg) => f.write_str(msg),
            Repr::Simple(kind) => f.debug_tuple("kind").field(kind).finish(),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.repr {
            Repr::Message(msg) => write!(f, "{}", msg),
            Repr::Simple(kind) => write!(f, "{:?}", kind),
        }
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error { repr: Repr::Message(msg.to_string().into_boxed_str()) }
    }
}