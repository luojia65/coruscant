use core::fmt;

use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use serde::de::{self, Deserializer, Error, Unexpected};
// use coruscant_nbt::to_vec;

#[derive(Serialize, Debug)]
pub struct PlayerDat {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    #[serde(rename = "Dimension")]
    dimension: Dimension,
}

#[derive(Debug)]
pub enum Dimension {
    Overworld,
    Nether,
    TheEnd,
}

impl Serialize for Dimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let number = match *self {
            Dimension::Overworld => 0,
            Dimension::Nether => -1, 
            Dimension::TheEnd => 1,
        };
        serializer.serialize_i32(number)
    }
}

impl<'de> Deserialize<'de> for Dimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Dimension;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an 32-bit integer 1, 0 or -1")
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(Dimension::Overworld),
                    -1 => Ok(Dimension::Nether),
                    1 => Ok(Dimension::TheEnd),
                    v => Err(Error::invalid_value(Unexpected::Signed(v.into()), &"1, 0 or -1"))
                }
            }
        }

        deserializer.deserialize_i32(Visitor)
    }
}

fn main() -> coruscant_nbt::Result<()> {
    let dat = PlayerDat {
        data_version: 19133,
        dimension: Dimension::Nether,
    };
    let v = coruscant_nbt::to_vec(&dat)?;
    println!("{:?}", v);
    Ok(())
}
