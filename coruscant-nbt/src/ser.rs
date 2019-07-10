use std::io;

use crate::consts;
use crate::error::{Error, ErrorCode, Result};
use crate::value::{Value, to_value};
use serde::ser::{self, Impossible, Serialize};
use byteorder::{WriteBytesExt, BigEndian}; // <- SPICY mojang

// not in no_std circumstances
pub struct Serializer<W, F> {
    writer: W,
    formatter: F
}

impl<W, F> Serializer<W, F> 
where 
    W: io::Write,
    F: Formatter,
{
    pub fn new(writer: W, formatter: F) -> Self {
        Serializer { writer, formatter }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

fn unsupported_type() -> Error {
    Error::syntax(ErrorCode::UnsupportedType, 0, 0)
}

impl<'a, W, F> ser::Serializer for &'a mut Serializer<W, F> 
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
    type SerializeMap = Compound<'a, W, F>;
    type SerializeStruct = Compound<'a, W, F>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.formatter.write_bool(&mut self.writer, value).map_err(Error::io)
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.formatter.write_i8(&mut self.writer, value).map_err(Error::io)
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.formatter.write_i16(&mut self.writer, value).map_err(Error::io)
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.formatter.write_i32(&mut self.writer, value).map_err(Error::io)
    }

    fn serialize_i64(self, value: i64) -> Result<()> {        
        self.formatter.write_i64(&mut self.writer, value).map_err(Error::io)
    }

    fn serialize_i128(self, _value: i128) -> Result<()> {
        Err(unsupported_type())
    }

    fn serialize_u8(self, _value: u8) -> Result<()> {
        Err(unsupported_type())
    }

    fn serialize_u16(self, _value: u16) -> Result<()> {
        Err(unsupported_type())
    }

    fn serialize_u32(self, _value: u32) -> Result<()> {
        Err(unsupported_type())
    }

    fn serialize_u64(self, _value: u64) -> Result<()> {
        Err(unsupported_type())
    }

    fn serialize_u128(self, _value: u128) -> Result<()> {
        Err(unsupported_type())
    }

    fn serialize_f32(self, value: f32) -> Result<()> {
        self.formatter.write_f32(&mut self.writer, value).map_err(Error::io)
    }

    fn serialize_f64(self, value: f64) -> Result<()> {
        self.formatter.write_f64(&mut self.writer, value).map_err(Error::io)
    }

    fn serialize_char(self, value: char) -> Result<()> {
        let mut buf = [0; 4];
        value.encode_utf8(&mut buf);
        let len = value.len_utf8() as i16; 
        self.formatter.write_str(&mut self.writer, len, &buf).map_err(Error::io)
    }

    fn serialize_str(self, s: &str) -> Result<()> {
        if s.len() > i16::max_value() as usize {
            return Err(Error::syntax(ErrorCode::InvalidStringLength, 0, 0))
        }
        self.formatter.write_str(&mut self.writer, s.len() as i16, s.as_bytes()).map_err(Error::io)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        Err(unsupported_type())
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        Err(unsupported_type())
    }

    fn serialize_unit(self) -> Result<()> {
        Err(unsupported_type())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

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

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.formatter.begin_compound(&mut self.writer).map_err(Error::io)?;
        self.formatter.write_str(&mut self.writer, name.len() as i16, name.as_bytes()).map_err(Error::io)?;
        Ok(Compound {
            ser: self
        })
    }

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

pub struct Compound<'a, W, F> {
    ser: &'a mut Serializer<W, F>,
}

impl<'a, W, F> ser::SerializeMap for Compound<'a, W, F> 
where 
    W: io::Write,
    F: Formatter
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        let key = to_value(key)?;
        self.ser.formatter.write_type_id(&mut self.ser.writer, &key).map_err(Error::io)?;
        key.serialize(MapKeySerializer { ser: self.ser })
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        let value_conv = to_value(value)?;
        self.ser.formatter.write_type_id(&mut self.ser.writer, &value_conv).map_err(Error::io)?;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        self.ser.formatter.end_compound(&mut self.ser.writer).map_err(Error::io)
    }
}

