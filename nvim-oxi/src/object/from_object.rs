use std::error::Error as StdError;
use std::mem::ManuallyDrop;
use std::result::Result as StdResult;
use std::string::String as StdString;

use nvim_types::{
    array::Array,
    dictionary::Dictionary,
    object::{Object, ObjectType},
    string::String as NvimString,
    Boolean,
    Float,
    Integer,
};

use crate::lua::LuaRef;

#[derive(thiserror::Error, Debug)]
pub enum FromObjectError {
    #[error("Was expecting a \"{expected:?}\", got \"{actual:?}\" instead")]
    Primitive { expected: ObjectType, actual: ObjectType },

    #[error("{0}")]
    Secondary(String),
}

impl FromObjectError {
    pub fn secondary<S, Err>(
        primitive: ObjectType,
        secondary: S,
        err: Err,
    ) -> Self
    where
        S: std::fmt::Display,
        Err: StdError,
    {
        Self::Secondary(format!(
            "Error converting {secondary} into {primitive:?}: {err}"
        ))
    }
}

pub trait FromObject: Sized {
    fn from_obj(obj: Object) -> StdResult<Self, FromObjectError>;
}

impl FromObject for Object {
    fn from_obj(obj: Object) -> StdResult<Self, FromObjectError> {
        Ok(obj)
    }
}

impl FromObject for () {
    fn from_obj(obj: Object) -> StdResult<Self, FromObjectError> {
        (matches!(obj.r#type, ObjectType::kObjectTypeNil))
            .then(|| ())
            .ok_or_else(|| FromObjectError::Primitive {
                expected: ObjectType::kObjectTypeNil,
                actual: obj.r#type,
            })
    }
}

/// Implements `FromObject` for primitive `Copy` types.
macro_rules! from_obj_copy {
    ($type:ident, $variant:ident, $data:ident) => {
        impl FromObject for $type {
            fn from_obj(obj: Object) -> StdResult<Self, FromObjectError> {
                (matches!(obj.r#type, ObjectType::$variant))
                    .then(|| unsafe { obj.data.$data })
                    .ok_or_else(|| FromObjectError::Primitive {
                        expected: ObjectType::$variant,
                        actual: obj.r#type,
                    })
            }
        }
    };
}

from_obj_copy!(Boolean, kObjectTypeBoolean, boolean);
from_obj_copy!(Integer, kObjectTypeInteger, integer);
from_obj_copy!(Float, kObjectTypeFloat, float);

/// Implements `FromObject` for primitive `ManuallyDrop` types.
macro_rules! from_obj_drop {
    ($type:ident, $variant:ident, $data:ident) => {
        impl FromObject for $type {
            fn from_obj(obj: Object) -> StdResult<Self, FromObjectError> {
                let tp = obj.r#type;
                (matches!(tp, ObjectType::$variant))
                    .then(|| unsafe {
                        ManuallyDrop::into_inner(obj.data.$data)
                    })
                    .ok_or_else(|| FromObjectError::Primitive {
                        expected: ObjectType::$variant,
                        actual: tp,
                    })
            }
        }
    };
}

from_obj_drop!(NvimString, kObjectTypeString, string);
from_obj_drop!(Array, kObjectTypeArray, array);
from_obj_drop!(Dictionary, kObjectTypeDictionary, dictionary);

impl FromObject for LuaRef {
    fn from_obj(obj: Object) -> StdResult<Self, FromObjectError> {
        (matches!(obj.r#type, ObjectType::kObjectTypeLuaRef))
            .then(|| LuaRef::from(unsafe { obj.data.luaref }))
            .ok_or_else(|| FromObjectError::Primitive {
                expected: ObjectType::kObjectTypeLuaRef,
                actual: obj.r#type,
            })
    }
}

impl FromObject for usize {
    fn from_obj(_obj: Object) -> StdResult<Self, FromObjectError> {
        todo!()
    }
}
