use std::io;
use serde::de;
use crate::error::{Error, Result};
use crate::read;

use serde::forward_to_deserialize_any;


pub fn from_reader<'de, R, T>(read: R) -> Result<T>
where 
    R: io::Read,
    T: de::DeserializeOwned
{
    let read = read::IoRead::new(read);
    let mut de = Deserializer::new(read);
    T::deserialize(&mut de)
}

pub struct Deserializer<R> {
    read: R
}

impl<'de, R> Deserializer<R> 
where 
    R: read::Read<'de>
{
    pub fn new(read: R) -> Self {
        Deserializer { read }
    }

    pub fn into_inner(self) -> R {
        self.read
    }
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R> 
where
    R: read::Read<'de>
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>
    {
        let _ = visitor;
        unimplemented!()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
