use core::fmt;

use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use serde::de::{self, Deserializer, Error, Unexpected};

#[derive(Serialize, Debug)]
#[serde(rename = "Player")]
pub struct PlayerDat {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    #[serde(rename = "Dimension")]
    dimension: Dimension,
    #[serde(rename = "abilities")]
    abilities: Abilities,
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

#[derive(Serialize, Debug)]
pub struct Abilities {
    #[serde(rename = "walkSpeed")]
    walk_speed: f32,
    #[serde(rename = "flySpeed")]
    fly_speed: f32,
    #[serde(rename = "mayfly")]
    may_fly: bool,
    #[serde(rename = "flying")]
    flying: bool,
    #[serde(rename = "invulnerable")]
    invulnerable: bool,
    #[serde(rename = "mayBuild")]
    may_build: bool,
    #[serde(rename = "instabuild")]
    instant_build: bool,
}

#[derive(Serialize, Debug)]
pub struct ItemStructure {
    #[serde(rename = "Count")]
    count: i8,
    #[serde(rename = "Slot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    slot_id: Option<i8>,
    #[serde(rename = "id")]
    item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)] 
    #[serde(rename = "tag")]
    item_tag: Option<ItemTag>,
}

#[derive(Serialize, Debug)]
pub enum ItemTag {

}

fn main() -> coruscant_nbt::Result<()> {
    let dat = PlayerDat {
        data_version: 19133,
        dimension: Dimension::Nether,
        abilities: Abilities {
            walk_speed: 0.1,
            fly_speed: 0.05,
            may_fly: false,
            flying: false,
            invulnerable: false,
            may_build: true,
            instant_build: false,
        },
    };
    let s = coruscant_nbt::to_string_transcript(&dat)?;
    println!("{}", s);
    let v = coruscant_nbt::to_vec(&dat)?;
    println!("{:?}", v);
    Ok(())
}
