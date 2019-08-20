pub struct Error {
    repr: Repr,
}

pub type Result<T> = core::result::Result<T, Error>;

pub enum ErrorKind {

}

enum Repr {
    Message(Box<str>),
    Simple(ErrorKind),
}

