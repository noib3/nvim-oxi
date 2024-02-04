//! Traits for converting between Neovim [`Object`]s and Rust types.

use std::collections::HashMap;

use thiserror::Error as ThisError;

use crate::{
    Array,
    Boolean,
    Dictionary,
    Float,
    Function,
    Integer,
    Object,
    ObjectKind,
};

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error("Was expecting a \"{expected}\" but received a \"{actual}\"")]
    FromWrongType { expected: &'static str, actual: &'static str },

    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[cfg(feature = "serde")]
    #[error(transparent)]
    Serde(#[from] crate::serde::Error),
}

/// Trait implemented for types can be obtained from an [`Object`].
pub trait FromObject: Sized {
    fn from_object(object: Object) -> Result<Self, Error>;
}

/// Trait implemented for types can be converted into an [`Object`].
pub trait ToObject {
    fn to_object(self) -> Result<Object, Error>;
}

impl FromObject for Object {
    fn from_object(obj: Object) -> Result<Self, Error> {
        Ok(obj)
    }
}

impl FromObject for () {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Nil => Ok(()),

            other => Err(Error::FromWrongType {
                expected: "nil",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Boolean {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Boolean => Ok(unsafe { obj.as_boolean_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "bool",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Integer {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Integer
            | ObjectKind::Buffer
            | ObjectKind::Window
            | ObjectKind::TabPage => Ok(unsafe { obj.as_integer_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "integer",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Float {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Float => Ok(unsafe { obj.as_float_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "float",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for crate::String {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::String => Ok(unsafe { obj.into_string_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Array {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Array => Ok(unsafe { obj.into_array_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Dictionary {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Dictionary => Ok(unsafe { obj.into_dict_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl<A, R> FromObject for Function<A, R> {
    fn from_object(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::LuaRef => {
                Ok(Self::from_ref(unsafe { obj.as_luaref_unchecked() }))
            },

            other => Err(Error::FromWrongType {
                expected: "function",
                actual: other.as_static(),
            }),
        }
    }
}

/// Implements `FromObject` for a type that implements `From<Integer>`.
macro_rules! from_int {
    ($integer:ty) => {
        impl FromObject for $integer {
            fn from_object(obj: Object) -> Result<Self, Error> {
                Integer::from_object(obj).map(Into::into)
            }
        }
    };
}

from_int!(i128);

/// Implements `FromObject` for a type that implements `TryFrom<Integer>`.
macro_rules! try_from_int {
    ($integer:ty) => {
        impl FromObject for $integer {
            fn from_object(obj: Object) -> Result<Self, Error> {
                Integer::from_object(obj).and_then(|n| Ok(n.try_into()?))
            }
        }
    };
}

try_from_int!(i8);
try_from_int!(u8);
try_from_int!(i16);
try_from_int!(u16);
try_from_int!(i32);
try_from_int!(u32);
try_from_int!(u64);
try_from_int!(u128);
try_from_int!(isize);
try_from_int!(usize);

impl FromObject for f32 {
    fn from_object(obj: Object) -> Result<Self, Error> {
        Ok(Float::from_object(obj)? as _)
    }
}

impl FromObject for String {
    fn from_object(obj: Object) -> Result<Self, Error> {
        crate::String::from_object(obj)
            .map(|nvim_str| nvim_str.to_string_lossy().into())
    }
}

impl<T> FromObject for Option<T>
where
    T: FromObject,
{
    fn from_object(obj: Object) -> Result<Self, Error> {
        (!obj.is_nil()).then(|| T::from_object(obj)).transpose()
    }
}

impl<T> FromObject for Vec<T>
where
    T: FromObject,
{
    fn from_object(obj: Object) -> Result<Self, Error> {
        Array::from_object(obj)?
            .into_iter()
            .map(FromObject::from_object)
            .collect()
    }
}

impl<T> ToObject for T
where
    T: Into<Object>,
{
    fn to_object(self) -> Result<Object, Error> {
        Ok(self.into())
    }
}

/// Implements `ToObject` for "big integer" types.
macro_rules! bigint_to_obj {
    ($type:ty) => {
        impl ToObject for $type {
            fn to_object(self) -> Result<Object, Error> {
                Ok(i64::try_from(self)?.into())
            }
        }
    };
}

bigint_to_obj!(u64);
bigint_to_obj!(isize);
bigint_to_obj!(usize);
bigint_to_obj!(i128);
bigint_to_obj!(u128);

impl<T> ToObject for Vec<T>
where
    T: ToObject,
{
    fn to_object(self) -> Result<Object, Error> {
        Ok(self
            .into_iter()
            .map(ToObject::to_object)
            .collect::<Result<Array, Error>>()?
            .into())
    }
}

impl<K, V> ToObject for HashMap<K, V>
where
    K: Into<crate::String>,
    V: ToObject,
{
    fn to_object(self) -> Result<Object, Error> {
        self.into_iter()
            .map(|(k, v)| Ok((k, v.to_object()?)))
            .collect::<Result<Dictionary, Error>>()
            .map(Into::into)
    }
}
