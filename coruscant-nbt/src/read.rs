use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::io;

pub trait Read<'de> {
    fn peek_u8(&mut self) -> Result<Option<u8>>;

    fn read_byte_inner(&mut self) -> Result<i8>;
}

pub struct IoRead<R> {
    read: R,
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
            let mut buf = [0];
            loop {
                return match self.read.read(&mut buf) {
                    Ok(0) => Ok(None),
                    Ok(..) => {
                        self.tmp_byte = Some(buf[0]);
                        Ok(Some(buf[0]))
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => Err(e)?,
                };
            }
        }
    }

    fn read_byte_inner(&mut self) -> Result<i8> {
        if let Some(byte) = self.tmp_byte.take() {
            Ok(byte as i8)
        } else {
            Ok(self.read.read_i8()?)
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
