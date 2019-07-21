use std::io;

pub trait Read<'de> {

}

pub struct IoRead<R> {
    read: R,
}

// pub struct SliceRead<'a> {
//     slice: &'a [u8]
// }

///////////////////////////////////////////////////////////////

impl<R> IoRead<R> 
where
    R: io::Read
{
    pub fn new(read: R) -> Self {
        IoRead { read }
    }

    pub fn into_inner(self) -> R {
        self.read
    }
}

impl<'de, R> Read<'de> for IoRead<R> 
where 
    R: io::Read
{}

///////////////////////////////////////////////////////////////

// impl<'a> SliceRead<'a> {
//     pub fn new(slice: &'a [u8]) -> Self {
//         SliceRead { slice }
//     }
// }

// impl<'a> Read<'a> for SliceRead<'a> {}
