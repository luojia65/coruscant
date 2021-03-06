//! Serialize a Rust structure into NBT data.

use std::io;

use crate::{
    consts,
    error::{Error, ErrorCode, Result},
    root, Compression,
};
use byteorder::{BigEndian, WriteBytesExt}; // <- SPICY mojang
use serde::ser::{self, Impossible, Serialize};
use std::borrow::Cow;

#[cfg(feature = "gzip")]
use flate2::write::GzEncoder;
#[cfg(feature = "zlib")]
use flate2::write::ZlibEncoder;

/// Serialize the given data structure with root name as NBT into the `std::io`
/// stream.
///
/// The optional NBT root name is wrapped into variable `root`. For the variable
/// `root`, you may directly pass reference `&value` or value with root name like
/// `("name", &value)`. See `root::Root` for detailed descriptions.
///
/// # Errors
///
/// Serialization can fail for various reasons, for example illegal NBT byte
/// input, an underlying IO operation fails, or `T` contains a map with
/// non-string keys.
pub fn to_writer<'k, 'v, W, T, R>(writer: W, root: R) -> Result<()>
where
    W: io::Write,
    T: 'v + Serialize + ?Sized,
    R: Into<root::Root<'k, 'v, T>>,
{
    let root::Root { root_name, value } = root.into();
    let mut ser = Serializer::binary(writer, root_name);
    value.serialize(&mut ser)
}

/// Serialize the given data structure with root name as NBT, compress it by
/// GZip, then write into the `std::io` stream.
///
/// The optional NBT root name is wrapped into variable `root`. For the variable
/// `root`, you may directly pass reference `&value` or value with root name like
/// `("name", &value)`. See `root::Root` for detailed descriptions.
///
/// # Errors
///
/// Serialization can fail for various reasons, for example illegal NBT byte
/// input, an underlying IO operation fails, or `T` contains a map with
/// non-string keys.
#[cfg(feature = "gzip")]
pub fn to_gzip_writer<'k, 'v, W, T, R>(writer: W, root: R, compression: Compression) -> Result<()>
where
    W: io::Write,
    T: 'v + Serialize + ?Sized,
    R: Into<root::Root<'k, 'v, T>>,
{
    let root::Root { root_name, value } = root.into();
    let writer = GzEncoder::new(writer, compression);
    let mut ser = Serializer::binary(writer, root_name);
    value.serialize(&mut ser)
}

/// Serialize the given data structure with root name as NBT, compress it by
/// Zlib, then write into the `std::io` stream.
///
/// The optional NBT root name is wrapped into variable `root`. For the variable
/// `root`, you may directly pass reference `&value` or value with root name like
/// `("name", &value)`. See `root::Root` for detailed descriptions.
///
/// # Errors
///
/// Serialization can fail for various reasons, for example illegal NBT byte
/// input, an underlying IO operation fails, or `T` contains a map with
/// non-string keys.
#[cfg(feature = "zlib")]
pub fn to_zlib_writer<'k, 'v, W, T, R>(writer: W, root: R, compression: Compression) -> Result<()>
where
    W: io::Write,
    T: 'v + Serialize + ?Sized,
    R: Into<root::Root<'k, 'v, T>>,
{
    let root::Root { root_name, value } = root.into();
    let writer = ZlibEncoder::new(writer, compression);
    let mut ser = Serializer::binary(writer, root_name);
    value.serialize(&mut ser)
}

/// Serialize the given data structure with root name as an NBT byte vector.
///
/// The optional NBT root name is wrapped into variable `root`. For the variable
/// `root`, you may directly pass reference `&value` or value with root name like
/// `("name", &value)`. See `root::Root` for detailed descriptions.
///
/// # Errors
///
/// Serialization can fail for various reasons, for example illegal NBT byte
/// input, an underlying IO operation fails, or `T` contains a map with
/// non-string keys.
pub fn to_vec<'k, 'v, T, R>(root: R) -> Result<Vec<u8>>
where
    T: 'v + Serialize + ?Sized,
    R: Into<root::Root<'k, 'v, T>>,
{
    let root::Root { root_name, value } = root.into();
    let writer = Vec::with_capacity(128);
    let mut ser = Serializer::binary(writer, root_name);
    value.serialize(&mut ser)?;
    Ok(ser.into_inner())
}

