use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::io;

pub trait Read<'de> {
    fn read_type_id(&mut self) -> Result<u8>;

    fn read_byte_inner(&mut self) -> Result<i8>;
}

pub struct IoRead<R> {
    inner: R,
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
        IoRead { inner: read }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<'de, R> Read<'de> for IoRead<R>
where
    R: io::Read,
{
    fn read_type_id(&mut self) -> Result<u8> {
        Ok(self.inner.read_u8()?)
    }

    fn read_byte_inner(&mut self) -> Result<i8> {
        Ok(self.inner.read_i8()?)
    }
    
}

///////////////////////////////////////////////////////////////

// impl<'a> SliceRead<'a> {
//     pub fn new(slice: &'a [u8]) -> Self {
//         SliceRead { slice }
//     }
// }

// impl<'a> Read<'a> for SliceRead<'a> {}
