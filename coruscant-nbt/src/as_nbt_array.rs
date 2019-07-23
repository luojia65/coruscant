use serde::Serialize;

#[doc(hidden)]
pub fn serialize<'a, T, S>(value: &T, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    T: serde::Serialize,
    S: serde::Serializer,
{
    let value = __WrapAsArray(value);
    value.serialize(serializer)
}

#[doc(hidden)]
#[derive(serde::Serialize)]
pub struct __WrapAsArray<T>(pub T);

pub(crate) const TOKEN_AS_ARRAY: &'static str = "__WrapAsArray";