/// Serialize the given data structure with root name as NBT string transcript
/// into a `String`.
///
/// The optional NBT root name is wrapped into variable `root`. For the variable
/// `root`, you may directly pass reference `&value` or value with root name like
/// `("name", &value)`. See `root::Root` for detailed descriptions.
///
/// # Errors
///
/// Serialization can fail for various reasons, for example illegal NBT byte
/// input, an underlying IO operation fails, or `T` contains a map with
/// non-string keys.
pub fn to_string_transcript<'k, 'v, T, R>(root: R) -> Result<String>
where
    T: 'v + Serialize + ?Sized,
    R: Into<root::Root<'k, 'v, T>>,
{
    let root::Root { root_name, value } = root.into();
    let writer = Vec::with_capacity(128);
    let mut ser = Serializer::transcript(writer, root_name);
    value.serialize(&mut ser)?;
    Ok(unsafe { String::from_utf8_unchecked(ser.into_inner()) })
}

/// A structure that serializes Rust values into JSON.
pub struct Serializer<'a, W, F> {
    writer: W,
    formatter: F,
    next_name: Cow<'a, str>,
    state: State,
}

#[derive(Debug, PartialEq)]
enum State {
    Root,
    Inner,
}

impl<'a, W, F> Serializer<'a, W, F> {
    /// Creates a new NBT serializer whose output will be written to `writer`
    /// with specific `formatter` given.
    #[inline]
    pub fn with_formatter(writer: W, formatter: F, root_name: &'a str) -> Self {
        Self::new(writer, formatter, root_name)
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }

    #[inline]
    fn new(writer: W, formatter: F, root_name: &'a str) -> Self {
        Serializer {
            writer,
            formatter,
            next_name: root_name.into(),
            state: State::Root,
        }
    }
}

impl<'a, W> Serializer<'a, W, BinaryFormatter> {
    /// Creates a new NBT binary serializer.
    #[inline]
    pub fn binary(writer: W, root_name: &'a str) -> Self {
        Self::new(writer, BinaryFormatter, root_name)
    }
}

impl<'a, W> Serializer<'a, W, TranscriptFormatter<'_>> {
    /// Creates a new NBT transcript serializer.
    #[inline]
    pub fn transcript(writer: W, root_name: &'a str) -> Self {
        Self::new(writer, TranscriptFormatter::new(), root_name)
    }
}

#[inline]
fn unsupported_type() -> Error {
    Error::syntax(ErrorCode::UnsupportedType, 0)
}

#[inline]
fn sequence_size_unknown() -> Error {
    Error::syntax(ErrorCode::SequenceSizeUnknown, 0)
}

