pub struct Deserializer<'de> {
    src: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(src: &'de [u8]) -> Deserializer<'de> {
        Deserializer { src }
    }
}