impl<'a, W, F> ser::SerializeStruct for Compound<'a, W, F> 
where
    W: io::Write, 
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where   
        T: ?Sized + Serialize 
    {   
        self.ser.formatter.write_str(&mut self.ser.writer, key.len() as i16, key.as_bytes()).map_err(Error::io)?;
        let value_conv = to_value(value)?;
        self.ser.formatter.write_type_id(&mut self.ser.writer, &value_conv).map_err(Error::io)?;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {        
        self.ser.formatter.end_compound(&mut self.ser.writer).map_err(Error::io)
    }
}

struct MapKeySerializer<'a, W: 'a, F: 'a> {
    ser: &'a mut Serializer<W, F>,
}

fn key_must_be_a_string() -> Error {
    Error::syntax(ErrorCode::KeyMustBeAString, 0, 0)
}

impl<'a, W, F> ser::Serializer for MapKeySerializer<'a, W, F> 
where W: io::Write, F: Formatter 
{
    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        self.ser.serialize_str(value)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.ser.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;


    fn serialize_bool(self, _value: bool) -> Result<()> {
        Err(key_must_be_a_string())
    }

    fn serialize_i8(self, _value: i8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i16(self, _value: i16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i32(self, _value: i32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i64(self, _value: i64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i128(self, _value: i128) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u8(self, _value: u8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u16(self, _value: u16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u32(self, _value: u32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u64(self, _value: u64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u128(self, _value: u128) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f32(self, _value: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, _value: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, _value: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> Result<()> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(key_must_be_a_string())
    }

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
        Err(key_must_be_a_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(key_must_be_a_string())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(key_must_be_a_string())
    }
}

pub trait Formatter {
    #[inline]
    fn write_bool<W: ?Sized>(&mut self, writer: &mut W, value: bool) -> io::Result<()> 
    where
        W: io::Write
    {
        writer.write_i8(if value { 1 } else { 0 })
    }

    #[inline]
    fn write_i8<W: ?Sized>(&mut self, writer: &mut W, value: i8) -> io::Result<()> 
    where
        W: io::Write
    {
        writer.write_i8(value)
    }

    #[inline]
    fn write_i16<W: ?Sized>(&mut self, writer: &mut W, value: i16) -> io::Result<()> 
    where
        W: io::Write
    {
        writer.write_i16::<BigEndian>(value)
    }

    #[inline]
    fn write_i32<W: ?Sized>(&mut self, writer: &mut W, value: i32) -> io::Result<()> 
    where
        W: io::Write
    {
        writer.write_i32::<BigEndian>(value)
    }

    #[inline]
    fn write_i64<W: ?Sized>(&mut self, writer: &mut W, value: i64) -> io::Result<()> 
    where
        W: io::Write
    {
        writer.write_i64::<BigEndian>(value)
    }

    #[inline]
    fn write_f32<W: ?Sized>(&mut self, writer: &mut W, value: f32) -> io::Result<()> 
    where
        W: io::Write
    {
        writer.write_f32::<BigEndian>(value)
    }

    #[inline]
    fn write_f64<W: ?Sized>(&mut self, writer: &mut W, value: f64) -> io::Result<()> 
    where
        W: io::Write
    {
        writer.write_f64::<BigEndian>(value)
    }

    #[inline]
    fn write_str<W: ?Sized>(&mut self, writer: &mut W, len: i16, bytes: &[u8]) -> io::Result<()> 
    where 
        W: io::Write
    {
        writer.write_i16::<BigEndian>(len)?;
        writer.write_all(bytes)
    }

    #[inline]
    fn write_type_id<W: ?Sized>(&mut self, writer: &mut W, value: &Value) -> io::Result<()> 
    where
        W: io::Write
    {
        writer.write_u8(value.type_id())
    }

    #[inline]
    fn begin_compound<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()> 
    where
        W: io::Write 
    {
        writer.write_u8(consts::TYPE_ID_COMPOUND)
    }

    #[inline]
    fn end_compound<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()> 
    where
        W: io::Write 
    {
        writer.write_u8(consts::TYPE_ID_END)
    }

}

pub struct BinaryFormatter;

impl Formatter for BinaryFormatter {}

pub fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()> 
where 
    W: io::Write,
    T: Serialize,
{
    let mut ser = Serializer::new(writer, BinaryFormatter);
    value.serialize(&mut ser)
}

pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>> 
where 
    T: Serialize
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}