impl<'a, 'b: 'a, W, F> ser::Serializer for &'a mut Serializer<'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SerializeList<'a, 'b, W, F>;
    type SerializeTuple = SerializeList<'a, 'b, W, F>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = SerializeCompound<'a, 'b, W, F>;
    type SerializeStruct = SerializeCompound<'a, 'b, W, F>;
    type SerializeStructVariant = Impossible<(), Error>;

    return_expr_for_serialized_types! {
        Err(unsupported_type());
        i128 u8 u16 u32 u64 u128
        unit bytes
    }

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.serialize_i8(if value { 1 } else { 0 })
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        self.formatter.write_byte_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
            value,
        )?;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        self.formatter.write_short_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
            value,
        )?;
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        self.formatter.write_int_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
            value,
        )?;
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        self.formatter.write_long_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
            value,
        )?;
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        self.formatter.write_float_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
            value,
        )?;
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        self.formatter.write_double_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
            value,
        )?;
        Ok(())
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        let mut buf = [0; 4];
        value.encode_utf8(&mut buf);
        let len = value.len_utf8() as i16;
        self.formatter.write_string_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
            len as i16,
            &buf,
        )?;
        Ok(())
    }

    #[inline]
    fn serialize_str(self, s: &str) -> Result<()> {
        if s.len() > i16::max_value() as usize {
            return Err(Error::syntax(ErrorCode::InvalidStringLength, 0));
        }
        self.formatter.write_string_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
            s.len() as i16,
            s.as_bytes(),
        )?;
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    /// Regard unit structs as an empty NBT compound.
    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.formatter.write_compound_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
        )?;
        self.formatter.write_end_tag(&mut self.writer)?;
        Ok(())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        if name == crate::as_nbt_array::TOKEN_ARRAY {
            value.serialize(ArraySerializer { ser: self })?;
            return Ok(());
        }
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            self.serialize_tuple(len)
        } else {
            Err(sequence_size_unknown())
        }
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(SerializeList {
            type_id: None,
            len,
            ser: self,
        })
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.formatter.write_compound_tag(
            &mut self.writer,
            self.next_name.len() as i16,
            self.next_name.as_bytes(),
        )?;
        Ok(SerializeCompound { ser: self })
    }

    #[inline]
    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        let next_name = if self.state == State::Root && self.next_name == "" {
            self.state = State::Inner;
            name
        } else {
            &self.next_name
        };
        self.formatter.write_compound_tag(
            &mut self.writer,
            next_name.len() as i16,
            next_name.as_bytes(),
        )?;
        Ok(SerializeCompound { ser: self })
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

#[doc(hidden)]
pub struct SerializeCompound<'a, 'b, W, F> {
    ser: &'a mut Serializer<'b, W, F>,
}

impl<'a, 'b, W, F> ser::SerializeMap for SerializeCompound<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(MapKeySerializer { ser: self.ser })
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.formatter.write_end_tag(&mut self.ser.writer)?;
        Ok(())
    }
}

impl<'a, 'b, W, F> ser::SerializeStruct for SerializeCompound<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.next_name = key.into();
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.formatter.write_end_tag(&mut self.ser.writer)?;
        Ok(())
    }
}

struct MapKeySerializer<'a, 'b, W, F> {
    ser: &'a mut Serializer<'b, W, F>,
}

#[inline]
fn key_must_be_a_string() -> Error {
    Error::syntax(ErrorCode::KeyMustBeAString, 0)
}

impl<'a, 'b, W, F> ser::Serializer for MapKeySerializer<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    return_expr_for_serialized_types! {
        Err(key_must_be_a_string());
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char bytes none some
        newtype_variant unit unit_struct seq
        tuple tuple_struct tuple_variant struct_variant map struct
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        self.ser.next_name = value.to_owned().into();
        Ok(())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }
}

#[doc(hidden)]
pub struct SerializeList<'a, 'b, W, F> {
    type_id: Option<u8>,
    len: usize,
    ser: &'a mut Serializer<'b, W, F>,
}

impl<'a, 'b, W, F> ser::SerializeSeq for SerializeList<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        if let Some(type_id) = self.type_id {
            value.serialize(ListInnerSerializer {
                type_id,
                ser: self.ser,
            })
        } else {
            let type_id = value.serialize(ListHeadSerializer {
                len: self.len,
                ser: self.ser,
            })?;
            self.type_id = Some(type_id);
            value.serialize(ListInnerSerializer {
                type_id,
                ser: self.ser,
            })
        }
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.formatter.close_list(&mut self.ser.writer)?;
        Ok(())
    }
}

