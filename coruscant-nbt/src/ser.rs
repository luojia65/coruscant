/// Test case taken from https://wiki.vg/NBT

use std::io;

use crate::consts;
use crate::error::{Error, ErrorCode, Result};
use crate::root;
use serde::ser::{self, Impossible, Serialize};
use byteorder::{WriteBytesExt, BigEndian}; // <- SPICY mojang

// not in no_std circumstances
pub struct Serializer<'a, W, F> {
    writer: W,
    formatter: F,
    next_name: &'a str,
}

impl<'a, W, F> Serializer<'a, W, F> {
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }

    #[inline]
    fn new(writer: W, formatter: F, root_name: &'a str) -> Self {
        Serializer { writer, formatter, next_name: root_name }
    }
}

impl<'a, W> Serializer<'a, W, BinaryFormatter> {
    #[inline]
    pub fn binary(writer: W, root_name: &'a str) -> Self {
        Self::new(writer, BinaryFormatter, root_name)
    }
}

impl<'a, W> Serializer<'a, W, TranscriptFormatter<'_>> {
    #[inline]
    pub fn transcript(writer: W, root_name: &'a str) -> Self {
        Self::new(writer, TranscriptFormatter::new(), root_name)
    }
}

#[inline]
fn unsupported_type() -> Error {
    Error::syntax(ErrorCode::UnsupportedType, 0, 0)
}

impl<'a, 'b: 'a, W, F> ser::Serializer for &'a mut Serializer<'b, W, F> 
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
            value
        ).map_err(Error::io)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        self.formatter.write_short_tag(
            &mut self.writer, 
            self.next_name.len() as i16, 
            self.next_name.as_bytes(),
            value
        ).map_err(Error::io)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        self.formatter.write_int_tag(
            &mut self.writer, 
            self.next_name.len() as i16, 
            self.next_name.as_bytes(),
            value
        ).map_err(Error::io)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {  
        self.formatter.write_long_tag(
            &mut self.writer, 
            self.next_name.len() as i16, 
            self.next_name.as_bytes(),
            value
        ).map_err(Error::io)
    }

    #[inline]
    fn serialize_i128(self, _value: i128) -> Result<()> {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_u8(self, _value: u8) -> Result<()> {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<()> {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<()> {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_u64(self, _value: u64) -> Result<()> {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_u128(self, _value: u128) -> Result<()> {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        self.formatter.write_float_tag(
            &mut self.writer, 
            self.next_name.len() as i16, 
            self.next_name.as_bytes(),
            value
        ).map_err(Error::io)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        self.formatter.write_double_tag(
            &mut self.writer, 
            self.next_name.len() as i16, 
            self.next_name.as_bytes(),
            value
        ).map_err(Error::io)
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
            &buf
        ).map_err(Error::io)
    }

    #[inline]
    fn serialize_str(self, s: &str) -> Result<()> {
        if s.len() > i16::max_value() as usize {
            return Err(Error::syntax(ErrorCode::InvalidStringLength, 0, 0))
        }
        self.formatter.write_string_tag(
            &mut self.writer, 
            self.next_name.len() as i16, 
            self.next_name.as_bytes(),
            s.len() as i16, 
            s.as_bytes()
        ).map_err(Error::io)
    }

    #[inline]
    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Err(unsupported_type())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
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
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
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
        unimplemented!()
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.formatter.write_compound_tag(
            &mut self.writer, 
            self.next_name.len() as i16, 
            self.next_name.as_bytes()
        ).map_err(Error::io)?;
        Ok(SerializeCompound {
            ser: self,
        })
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

pub struct SerializeCompound<'a, 'b, W, F> {
    ser: &'a mut Serializer<'b, W, F>,
}

impl<'a, 'b, W, F> ser::SerializeMap for SerializeCompound<'a, 'b, W, F> 
where 
    W: io::Write,
    F: Formatter
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        // let key = to_value(key)?;
        // self.ser.formatter.write_type_id(&mut self.ser.writer, &key).map_err(Error::io)?;
        // key.serialize(MapKeySerializer { ser: self.ser })
        unimplemented!()
    }

    #[inline]
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize
    {
        // let value_conv = to_value(value)?;
        // self.ser.formatter.write_type_id(&mut self.ser.writer, &value_conv).map_err(Error::io)?;
        // value.serialize(&mut *self.ser)
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        // self.ser.formatter.end_compound(&mut self.ser.writer).map_err(Error::io)
        unimplemented!()
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
        T: ?Sized + Serialize 
    {   
        self.ser.next_name = key;
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.formatter.write_end_tag(&mut self.ser.writer).map_err(Error::io)
    }
}

pub trait Formatter {
    #[inline]
    fn write_compound_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8]) -> io::Result<()>
    where
        W: io::Write
    {
        w.write_u8(consts::TYPE_ID_COMPOUND)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)
    }

    #[inline]
    fn write_end_tag<W: ?Sized>(&mut self, w: &mut W) -> io::Result<()> 
    where 
        W: io::Write
    {
        w.write_u8(consts::TYPE_ID_END)
    }

    #[inline]
    fn write_byte_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: i8) -> io::Result<()> 
    where
        W: io::Write 
    {
        w.write_u8(consts::TYPE_ID_BYTE)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i8(value)
    }

    #[inline]
    fn write_short_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: i16) -> io::Result<()> 
    where
        W: io::Write 
    {
        w.write_u8(consts::TYPE_ID_SHORT)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i16::<BigEndian>(value)
    }

    #[inline]
    fn write_int_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: i32) -> io::Result<()> 
    where
        W: io::Write 
    {
        w.write_u8(consts::TYPE_ID_INT)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i32::<BigEndian>(value)
    }

    #[inline]
    fn write_long_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: i64) -> io::Result<()> 
    where
        W: io::Write 
    {
        w.write_u8(consts::TYPE_ID_LONG)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_i64::<BigEndian>(value)
    }

    #[inline]
    fn write_float_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: f32) -> io::Result<()> 
    where
        W: io::Write 
    {
        w.write_u8(consts::TYPE_ID_FLOAT)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_f32::<BigEndian>(value)
    }

    #[inline]
    fn write_double_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: f64) -> io::Result<()> 
    where
        W: io::Write 
    {
        w.write_u8(consts::TYPE_ID_DOUBLE)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
        w.write_f64::<BigEndian>(value)
    }

    #[inline]
    fn write_string_tag<W: ?Sized>(
        &mut self, w: &mut W, 
        name_len: i16, name_bytes: &[u8], 
        string_len: i16, string_bytes: &[u8]
    ) -> io::Result<()> 
    where
        W: io::Write 
    {
        w.write_u8(consts::TYPE_ID_STRING)?;
        w.write_i16::<BigEndian>(name_len)?;
        w.write_all(name_bytes)?;
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
    pub fn new() -> Self {
        Self::with_indent(b"  ")
    }

    pub fn with_indent(indent: &'a [u8]) -> Self {
        TranscriptFormatter {
            current_indent: 0,
            indent,
        }
    }
}

