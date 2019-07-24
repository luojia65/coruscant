use crate::error::{Error, Result};
use byteorder::{BigEndian, ReadBytesExt};
use std::io;
use std::borrow::Cow;
use core::mem::size_of;

pub trait Read<'de> {
    fn index(&self) -> usize;
    
    fn read_type_id(&mut self) -> Result<u8>;

    fn read_name(&mut self) -> Result<Cow<'de, str>>;

    fn read_byte_inner(&mut self) -> Result<i8>;

    fn read_short_inner(&mut self) -> Result<i16>;
}

pub struct IoRead<R> {
    inner: R,
    index: usize,
}

// pub struct SliceRead<'a> {
//     slice: &'a [u8]
// }

///////////////////////////////////////////////////////////////

impl<R> IoRead<R>
where
    R: io::Read,
{
    pub fn new(read: R) -> Self {
        IoRead { inner: read, index: 0 }
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
        let value = self.inner.read_u8()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<u8>();
        Ok(value)
    }

    fn read_name(&mut self) -> Result<Cow<'de, str>> {
        let len = self.inner.read_i16::<BigEndian>()
            .map_err(|e| Error::io_at(e, self.index))? as usize;
        self.index += size_of::<i16>();
        let mut buf = vec![0; len];
        self.inner.read_exact(&mut buf)
            .map_err(|e| Error::io_at(e, self.index))?;
        let string = String::from_utf8(buf)
            .map_err(|_| Error::utf8_at(self.index))?;
        self.index += len;
        let ans = Cow::Owned(string);
        Ok(ans)
    }

    fn read_byte_inner(&mut self) -> Result<i8> {
        let value = self.inner.read_i8()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<i8>();
        Ok(value)
    }

    fn read_short_inner(&mut self) -> Result<i16> {
        let value = self.inner.read_i16::<BigEndian>()
            .map_err(|e| Error::io_at(e, self.index))?;
        self.index += size_of::<i16>();
        Ok(value)
    }
    
}

///////////////////////////////////////////////////////////////

// impl<'a> SliceRead<'a> {
//     pub fn new(slice: &'a [u8]) -> Self {
//         SliceRead { slice }
//     }
// }

// impl<'a> Read<'a> for SliceRead<'a> {}
