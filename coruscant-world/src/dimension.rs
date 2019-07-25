use core::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Dimension {
    Overworld,
    Nether,
    TheEnd,
}

impl serde::Serialize for Dimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let number: i32 = match *self {
            Dimension::Overworld => 0,
            Dimension::Nether => -1,
            Dimension::TheEnd => 1,
        };
        serializer.serialize_i32(number)
    }
}

impl<'de> serde::Deserialize<'de> for Dimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i32(DimensionVisitor)
    }
}

struct DimensionVisitor;

impl<'de> serde::de::Visitor<'de> for DimensionVisitor {
    type Value = Dimension;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer 1, 0, or -1")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let ans = match value {
            0 => Dimension::Overworld,
            -1 => Dimension::Nether,
            1 => Dimension::TheEnd,
            invalid => {
                return Err(E::invalid_value(
                    serde::de::Unexpected::Signed(invalid.into()),
                    &self,
                ))
            }
        };
        Ok(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::Dimension;
    use coruscant_nbt::{to_vec, from_slice};

    #[test]
    fn dimension_serialize() {
        let cond = [
            (("OW", &Dimension::Overworld), vec![3u8, 0, 2, 79, 87, 0, 0, 0, 0]),
            (("N", &Dimension::Nether), vec![3, 0, 1, 78, 255, 255, 255, 255]),
            (("E", &Dimension::TheEnd), vec![3, 0, 1, 69, 0, 0, 0, 1]),
        ];
        for (pair, slice) in &cond {
            let ans = to_vec(*pair).expect("serialize data to NBT vec");
            assert_eq!(ans, *slice);
        }
    }

    #[test]
    fn dimension_deserialize() {
        let cond = [
            (("OW", &Dimension::Overworld), vec![3u8, 0, 2, 79, 87, 0, 0, 0, 0]),
            (("N", &Dimension::Nether), vec![3, 0, 1, 78, 255, 255, 255, 255]),
            (("E", &Dimension::TheEnd), vec![3, 0, 1, 69, 0, 0, 0, 1]),
        ];
        for ((_name, data), slice) in &cond {
            let ans: Dimension = from_slice(slice)
                .expect("deserialize data from NBT slice");
            assert_eq!(ans, **data);
        }
    }

    #[test]
    #[should_panic]
    fn dimension_deserialize_invalid() {
        // int value is changed to 2
        let slice = vec![3u8, 0, 1, 69, 0, 0, 0, 2]; 
        let _: Dimension = from_slice(&slice)
            .expect("deserialize data from NBT slice");
    }
}