impl Formatter for TranscriptFormatter<'_> {
    fn write_compound_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8]) -> io::Result<()>
    where
        W: io::Write
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
        W: io::Write
    {
        self.current_indent -= 1;
        indent(w, self.current_indent, self.indent)?;
        write!(w, "EndCompound")?;
        if self.current_indent != 0 {
            writeln!(w)?;
        }
        Ok(())
    }
    
    fn write_byte_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: i8) -> io::Result<()> 
    where
        W: io::Write 
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Byte '{}' {}", name, value)?;
        Ok(())
    }
    
    fn write_short_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: i16) -> io::Result<()> 
    where
        W: io::Write 
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Short '{}' {}", name, value)?;
        Ok(())
    }
    
    fn write_int_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: i32) -> io::Result<()> 
    where
        W: io::Write 
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Int '{}' {}", name, value)?;
        Ok(())
    }
    
    fn write_long_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: i64) -> io::Result<()> 
    where
        W: io::Write 
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Long '{}' {}", name, value)?;
        Ok(())
    }

    fn write_float_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: f32) -> io::Result<()> 
    where
        W: io::Write 
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Float '{}' {}", name, value)?;
        Ok(())
    }
    
    fn write_double_tag<W: ?Sized>(&mut self, w: &mut W, name_len: i16, name_bytes: &[u8], value: f64) -> io::Result<()> 
    where
        W: io::Write 
    {
        let _ = name_len;
        let name = String::from_utf8_lossy(name_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "Double '{}' {}", name, value)?;
        Ok(())
    }

    fn write_string_tag<W: ?Sized>(
        &mut self, w: &mut W, 
        name_len: i16, name_bytes: &[u8], 
        string_len: i16, string_bytes: &[u8]
    ) -> io::Result<()> 
    where
        W: io::Write 
    {
        let _ = (name_len, string_len);
        let name = String::from_utf8_lossy(name_bytes);
        let string = String::from_utf8_lossy(string_bytes);
        indent(w, self.current_indent, self.indent)?;
        writeln!(w, "String '{}' {}", name, string)?;
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

pub fn to_writer<'k, 'v, W, T, R>(writer: W, root: R)
    -> Result<()> 
where 
    W: io::Write,
    T: 'v + Serialize + ?Sized,
    R: Into<root::Root<'k, 'v, T>>,
{  
    let root::Root { root_name, value } = root.into();
    let mut ser = Serializer::binary(writer, root_name);
    value.serialize(&mut ser)
}

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

pub fn to_string_transcript<'k, 'v, T, R>(root: R) 
    -> Result<String> 
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