impl<'a, 'b, W, F> ser::SerializeTuple for SerializeList<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        if let Some(type_id) = self.type_id {
            value.serialize(ListInnerSerializer {
                type_id,
                ser: self.ser,
            })
        } else {
            let type_id = value.serialize(ListHeadSerializer {
                len: self.len,
                ser: self.ser,
            })?;
            self.type_id = Some(type_id);
            value.serialize(ListInnerSerializer {
                type_id,
                ser: self.ser,
            })
        }
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.formatter.close_list(&mut self.ser.writer)?;
        Ok(())
    }
}

struct ListHeadSerializer<'a, 'b, W, F> {
    len: usize,
    ser: &'a mut Serializer<'b, W, F>,
}

impl<W, F> ListHeadSerializer<'_, '_, W, F>
where
    W: io::Write,
    F: Formatter,
{
    #[inline]
    fn serialize_head(&mut self, type_id: u8) -> Result<u8> {
        self.ser.formatter.write_list_tag(
            &mut self.ser.writer,
            type_id,
            self.len as i16,
            self.ser.next_name.len() as i16,
            self.ser.next_name.as_bytes(),
        )?;
        Ok(type_id)
    }
}

impl<'a, 'b: 'a, W, F> ser::Serializer for ListHeadSerializer<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = u8;
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = NoOp;
    type SerializeStruct = NoOp;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    return_expr_for_serialized_types! {
        Err(unsupported_list_inner_type());
        u8 u16 u32 u64 bytes
        newtype_variant unit unit_struct seq
        tuple tuple_struct tuple_variant struct_variant
    }

    #[inline]
    fn serialize_bool(mut self, value: bool) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_BYTE)
    }

    #[inline]
    fn serialize_i8(mut self, value: i8) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_BYTE)
    }

    #[inline]
    fn serialize_i16(mut self, value: i16) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_SHORT)
    }

    #[inline]
    fn serialize_i32(mut self, value: i32) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_INT)
    }

    #[inline]
    fn serialize_i64(mut self, value: i64) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_LONG)
    }

    #[inline]
    fn serialize_f32(mut self, value: f32) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_FLOAT)
    }

    #[inline]
    fn serialize_f64(mut self, value: f64) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_DOUBLE)
    }

    #[inline]
    fn serialize_char(mut self, value: char) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_STRING)
    }

    #[inline]
    fn serialize_str(mut self, s: &str) -> Result<Self::Ok> {
        let _ = s;
        self.serialize_head(consts::TYPE_ID_STRING)
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(consts::TYPE_ID_END) //todo!
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_map(mut self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.serialize_head(consts::TYPE_ID_COMPOUND)?;
        Ok(NoOp {
            type_id: consts::TYPE_ID_COMPOUND,
        })
    }

    #[inline]
    fn serialize_struct(
        mut self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_head(consts::TYPE_ID_COMPOUND)?;
        Ok(NoOp {
            type_id: consts::TYPE_ID_COMPOUND,
        })
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }
}

struct ListInnerSerializer<'a, 'b, W, F> {
    type_id: u8,
    ser: &'a mut Serializer<'b, W, F>,
}

#[inline]
fn list_different_type() -> Error {
    Error::syntax(ErrorCode::ListDifferentType, 0)
}

impl<W, F> ListInnerSerializer<'_, '_, W, F>
where
    W: io::Write,
    F: Formatter,
{
    #[inline]
    fn verify_type(&self, type_id: u8) -> Result<()> {
        if type_id != self.type_id {
            return Err(list_different_type());
        }
        Ok(())
    }
}

#[inline]
fn unsupported_list_inner_type() -> Error {
    Error::syntax(ErrorCode::UnsupportedListInnerType, 0)
}

