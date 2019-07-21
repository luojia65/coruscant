use crate::consts;
use crate::error::{Error, ErrorCode, Result};
use crate::read;
use serde::de;
use std::io;

use serde::forward_to_deserialize_any;

pub fn from_reader<'de, R, T>(read: R) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let read = read::IoRead::new(read);
    let mut de = Deserializer::new(read);
    T::deserialize(&mut de)
}

pub struct Deserializer<R> {
    read: R,
}

impl<'de, R> Deserializer<R>
where
    R: read::Read<'de>,
{
    pub fn new(read: R) -> Self {
        Deserializer { read }
    }

    pub fn into_inner(self) -> R {
        self.read
    }
}

#[inline]
fn invalid_bool_byte() -> Error {
    Error::syntax(ErrorCode::InvalidBoolByte, 0)
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: read::Read<'de>,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.read.peek_u8()? {
            Some(consts::TYPE_ID_BYTE) => self.deserialize_i8(visitor),
            _ => unimplemented!(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.read.read_byte_inner()? {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            _ => Err(invalid_bool_byte()),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.read.read_byte_inner()?)
    }

    forward_to_deserialize_any! {
        i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
