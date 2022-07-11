use std::string::String as StdString;

use nvim_types::{Array, Dictionary, Object, String as NvimString};

use crate::Result;

pub trait ToObject {
    fn to_obj(self) -> Result<Object>;
}

/// Implements `ToObject` for `Into<Object>` types.
macro_rules! impl_into {
    ($type:ty) => {
        impl ToObject for $type {
            fn to_obj(self) -> Result<Object> {
                Ok(self.into())
            }
        }
    };
}

impl_into!(());
impl_into!(bool);
impl_into!(i8);
impl_into!(u8);
impl_into!(i16);
impl_into!(u16);
impl_into!(i32);
impl_into!(u32);
impl_into!(i64);
impl_into!(f64);
impl_into!(StdString);
impl_into!(NvimString);
impl_into!(Array);
impl_into!(Dictionary);

/// Implements `ToObject` for "big integer" types.
macro_rules! impl_bigint {
    ($type:ty) => {
        impl ToObject for $type {
            fn to_obj(self) -> Result<Object> {
                Ok(i64::try_from(self)?.into())
            }
        }
    };
}

impl_bigint!(u64);
impl_bigint!(i128);
impl_bigint!(u128);
impl_bigint!(isize);
impl_bigint!(usize);

impl ToObject for &str {
    fn to_obj(self) -> Result<Object> {
        Ok(NvimString::from(self).into())
    }
}

impl ToObject for std::borrow::Cow<'_, str> {
    fn to_obj(self) -> Result<Object> {
        Ok(NvimString::from(self).into())
    }
}

impl<T> ToObject for Option<T>
where
    T: ToObject,
{
    fn to_obj(self) -> Result<Object> {
        self.map(ToObject::to_obj).transpose().map(Option::unwrap_or_default)
    }
}

impl<T> ToObject for Vec<T>
where
    T: ToObject,
{
    fn to_obj(self) -> Result<Object> {
        Ok(self
            .into_iter()
            .map(ToObject::to_obj)
            .collect::<Result<Array>>()?
            .into())
    }
}

// impl<K, V, I> ToObject for I
// where
//     K: Into<NvimString>,
//     V: ToObject,
//     I: IntoIterator<Item = (NvimString, V)>,
// {
//     fn to_obj(iter: I) -> Object {
//         todo!()
//     }
// }

impl<K, V> ToObject for std::collections::HashMap<K, V>
where
    K: Into<NvimString>,
    V: ToObject,
{
    fn to_obj(self) -> Result<Object> {
        self.into_iter()
            .map(|(k, v)| Ok((k, v.to_obj()?)))
            .collect::<Result<Dictionary>>()
            .map(Into::into)
    }
}
