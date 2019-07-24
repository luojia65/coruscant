use crate::consts;
use crate::error::{Error, Result};
use crate::read;
use std::borrow::Cow;
use serde::de;
use std::io;

use serde::forward_to_deserialize_any;

pub fn from_reader<'de, R, T>(read: R) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let mut de = Deserializer::new(read);
    T::deserialize(&mut de)
} 

pub struct Deserializer<R> {
    read: R,
}

impl<'de, R> Deserializer<read::IoRead<R>>
where
    R: io::Read,
{
    pub fn new(read: R) -> Self {
        Deserializer { read: read::IoRead::new(read) }
    }

    pub fn into_inner(self) -> R {
        self.read.into_inner()
    }
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
        let type_id = self.read.read_type_id()?;
        let root_name = self.read.read_name()?;
        let value = match type_id {
            consts::TYPE_ID_COMPOUND => visitor.visit_map(CompoundAccess::new(self))?, 
            _ => unimplemented!(),
        };
        drop(root_name);
        Ok(value)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct struct map enum identifier ignored_any
    }
}

struct CompoundAccess<'a, R> {
    type_id: Option<u8>,
    outer: &'a mut Deserializer<R>,
}

impl<'a, R> CompoundAccess<'a, R> {
    fn new(outer: &'a mut Deserializer<R>) -> Self {
        Self { type_id: None, outer }
    }
}

impl<'de, 'a, R> de::MapAccess<'de> for CompoundAccess<'a, R> 
where
    R: read::Read<'de>
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where 
        K: de::DeserializeSeed<'de>
    {
        let type_id = self.outer.read.read_type_id()?;
        if type_id == consts::TYPE_ID_END {
            return Ok(None);
        }
        let key = seed.deserialize(MapKeyDeserializer { outer: self.outer })?;
        self.type_id = Some(type_id);
        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where 
        V: de::DeserializeSeed<'de>
    {
        let type_id = match self.type_id {
            Some(type_id) => type_id,
            None => panic!("call next_value_seed before next_key_seed")
        };
        let value = seed.deserialize(InnerDeserializer { 
            outer: self.outer,
            type_id
        })?;
        Ok(value)
    }
}

struct MapKeyDeserializer<'a, R> {
    outer: &'a mut Deserializer<R>,
}

impl<'de, 'a, R> de::Deserializer<'de> for MapKeyDeserializer<'a, R>
where
    R: read::Read<'de>,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.outer.read.read_name()? {
            Cow::Owned(v) => visitor.visit_string(v),
            Cow::Borrowed(v) => visitor.visit_str(v),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct InnerDeserializer<'a, R> {
    type_id: u8,
    outer: &'a mut Deserializer<R>,
}

impl<'de, 'a, R> de::Deserializer<'de> for InnerDeserializer<'a, R>
where
    R: read::Read<'de>,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {   
        match self.type_id {
            consts::TYPE_ID_BYTE => visitor.visit_i8(self.outer.read.read_byte_inner()?),
            consts::TYPE_ID_SHORT => visitor.visit_i16(self.outer.read.read_short_inner()?),
            consts::TYPE_ID_INT => visitor.visit_i32(self.outer.read.read_int_inner()?),
            consts::TYPE_ID_LONG => visitor.visit_i64(self.outer.read.read_long_inner()?),
            consts::TYPE_ID_FLOAT => visitor.visit_f32(self.outer.read.read_float_inner()?),
            consts::TYPE_ID_DOUBLE => visitor.visit_f64(self.outer.read.read_double_inner()?),
            consts::TYPE_ID_STRING => match self.outer.read.read_string_inner()? {
                Cow::Owned(v) => visitor.visit_string(v),
                Cow::Borrowed(v) => visitor.visit_str(v),
            },
            consts::TYPE_ID_COMPOUND => visitor.visit_map(CompoundAccess::new(self.outer)),
            invalid => Err(Error::invalid_id_at(invalid, self.outer.read.index()))
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.type_id != consts::TYPE_ID_BYTE {
            return Err(Error::mismatch_at(self.type_id, consts::TYPE_ID_BYTE, self.outer.read.index()))
        }
        match self.outer.read.read_byte_inner()? {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            invalid => Err(Error::bool_at(invalid, self.outer.read.index())),
        }
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
