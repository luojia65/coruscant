use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::io;

pub trait Read<'de> {
    fn peek_u8(&mut self) -> Result<Option<u8>>;
}

pub struct IoRead<R> {
    read: R,
    cur_index: usize,
    tmp_byte: Option<u8>,
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
        IoRead {
            read,
            cur_index: 0,
            tmp_byte: None,
        }
    }

    pub fn into_inner(self) -> R {
        self.read
    }
}

impl<'de, R> Read<'de> for IoRead<R>
where
    R: io::Read,
{
    fn peek_u8(&mut self) -> Result<Option<u8>> {
        if let Some(byte) = self.tmp_byte {
            Ok(Some(byte))
        } else {
            let byte = self.read.read_u8()?;
            Ok(Some(byte))
        }
    }
}

///////////////////////////////////////////////////////////////

// impl<'a> SliceRead<'a> {
//     pub fn new(slice: &'a [u8]) -> Self {
//         SliceRead { slice }
//     }
// }

// impl<'a> Read<'a> for SliceRead<'a> {}
