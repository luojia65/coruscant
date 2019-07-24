//! Deserialize NBT data to a Rust data structure.

use crate::consts;
use crate::error::{Error, Result};
use crate::read;
use serde::de;
use std::borrow::Cow;
use std::io;

#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;
#[cfg(feature = "zlib")]
use flate2::read::ZlibDecoder;

use serde::forward_to_deserialize_any;

/// Deserialize an instance of type `T` from an IO stream of NBT. 
pub fn from_reader<R, T>(read: R) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let mut de = Deserializer::io(read);
    T::deserialize(&mut de)
}

/// Deserialize an instance of type `T` from a GZip compressed IO stream of NBT. 
#[cfg(feature = "gzip")]
pub fn from_gzip_reader<R, T>(read: R) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let read = GzDecoder::new(read);
    let mut de = Deserializer::io(read);
    T::deserialize(&mut de)
}

/// Deserialize an instance of type `T` from a Zlib compressed IO stream of NBT. 
#[cfg(feature = "zlib")]
pub fn from_zlib_reader<R, T>(read: R) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let read = ZlibDecoder::new(read);
    let mut de = Deserializer::io(read);
    T::deserialize(&mut de)
}

/// Deserialize an instance of type `T` from an NBT byte slice. 
pub fn from_slice<'a, T>(slice: &'a [u8]) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    let mut de = Deserializer::slice(slice);
    T::deserialize(&mut de)
}

/// A struct that deserializes NBT into Rust values.
pub struct Deserializer<R> {
    read: R,
}

impl<R> Deserializer<read::IoRead<R>>
where
    R: io::Read,
{
    /// Create a NBT deserializer from an `io::Read`. 
    pub fn io(read: R) -> Self {
        Deserializer {
            read: read::IoRead::new(read),
        }
    }

    /// Unwrap `io::Read` from the NBT deserializer.
    pub fn into_inner(self) -> R {
        self.read.into_inner()
    }
}

impl<'a> Deserializer<read::SliceRead<'a>> {
    /// Create a NBT deserializer from a `&[u8]`.
    pub fn slice(slice: &'a [u8]) -> Self {
        Deserializer {
            read: read::SliceRead::new(slice),
        }
    }

    /// Unwrap `&[u8]` from the NBT deserializer.
    pub fn into_inner(self) -> &'a [u8] {
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
        let value = proc_deserialize_value(visitor, type_id, self)?;
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
        Self {
            type_id: None,
            outer,
        }
    }
}

impl<'de, 'a, R> de::MapAccess<'de> for CompoundAccess<'a, R>
where
    R: read::Read<'de>,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
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
        V: de::DeserializeSeed<'de>,
    {
        let type_id = match self.type_id {
            Some(type_id) => type_id,
            None => panic!("call next_value_seed before next_key_seed"),
        };
        let value = seed.deserialize(InnerDeserializer {
            outer: self.outer,
            type_id,
        })?;
        Ok(value)
    }
}

struct ListOrArrayAccess<'a, R> {
    type_id: u8,
    cur_len: i16,
    total_len: i16,
    outer: &'a mut Deserializer<R>,
}

impl<'de, 'a, R> ListOrArrayAccess<'a, R>
where
    R: read::Read<'de>,
{
    fn list(outer: &'a mut Deserializer<R>) -> Result<Self> {
        let type_id = outer.read.read_type_id()?;
        let total_len = outer.read.read_length()?;
        Ok(Self {
            type_id,
            cur_len: 0,
            total_len,
            outer,
        })
    }

    fn byte_array(outer: &'a mut Deserializer<R>) -> Result<Self> {
        let total_len = outer.read.read_length()?;
        Ok(Self {
            type_id: consts::TYPE_ID_BYTE,
            cur_len: 0,
            total_len,
            outer,
        })
    }

    fn int_array(outer: &'a mut Deserializer<R>) -> Result<Self> {
        let total_len = outer.read.read_length()?;
        Ok(Self {
            type_id: consts::TYPE_ID_INT,
            cur_len: 0,
            total_len,
            outer,
        })
    }

    fn long_array(outer: &'a mut Deserializer<R>) -> Result<Self> {
        let total_len = outer.read.read_length()?;
        Ok(Self {
            type_id: consts::TYPE_ID_LONG,
            cur_len: 0,
            total_len,
            outer,
        })
    }
}

impl<'de, 'a, R> de::SeqAccess<'de> for ListOrArrayAccess<'a, R>
where
    R: read::Read<'de>,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.cur_len == self.total_len {
            return Ok(None);
        }
        let value = seed.deserialize(InnerDeserializer {
            outer: self.outer,
            type_id: self.type_id,
        })?;
        self.cur_len += 1;
        Ok(Some(value))
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
        proc_deserialize_value(visitor, self.type_id, self.outer)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.type_id != consts::TYPE_ID_BYTE {
            return Err(Error::mismatch_at(
                self.type_id,
                consts::TYPE_ID_BYTE,
                self.outer.read.index(),
            ));
        }
        match self.outer.read.read_byte_inner()? {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            invalid => Err(Error::bool_at(invalid, self.outer.read.index())),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.type_id != consts::TYPE_ID_STRING {
            return Err(Error::mismatch_at(
                self.type_id,
                consts::TYPE_ID_STRING,
                self.outer.read.index(),
            ));
        }
        match self.outer.read.read_string_inner()? {
            Cow::Borrowed(borrowed) => visitor.visit_borrowed_str(borrowed),
            Cow::Owned(owned) => visitor.visit_str(&owned),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.type_id != consts::TYPE_ID_STRING {
            return Err(Error::mismatch_at(
                self.type_id,
                consts::TYPE_ID_STRING,
                self.outer.read.index(),
            ));
        }
        let owned = self.outer.read.read_string_inner()?.into_owned();
        visitor.visit_string(owned)
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

#[inline]
fn proc_deserialize_value<'de, 'a, R, V>(
    visitor: V,
    type_id: u8,
    outer: &'a mut Deserializer<R>,
) -> Result<V::Value>
where
    R: read::Read<'de>,
    V: de::Visitor<'de>,
{
    // no TYPE_ID_STRING here: they should be deserialized as borrowed for owned format
    match type_id {
        consts::TYPE_ID_BYTE => visitor.visit_i8(outer.read.read_byte_inner()?),
        consts::TYPE_ID_SHORT => visitor.visit_i16(outer.read.read_short_inner()?),
        consts::TYPE_ID_INT => visitor.visit_i32(outer.read.read_int_inner()?),
        consts::TYPE_ID_LONG => visitor.visit_i64(outer.read.read_long_inner()?),
        consts::TYPE_ID_FLOAT => visitor.visit_f32(outer.read.read_float_inner()?),
        consts::TYPE_ID_DOUBLE => visitor.visit_f64(outer.read.read_double_inner()?),
        consts::TYPE_ID_BYTE_ARRAY => visitor.visit_seq(ListOrArrayAccess::byte_array(outer)?),
        consts::TYPE_ID_LIST => visitor.visit_seq(ListOrArrayAccess::list(outer)?),
        consts::TYPE_ID_COMPOUND => visitor.visit_map(CompoundAccess::new(outer)),
        consts::TYPE_ID_INT_ARRAY => visitor.visit_seq(ListOrArrayAccess::int_array(outer)?),
        consts::TYPE_ID_LONG_ARRAY => visitor.visit_seq(ListOrArrayAccess::long_array(outer)?),
        invalid => Err(Error::invalid_id_at(invalid, outer.read.index())),
    }
}