impl<'a, 'b: 'a, W, F> ser::Serializer for ListInnerSerializer<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = SerializeCompound<'a, 'b, W, F>;
    type SerializeStruct = SerializeCompound<'a, 'b, W, F>;
    type SerializeStructVariant = Impossible<(), Error>;

    return_expr_for_serialized_types! {
        Err(unsupported_list_inner_type());
        u8 u16 u32 u64 bytes
        newtype_variant unit unit_struct seq
        tuple tuple_struct tuple_variant struct_variant
    }

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.serialize_i8(if value { 1 } else { 0 })
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        self.verify_type(consts::TYPE_ID_BYTE)?;
        self.ser
            .formatter
            .write_byte_inner(&mut self.ser.writer, value)?;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        self.verify_type(consts::TYPE_ID_SHORT)?;
        self.ser
            .formatter
            .write_short_inner(&mut self.ser.writer, value)?;
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        self.verify_type(consts::TYPE_ID_INT)?;
        self.ser
            .formatter
            .write_int_inner(&mut self.ser.writer, value)?;
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        self.verify_type(consts::TYPE_ID_LONG)?;
        self.ser
            .formatter
            .write_long_inner(&mut self.ser.writer, value)?;
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        self.verify_type(consts::TYPE_ID_FLOAT)?;
        self.ser
            .formatter
            .write_float_inner(&mut self.ser.writer, value)?;
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        self.verify_type(consts::TYPE_ID_DOUBLE)?;
        self.ser
            .formatter
            .write_double_inner(&mut self.ser.writer, value)?;
        Ok(())
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        self.verify_type(consts::TYPE_ID_STRING)?;
        let mut buf = [0; 4];
        value.encode_utf8(&mut buf);
        let len = value.len_utf8() as i16;
        self.ser
            .formatter
            .write_string_inner(&mut self.ser.writer, len, &buf)?;
        Ok(())
    }

    #[inline]
    fn serialize_str(self, s: &str) -> Result<()> {
        self.verify_type(consts::TYPE_ID_STRING)?;
        if s.len() > i16::max_value() as usize {
            return Err(Error::syntax(ErrorCode::InvalidStringLength, 0));
        }
        self.ser.formatter.write_string_inner(
            &mut self.ser.writer,
            s.len() as i16,
            s.as_bytes(),
        )?;
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.verify_type(consts::TYPE_ID_COMPOUND)?;
        self.ser
            .formatter
            .write_compound_inner(&mut self.ser.writer)?;
        Ok(SerializeCompound { ser: self.ser })
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.verify_type(consts::TYPE_ID_COMPOUND)?;
        self.ser
            .formatter
            .write_compound_inner(&mut self.ser.writer)?;
        Ok(SerializeCompound { ser: self.ser })
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }
}

struct ArraySerializer<'a, 'b, W, F> {
    ser: &'a mut Serializer<'b, W, F>,
}

#[inline]
fn unsupported_array_type() -> Error {
    Error::syntax(ErrorCode::UnsupportedArrayType, 0)
}

impl<'a, 'b: 'a, W, F> ser::Serializer for ArraySerializer<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SerializeArray<'a, 'b, W, F>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    return_expr_for_serialized_types! {
        Err(unsupported_array_type());
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char bytes none some
        newtype_variant unit unit_struct
        tuple tuple_struct tuple_variant struct_variant map struct
        str unit_variant newtype_struct
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            Ok(SerializeArray {
                len,
                ser: self.ser,
                type_id: None,
            })
        } else {
            Err(sequence_size_unknown())
        }
    }
}

struct SerializeArray<'a, 'b, W, F> {
    type_id: Option<u8>,
    len: usize,
    ser: &'a mut Serializer<'b, W, F>,
}

impl<'a, 'b, W, F> ser::SerializeSeq for SerializeArray<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        if let Some(type_id) = self.type_id {
            value.serialize(ArrayInnerSerializer {
                type_id,
                ser: self.ser,
            })
        } else {
            let type_id = value.serialize(ArrayHeadSerializer {
                len: self.len,
                ser: self.ser,
            })?;
            self.type_id = Some(type_id);
            value.serialize(ArrayInnerSerializer {
                type_id,
                ser: self.ser,
            })
        }
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        self.ser.formatter.close_array(&mut self.ser.writer)?;
        Ok(())
    }
}

