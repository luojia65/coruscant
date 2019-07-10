mod ser;

use core::fmt;
use crate::consts;
use crate::error::Result;
use crate::map::Map;

#[derive(Clone, PartialEq)]
pub enum Value {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String), // valid UTF-8
    List(Vec<Value>),
    Compound(Map<String, Value>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Value {
    pub(crate) fn type_id(&self) -> u8 {
        match *self {
            Value::Byte(_) => consts::TYPE_ID_BYTE,
            Value::Short(_) => consts::TYPE_ID_SHORT,
            Value::Int(_) => consts::TYPE_ID_INT,
            Value::Long(_) => consts::TYPE_ID_LONG,
            Value::Float(_) => consts::TYPE_ID_FLOAT,
            Value::Double(_) => consts::TYPE_ID_DOUBLE,
            Value::ByteArray(_) => consts::TYPE_ID_BYTE_ARRAY,
            Value::String(_) => consts::TYPE_ID_STRING,
            Value::List(_) => consts::TYPE_ID_LIST,
            Value::Compound(_) => consts::TYPE_ID_COMPOUND,
            Value::IntArray(_) => consts::TYPE_ID_INT_ARRAY,
            Value::LongArray(_) => consts::TYPE_ID_LONG_ARRAY,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Byte(v) => f.debug_tuple("Byte").field(&v).finish(),
            Value::Short(v) => f.debug_tuple("Short").field(&v).finish(),
            Value::Int(v) => f.debug_tuple("Int").field(&v).finish(),
            Value::Long(v) => f.debug_tuple("Long").field(&v).finish(),
            Value::Float(v) => f.debug_tuple("Float").field(&v).finish(),
            Value::Double(v) => f.debug_tuple("Double").field(&v).finish(),
            Value::ByteArray(v) => f.debug_tuple("ByteArray").field(&v).finish(),
            Value::String(v) => f.debug_tuple("String").field(&v).finish(),
            Value::List(v) => f.debug_tuple("List").field(&v).finish(),
            Value::Compound(v) => f.debug_tuple("Compound").field(&v).finish(),
            Value::IntArray(v) => f.debug_tuple("IntArray").field(&v).finish(),
            Value::LongArray(v) => f.debug_tuple("LongArray").field(&v).finish(),
        }
    }
}

// impl Default for Value {}
// NBT does not have an null value

pub fn to_value<T>(value: T) -> Result<Value> 
where T: serde::Serialize 
{
    value.serialize(ser::Serializer)
}