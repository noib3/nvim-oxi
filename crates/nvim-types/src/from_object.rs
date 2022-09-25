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

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error("Was expecting a \"{expected}\" but received a \"{actual}\"")]
    WrongType { expected: &'static str, actual: &'static str },

    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[cfg(feature = "serde")]
    #[error(transparent)]
    Deserialize(#[from] crate::serde::Error),
}

pub trait FromObject: Sized {
    fn from_obj(obj: Object) -> Result<Self>;
}

impl FromObject for () {
    fn from_obj(obj: Object) -> Result<Self> {
        match obj.kind() {
            ObjectKind::Nil => Ok(()),

            other => Err(Error::WrongType {
                expected: "nil",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Boolean {
    fn from_obj(obj: Object) -> Result<Self> {
        match obj.kind() {
            ObjectKind::Boolean => Ok(unsafe { obj.as_boolean_unchecked() }),

            other => Err(Error::WrongType {
                expected: "bool",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Integer {
    fn from_obj(obj: Object) -> Result<Self> {
        match obj.kind() {
            ObjectKind::Integer => Ok(unsafe { obj.as_integer_unchecked() }),

            other => Err(Error::WrongType {
                expected: "integer",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Float {
    fn from_obj(obj: Object) -> Result<Self> {
        match obj.kind() {
            ObjectKind::Float => Ok(unsafe { obj.as_float_unchecked() }),

            other => Err(Error::WrongType {
                expected: "float",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for crate::String {
    fn from_obj(obj: Object) -> Result<Self> {
        match obj.kind() {
            ObjectKind::String => Ok(unsafe { obj.into_string_unchecked() }),

            other => Err(Error::WrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Array {
    fn from_obj(obj: Object) -> Result<Self> {
        match obj.kind() {
            ObjectKind::Array => Ok(unsafe { obj.into_array_unchecked() }),

            other => Err(Error::WrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl FromObject for Dictionary {
    fn from_obj(obj: Object) -> Result<Self> {
        match obj.kind() {
            ObjectKind::Dictionary => Ok(unsafe { obj.into_dict_unchecked() }),

            other => Err(Error::WrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl<A, R> FromObject for Function<A, R> {
    fn from_obj(obj: Object) -> Result<Self> {
        match obj.kind() {
            ObjectKind::LuaRef => {
                Ok(Self::from_ref(unsafe { obj.as_luaref_unchecked() }))
            },

            other => Err(Error::WrongType {
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
            fn from_obj(obj: Object) -> Result<Self> {
                Integer::from_obj(obj).map(Into::into)
            }
        }
    };
}

from_int!(i128);

/// Implements `FromObject` for a type that implements `TryFrom<Integer>`.
macro_rules! try_from_int {
    ($integer:ty) => {
        impl FromObject for $integer {
            fn from_obj(obj: Object) -> Result<Self> {
                Integer::from_obj(obj).and_then(|n| Ok(n.try_into()?))
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
    fn from_obj(obj: Object) -> Result<Self> {
        Ok(Float::from_obj(obj)? as _)
    }
}

impl FromObject for String {
    fn from_obj(obj: Object) -> Result<Self> {
        crate::String::from_obj(obj)
            .and_then(|nvim_str| Ok(nvim_str.into_string()?))
    }
}

impl<T: FromObject> FromObject for Option<T> {
    fn from_obj(obj: Object) -> Result<Self> {
        (!obj.is_nil()).then(|| T::from_obj(obj)).transpose()
    }
}

impl<T: FromObject> FromObject for Vec<T> {
    fn from_obj(obj: Object) -> Result<Self> {
        Array::from_obj(obj)?.into_iter().map(FromObject::from_obj).collect()
    }
}