struct ArrayHeadSerializer<'a, 'b, W, F> {
    len: usize,
    ser: &'a mut Serializer<'b, W, F>,
}

impl<W, F> ArrayHeadSerializer<'_, '_, W, F>
where
    W: io::Write,
    F: Formatter,
{
    #[inline]
    fn serialize_head(&mut self, type_id: u8) -> Result<u8> {
        self.ser.formatter.write_array_head(
            &mut self.ser.writer,
            type_id,
            self.len as i16,
            self.ser.next_name.len() as i16,
            self.ser.next_name.as_bytes(),
        )?;
        Ok(type_id)
    }
}

impl<'a, 'b: 'a, W, F> ser::Serializer for ArrayHeadSerializer<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = u8;
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    return_expr_for_serialized_types! {
        Err(unsupported_list_inner_type());
        bool i16 u8 u16 u32 u64 f32 f64 char str bytes
        none some newtype_variant unit unit_struct unit_variant seq
        map struct tuple tuple_struct tuple_variant struct_variant
    }

    #[inline]
    fn serialize_i8(mut self, value: i8) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_BYTE_ARRAY)
    }

    #[inline]
    fn serialize_i32(mut self, value: i32) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_INT_ARRAY)
    }

    #[inline]
    fn serialize_i64(mut self, value: i64) -> Result<Self::Ok> {
        let _ = value;
        self.serialize_head(consts::TYPE_ID_LONG_ARRAY)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }
}

struct ArrayInnerSerializer<'a, 'b, W, F> {
    type_id: u8,
    ser: &'a mut Serializer<'b, W, F>,
}

#[inline]
fn array_different_type() -> Error {
    Error::syntax(ErrorCode::ArrayDifferentType, 0)
}

impl<W, F> ArrayInnerSerializer<'_, '_, W, F>
where
    W: io::Write,
    F: Formatter,
{
    #[inline]
    fn verify_type(&self, type_id: u8) -> Result<()> {
        if type_id != self.type_id {
            return Err(array_different_type());
        }
        Ok(())
    }
}

#[inline]
fn unsupported_array_inner_type() -> Error {
    Error::syntax(ErrorCode::UnsupportedArrayInnerType, 0)
}

impl<'a, 'b: 'a, W, F> ser::Serializer for ArrayInnerSerializer<'a, 'b, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = SerializeCompound<'a, 'b, W, F>;
    type SerializeStruct = SerializeCompound<'a, 'b, W, F>;
    type SerializeStructVariant = Impossible<(), Error>;

    return_expr_for_serialized_types! {
        Err(unsupported_array_inner_type());
        bool u8 u16 u32 u64 i16 f32 f64 char str bytes none some
        newtype_variant unit unit_struct unit_variant seq map struct
        tuple tuple_struct tuple_variant struct_variant
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        self.verify_type(consts::TYPE_ID_BYTE_ARRAY)?;
        self.ser
            .formatter
            .write_byte_inner(&mut self.ser.writer, value)?;
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        self.verify_type(consts::TYPE_ID_INT_ARRAY)?;
        self.ser
            .formatter
            .write_int_inner(&mut self.ser.writer, value)?;
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        self.verify_type(consts::TYPE_ID_LONG_ARRAY)?;
        self.ser
            .formatter
            .write_long_inner(&mut self.ser.writer, value)?;
        Ok(())
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }
}

struct NoOp {
    type_id: u8,
}

impl ser::SerializeSeq for NoOp {
    type Ok = u8;
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<u8> {
        Ok(self.type_id)
    }
}

impl ser::SerializeStruct for NoOp {
    type Ok = u8;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<u8> {
        Ok(self.type_id)
    }
}

