use crate::error::{Error, Result};
use byteorder::{BigEndian, ReadBytesExt};
use core::convert::TryInto;
use core::mem::size_of;
use std::borrow::Cow;
use std::io;

pub trait Read<'de> {
    fn index(&self) -> usize;

    fn read_type_id(&mut self) -> Result<u8>;

    fn read_name(&mut self) -> Result<Cow<'de, str>>;

    fn read_length(&mut self) -> Result<i16>;

    fn read_byte_inner(&mut self) -> Result<i8>;

    fn read_short_inner(&mut self) -> Result<i16>;

    fn read_int_inner(&mut self) -> Result<i32>;

    fn read_long_inner(&mut self) -> Result<i64>;

    fn read_float_inner(&mut self) -> Result<f32>;

    fn read_double_inner(&mut self) -> Result<f64>;

    fn read_string_inner(&mut self) -> Result<Cow<'de, str>>;
}

pub struct IoRead<R> {
    inner: R,
    index: usize,
}

pub struct SliceRead<'a> {
    inner: &'a [u8],
    original_inner: &'a [u8],
    index: usize,
}

///////////////////////////////////////////////////////////////

impl<R> IoRead<R>
where
    R: io::Read,
{
    pub fn new(read: R) -> Self {
        IoRead {
            inner: read,
            index: 0,
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<'de, R> Read<'de> for IoRead<R>
where
    R: io::Read,
{
    fn index(&self) -> usize {
        self.index
    }

    fn read_type_id(&mut self) -> Result<u8> {
        let value = self
            .inner
            .read_u8()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<u8>();
        Ok(value)
    }

    fn read_name(&mut self) -> Result<Cow<'de, str>> {
        self.read_string_inner()
    }

    fn read_length(&mut self) -> Result<i16> {
        self.read_short_inner()
    }

    fn read_byte_inner(&mut self) -> Result<i8> {
        let value = self
            .inner
            .read_i8()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<i8>();
        Ok(value)
    }

    fn read_short_inner(&mut self) -> Result<i16> {
        let value = self
            .inner
            .read_i16::<BigEndian>()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<i16>();
        Ok(value)
    }

    fn read_int_inner(&mut self) -> Result<i32> {
        let value = self
            .inner
            .read_i32::<BigEndian>()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<i32>();
        Ok(value)
    }

    fn read_long_inner(&mut self) -> Result<i64> {
        let value = self
            .inner
            .read_i64::<BigEndian>()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<i64>();
        Ok(value)
    }

    fn read_float_inner(&mut self) -> Result<f32> {
        let value = self
            .inner
            .read_f32::<BigEndian>()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<f32>();
        Ok(value)
    }

    fn read_double_inner(&mut self) -> Result<f64> {
        let value = self
            .inner
            .read_f64::<BigEndian>()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<f64>();
        Ok(value)
    }

    fn read_string_inner(&mut self) -> Result<Cow<'de, str>> {
        let len = self.read_length()?;
        if len < 0 {
            return Err(Error::invalid_len_at(len, self.index));
        }
        let len = len as usize;
        let mut buf = vec![0; len];
        self.inner
            .read_exact(&mut buf)
            .map_err(|e| Error::io_at(e, self.index))?;
        let string = String::from_utf8(buf).map_err(|_| Error::utf8_at(self.index))?;
        self.index += len;
        let ans = Cow::Owned(string);
        Ok(ans)
    }
}

///////////////////////////////////////////////////////////////

impl<'a> SliceRead<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        SliceRead {
            inner: slice,
            original_inner: slice,
            index: 0,
        }
    }

    pub fn into_inner(self) -> &'a [u8] {
        self.original_inner
    }

    #[inline]
    fn check_eof(&self, remain_at_least: usize) -> Result<()> {
        if self.index > self.original_inner.len() - remain_at_least {
            return Err(Error::slice_eof());
        }
        Ok(())
    }
}

// todo: enhance transmutions

impl<'a> Read<'a> for SliceRead<'a> {
    fn index(&self) -> usize {
        self.index
    }

    fn read_type_id(&mut self) -> Result<u8> {
        self.check_eof(size_of::<u8>())?;
        if let Some((ans, rest)) = self.inner.split_first() {
            self.inner = rest;
            self.index += size_of::<u8>();
            Ok(*ans)
        } else {
            return Err(Error::slice_eof());
        }
    }

    fn read_name(&mut self) -> Result<Cow<'a, str>> {
        self.read_string_inner()
    }

    fn read_length(&mut self) -> Result<i16> {
        self.read_short_inner()
    }

    fn read_byte_inner(&mut self) -> Result<i8> {
        if let Some((ans, rest)) = self.inner.split_first() {
            self.inner = rest;
            self.index += size_of::<i8>();
            Ok(i8::from_be_bytes([*ans]))
        } else {
            return Err(Error::slice_eof());
        }
    }

    fn read_short_inner(&mut self) -> Result<i16> {
        self.check_eof(size_of::<i16>())?;
        let (bytes, rest) = self.inner.split_at(size_of::<i16>());
        self.inner = rest;
        self.index += size_of::<i16>();
        let value = i16::from_be_bytes(bytes.try_into().unwrap());
        Ok(value)
    }

    fn read_int_inner(&mut self) -> Result<i32> {
        self.check_eof(size_of::<i32>())?;
        let (bytes, rest) = self.inner.split_at(size_of::<i32>());
        self.inner = rest;
        self.index += size_of::<i32>();
        let value = i32::from_be_bytes(bytes.try_into().unwrap());
        Ok(value)
    }

    fn read_long_inner(&mut self) -> Result<i64> {
        self.check_eof(size_of::<i64>())?;
        let (bytes, rest) = self.inner.split_at(size_of::<i64>());
        self.inner = rest;
        self.index += size_of::<i64>();
        let value = i64::from_be_bytes(bytes.try_into().unwrap());
        Ok(value)
    }

    fn read_float_inner(&mut self) -> Result<f32> {
        self.check_eof(size_of::<f32>())?;
        let (bytes, rest) = self.inner.split_at(size_of::<u32>());
        self.inner = rest;
        self.index += size_of::<f32>();
        let value = u32::from_be_bytes(bytes.try_into().unwrap());
        Ok(f32::from_bits(value))
    }

    fn read_double_inner(&mut self) -> Result<f64> {
        self.check_eof(size_of::<f64>())?;
        let (bytes, rest) = self.inner.split_at(size_of::<u64>());
        self.inner = rest;
        self.index += size_of::<f64>();
        let value = u64::from_be_bytes(bytes.try_into().unwrap());
        Ok(f64::from_bits(value))
    }

    fn read_string_inner(&mut self) -> Result<Cow<'a, str>> {
        let len = self.read_length()?;
        if len < 0 {
            return Err(Error::invalid_len_at(len, self.index));
        }
        let len = len as usize;
        let (bytes, rest) = self.inner.split_at(len);
        let borrowed = std::str::from_utf8(bytes).map_err(|_| Error::utf8_at(self.index))?;
        self.index += len;
        self.inner = rest;
        Ok(Cow::Borrowed(borrowed))
    }
}
