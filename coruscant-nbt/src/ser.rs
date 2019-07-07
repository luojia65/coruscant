use std::io;

use crate::consts;
use crate::error::{Error, ErrorCode, Result};
use serde::ser::{self, Impossible, Serialize};
use byteorder::{WriteBytesExt, BigEndian}; // <- SPICY mojang

// not in no_std circumstances
pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> 
where 
    W: io::Write 
{
    pub fn new(writer: W) -> Self {
        Serializer { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

fn unsupported_type() -> Error {
    Error::syntax(ErrorCode::UnsupportedType, 0, 0)
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W> 
where  
    W: io::Write
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.serialize_i8(if value { 1 } else { 0 })
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.writer.write_u8(consts::TYPE_ID_BYTE).map_err(Error::io)?;
        self.writer.write_i8(value).map_err(Error::io)
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.writer.write_u8(consts::TYPE_ID_SHORT).map_err(Error::io)?;
        self.writer.write_i16::<BigEndian>(value).map_err(Error::io)
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.writer.write_u8(consts::TYPE_ID_INT).map_err(Error::io)?;
        self.writer.write_i32::<BigEndian>(value).map_err(Error::io)
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        self.writer.write_u8(consts::TYPE_ID_LONG).map_err(Error::io)?;
        self.writer.write_i64::<BigEndian>(value).map_err(Error::io)
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
        self.writer.write_u8(consts::TYPE_ID_FLOAT).map_err(Error::io)?;
        self.writer.write_f32::<BigEndian>(value).map_err(Error::io)
    }

    fn serialize_f64(self, value: f64) -> Result<()> {
        self.writer.write_u8(consts::TYPE_ID_DOUBLE).map_err(Error::io)?;
        self.writer.write_f64::<BigEndian>(value).map_err(Error::io)
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.serialize_str(&value.to_string())
    }

    fn serialize_str(self, s: &str) -> Result<()> {
        self.writer.write_u8(consts::TYPE_ID_STRING).map_err(Error::io)?;
        if s.len() > i16::max_value() as usize {
            return Err(Error::syntax(ErrorCode::InvalidStringLength, 0, 0))
        }
        self.writer.write_i16::<BigEndian>(s.len() as i16).map_err(Error::io)?;
        self.writer.write_all(s.as_bytes()).map_err(Error::io)
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
        _value: &T,
    ) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
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
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_str(name)?;
        self.serialize_map(Some(len))
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

impl<'a, W> ser::SerializeMap for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where   
        T: ?Sized + Serialize 
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