impl ser::SerializeMap for NoOp {
    type Ok = u8;
    type Error = Error;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<u8> {
        Ok(self.type_id)
    }
}

pub trait Formatter {
    #[inline]
    fn write_compound_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_COMPOUND)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)
    }

    #[inline]
    fn write_end_tag<W: ?Sized>(&mut self, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_END)
    }

    #[inline]
    fn write_byte_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: i8,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_BYTE)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i8(value)
    }

    #[inline]
    fn write_short_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: i16,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_SHORT)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i16::<BigEndian>(value)
    }

    #[inline]
    fn write_int_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: i32,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_INT)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i32::<BigEndian>(value)
    }

    #[inline]
    fn write_long_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: i64,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_LONG)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i64::<BigEndian>(value)
    }

    #[inline]
    fn write_float_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: f32,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_FLOAT)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_f32::<BigEndian>(value)
    }

    #[inline]
    fn write_double_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: f64,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_DOUBLE)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_f64::<BigEndian>(value)
    }

    #[inline]
    fn write_string_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        string_len: i16,
        string_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_STRING)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i16::<BigEndian>(string_len)?;
        w.write_all(string_bytes)
    }

    #[inline]
    fn write_list_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        type_id: u8,
        len: i16,
        name_len: i16,
        name_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(consts::TYPE_ID_LIST)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_u8(type_id)?;
        w.write_i16::<BigEndian>(len)
    }

    #[inline]
    fn close_list<W: ?Sized>(&mut self, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = w;
        Ok(())
    }

    #[inline]
    fn write_array_head<W: ?Sized>(
        &mut self,
        w: &mut W,
        type_id: u8,
        len: i16,
        name_len: i16,
        name_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_u8(type_id)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i16::<BigEndian>(len)
    }

    #[inline]
    fn close_array<W: ?Sized>(&mut self, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = w;
        Ok(())
    }

    #[inline]
    fn write_compound_inner<W: ?Sized>(&mut self, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = w;
        Ok(())
    }

    #[inline]
    fn write_byte_inner<W>(&mut self, w: &mut W, value: i8) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_i8(value)
    }

    #[inline]
    fn write_short_inner<W>(&mut self, w: &mut W, value: i16) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_i16::<BigEndian>(value)
    }

    #[inline]
    fn write_int_inner<W>(&mut self, w: &mut W, value: i32) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_i32::<BigEndian>(value)
    }

    #[inline]
    fn write_long_inner<W>(&mut self, w: &mut W, value: i64) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_i64::<BigEndian>(value)
    }

    #[inline]
    fn write_float_inner<W>(&mut self, w: &mut W, value: f32) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_f32::<BigEndian>(value)
    }

    #[inline]
    fn write_double_inner<W>(&mut self, w: &mut W, value: f64) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_f64::<BigEndian>(value)
    }

    #[inline]
    fn write_string_inner<W: ?Sized>(
        &mut self,
        w: &mut W,
        string_len: i16,
        string_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        w.write_i16::<BigEndian>(string_len)?;
        w.write_all(string_bytes)
    }
}

pub struct BinaryFormatter;

impl Formatter for BinaryFormatter {}

pub struct TranscriptFormatter<'a> {
    current_indent: usize,
    indent: &'a [u8],
}

impl<'a> TranscriptFormatter<'a> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn with_indent(indent: &'a [u8]) -> Self {
        TranscriptFormatter {
            current_indent: 0,
            indent,
        }
    }
}

impl Default for TranscriptFormatter<'_> {
    #[inline]
    fn default() -> Self {
        Self::with_indent(b"  ")
    }
}

