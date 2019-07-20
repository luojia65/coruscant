use std::io;
use serde::de;
use crate::error::{Error, Result};

use serde::forward_to_deserialize_any;


pub fn from_reader<R, T>(reader: R) -> Result<T>
where 
    R: io::Read,
    T: de::DeserializeOwned
{
    let mut de = Deserializer::new(reader);
    T::deserialize(&mut de)
}

pub struct Deserializer<R> {
    reader: R
}

impl<R> Deserializer<R> 
where 
    R: io::Read
{
    pub fn new(reader: R) -> Self {
        Deserializer { reader }
    }

    pub fn into_inner(self) -> R {
        self.reader
    }
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R> 
where
    R: io::Read 
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
