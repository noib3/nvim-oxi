use std::borrow::Cow;
use std::collections::HashMap;

use thiserror::Error as ThisError;

use crate::{Array, Dictionary, Function, Object};

pub type ToObjectResult = std::result::Result<Object, Error>;

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[cfg(feature = "serde")]
    #[error(transparent)]
    Serialize(#[from] crate::serde::Error),
}

pub trait ToObject {
    fn to_obj(self) -> Result<Object, Error>;
}

/// Implements `ToObject` for `Into<Object>` types.
macro_rules! impl_into {
    ($type:ty) => {
        impl ToObject for $type {
            fn to_obj(self) -> Result<Object, Error> {
                Ok(self.into())
            }
        }
    };
}

impl_into!(Object);
impl_into!(());
impl_into!(bool);
impl_into!(i8);
impl_into!(u8);
impl_into!(i16);
impl_into!(u16);
impl_into!(i32);
impl_into!(u32);
impl_into!(i64);
impl_into!(f32);
impl_into!(f64);
impl_into!(String);
impl_into!(crate::String);
impl_into!(Array);
impl_into!(Dictionary);

/// Implements `ToObject` for "big integer" types.
macro_rules! impl_bigint {
    ($type:ty) => {
        impl ToObject for $type {
            fn to_obj(self) -> Result<Object, Error> {
                Ok(i64::try_from(self)?.into())
            }
        }
    };
}

impl_bigint!(u64);
impl_bigint!(isize);
impl_bigint!(usize);
impl_bigint!(i128);
impl_bigint!(u128);

impl<A, R> ToObject for Function<A, R> {
    fn to_obj(self) -> Result<Object, Error> {
        Ok(self.into())
    }
}

impl ToObject for &str {
    fn to_obj(self) -> Result<Object, Error> {
        Ok(crate::String::from(self).into())
    }
}

impl ToObject for Cow<'_, str> {
    fn to_obj(self) -> Result<Object, Error> {
        Ok(crate::String::from(self).into())
    }
}

impl<T: ToObject> ToObject for Option<T> {
    fn to_obj(self) -> Result<Object, Error> {
        self.map(ToObject::to_obj).transpose().map(Option::unwrap_or_default)
    }
}

impl<T: ToObject> ToObject for Vec<T> {
    fn to_obj(self) -> Result<Object, Error> {
        Ok(self
            .into_iter()
            .map(ToObject::to_obj)
            .collect::<Result<Array, Error>>()?
            .into())
    }
}

impl<K, V> ToObject for HashMap<K, V>
where
    K: Into<crate::String>,
    V: ToObject,
{
    fn to_obj(self) -> Result<Object, Error> {
        self.into_iter()
            .map(|(k, v)| Ok((k, v.to_obj()?)))
            .collect::<Result<Dictionary, Error>>()
            .map(Into::into)
    }
}