impl Formatter for TranscriptFormatter<'_> {
    fn write_compound_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Compound '{}'", name)?;
        self.current_indent += 1;
        Ok(())
    }

    fn write_end_tag<W: ?Sized>(&mut self, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.current_indent -= 1;
        indent(w, self.current_indent, self.indent)?;
        write!(w, "EndCompound")?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_byte_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: i8,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        write!(w, "Byte '{}' {}", name, value)?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_short_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: i16,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        write!(w, "Short '{}' {}", name, value)?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_int_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: i32,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        write!(w, "Int '{}' {}", name, value)?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_long_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: i64,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        write!(w, "Long '{}' {}", name, value)?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_float_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: f32,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        write!(w, "Float '{}' {}", name, value)?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_double_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        value: f64,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        write!(w, "Double '{}' {}", name, value)?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_string_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        name_len: i16,
        name_bytes: &[u8],
        string_len: i16,
        string_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = (name_len, string_len);
        let name = String::from_utf8_lossy(name_bytes);
        let string = String::from_utf8_lossy(string_bytes);
        indent(w, self.current_indent, self.indent)?;
        write!(w, "String '{}' {}", name, string)?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_list_tag<W: ?Sized>(
        &mut self,
        w: &mut W,
        type_id: u8,
        len: i16,
        name_len: i16,
        name_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "List '{}': [{}; {}]", name, type_id, len)?;
        self.current_indent += 1;
        Ok(())
    }

    fn close_list<W: ?Sized>(&mut self, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.current_indent -= 1;
        indent(w, self.current_indent, self.indent)?;
        write!(w, "EndList")?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    #[inline]
    fn write_array_head<W: ?Sized>(
        &mut self,
        w: &mut W,
        type_id: u8,
        len: i16,
        name_len: i16,
        name_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        let (array_type, inner_type) = match type_id {
            consts::TYPE_ID_BYTE_ARRAY => ("ByteArray", "Byte"),
            consts::TYPE_ID_INT_ARRAY => ("IntArray", "Int"),
            consts::TYPE_ID_LONG_ARRAY => ("LongArray", "Long"),
            _ => panic!("wrong type_id parameter"),
        };
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "{} '{}': [{}; {}]", array_type, name, inner_type, len)?;
        self.current_indent += 1;
        Ok(())
    }

    #[inline]
    fn close_array<W: ?Sized>(&mut self, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.current_indent -= 1;
        indent(w, self.current_indent, self.indent)?;
        write!(w, "EndArray")?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_compound_inner<W: ?Sized>(&mut self, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Compound")?;
        self.current_indent += 1;
        Ok(())
    }

    fn write_byte_inner<W>(&mut self, w: &mut W, value: i8) -> io::Result<()>
    where
        W: io::Write,
    {
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Byte {}", value)?;
        Ok(())
    }

    fn write_short_inner<W>(&mut self, w: &mut W, value: i16) -> io::Result<()>
    where
        W: io::Write,
    {
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Short {}", value)?;
        Ok(())
    }

    fn write_int_inner<W>(&mut self, w: &mut W, value: i32) -> io::Result<()>
    where
        W: io::Write,
    {
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Int {}", value)?;
        Ok(())
    }

    fn write_long_inner<W>(&mut self, w: &mut W, value: i64) -> io::Result<()>
    where
        W: io::Write,
    {
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Long {}", value)?;
        Ok(())
    }

    fn write_float_inner<W>(&mut self, w: &mut W, value: f32) -> io::Result<()>
    where
        W: io::Write,
    {
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Float {}", value)?;
        Ok(())
    }

    fn write_double_inner<W>(&mut self, w: &mut W, value: f64) -> io::Result<()>
    where
        W: io::Write,
    {
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Double {}", value)?;
        Ok(())
    }

    fn write_string_inner<W: ?Sized>(
        &mut self,
        w: &mut W,
        string_len: i16,
        string_bytes: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let _ = string_len;
        let string = String::from_utf8_lossy(string_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "String {}", string)?;
        Ok(())
    }
}

fn indent<W: ?Sized>(w: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: io::Write,
{
    for _ in 0..n {
        w.write_all(s)?;
    }

    Ok(())
}
