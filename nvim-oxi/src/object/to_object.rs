use std::mem::ManuallyDrop;
use std::string::String as StdString;

use nvim_types::{
    array::Array,
    dictionary::Dictionary,
    object::{Object, ObjectData, ObjectType},
    string::String as NvimString,
    Boolean,
    Float,
    Integer,
};

use crate::lua;

#[derive(thiserror::Error, Debug)]
pub enum ToObjectError {}

pub trait ToObject {
    fn to_obj(self) -> Object;
}

impl<A, R> ToObject for Box<dyn FnMut(A) -> crate::Result<R> + 'static>
where
    A: lua::LuaPoppable + 'static,
    R: lua::LuaPushable + 'static,
{
    fn to_obj(self) -> Object {
        lua::LuaFun::from_fn_mut(self).to_obj()
    }
}

impl ToObject for Object {
    #[inline(always)]
    fn to_obj(self) -> Object {
        self
    }
}

impl ToObject for () {
    fn to_obj(self) -> Object {
        Object::nil()
    }
}

/// Implements `ToObject` for primitive `Copy` types.
macro_rules! to_object_copy {
    ($type:ident, $variant:ident, $data:ident) => {
        impl ToObject for $type {
            fn to_obj(self) -> Object {
                Object {
                    r#type: ObjectType::$variant,
                    data: ObjectData { $data: self },
                }
            }
        }
    };
}

to_object_copy!(Boolean, kObjectTypeBoolean, boolean);
to_object_copy!(Integer, kObjectTypeInteger, integer);
to_object_copy!(Float, kObjectTypeFloat, float);

/// Implements `ToObject` for primitive `Clone` types.
macro_rules! to_object_clone {
    ($type:ident, $variant:ident, $data:ident) => {
        impl ToObject for $type {
            fn to_obj(self) -> Object {
                Object {
                    r#type: ObjectType::$variant,
                    data: ObjectData { $data: ManuallyDrop::new(self) },
                }
            }
        }
    };
}

to_object_clone!(NvimString, kObjectTypeString, string);
to_object_clone!(Array, kObjectTypeArray, array);
to_object_clone!(Dictionary, kObjectTypeDictionary, dictionary);

impl<A, R> ToObject for lua::LuaFun<A, R>
where
    A: lua::LuaPoppable,
    R: lua::LuaPushable,
{
    fn to_obj(self) -> Object {
        Object {
            r#type: ObjectType::kObjectTypeLuaRef,
            data: ObjectData { luaref: self.0 },
        }
    }
}

/// Implements `ToObject` for an integer type convertible to `Integer`.
macro_rules! to_object_int {
    ($type:ident) => {
        impl ToObject for $type {
            fn to_obj(self) -> Object {
                Integer::from(self).to_obj()
            }
        }
    };
}

to_object_int!(i8);
to_object_int!(u8);
to_object_int!(i16);
to_object_int!(u16);
to_object_int!(i32);
to_object_int!(u32);

impl ToObject for f32 {
    fn to_obj(self) -> Object {
        Float::from(self).to_obj()
    }
}

impl ToObject for StdString {
    fn to_obj(self) -> Object {
        NvimString::from(self).to_obj()
    }
}

impl<'a> ToObject for &'a str {
    fn to_obj(self) -> Object {
        NvimString::from(self).to_obj()
    }
}

impl<'a, T: ToObject + Clone> ToObject for std::borrow::Cow<'a, T> {
    fn to_obj(self) -> Object {
        self.into_owned().to_obj()
    }
}

impl<T: ToObject> ToObject for Option<T> {
    fn to_obj(self) -> Object {
        match self {
            Some(t) => t.to_obj(),
            _ => Object::nil(),
        }
    }
}

impl<T: ToObject> ToObject for Box<T> {
    fn to_obj(self) -> Object {
        (*self).to_obj()
    }
}

impl<T: ToObject> ToObject for Vec<T> {
    fn to_obj(self) -> Object {
        self.into_iter()
            .filter_map(|item| {
                let obj = item.to_obj();
                (!obj.is_nil()).then(|| obj)
            })
            .collect::<Array>()
            .to_obj()
    }
}
