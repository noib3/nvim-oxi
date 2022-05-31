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
macro_rules! impl_prim_copy {
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

impl_prim_copy!(Boolean, kObjectTypeBoolean, boolean);
impl_prim_copy!(Integer, kObjectTypeInteger, integer);
impl_prim_copy!(Float, kObjectTypeFloat, float);

/// Implements `ToObject` for primitive `Clone` types.
macro_rules! impl_prim_clone {
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

impl_prim_clone!(NvimString, kObjectTypeString, string);
impl_prim_clone!(Array, kObjectTypeArray, array);
impl_prim_clone!(Dictionary, kObjectTypeDictionary, dictionary);

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
macro_rules! impl_int {
    ($type:ident) => {
        impl ToObject for $type {
            fn to_obj(self) -> Object {
                Integer::from(self).to_obj()
            }
        }
    };
}

impl_int!(i8);
impl_int!(u8);
impl_int!(i16);
impl_int!(u16);
impl_int!(i32);
impl_int!(u32);

impl ToObject for f32 {
    fn to_obj(self) -> Object {
        Float::from(self).to_obj()
    }
}

// macro_rules! impl_closure {
//     ($fn_trait:ident, $from_fn:ident) => {
//         impl<A, R, F> ToObject for F
//         where
//             A: lua::LuaPoppable + 'static,
//             R: lua::LuaPushable + 'static,
//             F: $fn_trait(A) -> crate::Result<R> + 'static,
//         {
//             fn to_obj(self) -> Object {
//                 lua::LuaFun::$from_fn(self).to_obj()
//             }
//         }
//     };
// }

// impl_closure!(FnMut, from_fn_mut);
// impl_closure!(FnOnce, from_fn_once);

macro_rules! impl_boxed_closure {
    ($fn_trait:ident, $from_fn:ident) => {
        impl<A, R> ToObject
            for Box<dyn $fn_trait(A) -> crate::Result<R> + 'static>
        where
            A: lua::LuaPoppable + 'static,
            R: lua::LuaPushable + 'static,
        {
            fn to_obj(self) -> Object {
                lua::LuaFun::$from_fn(self).to_obj()
            }
        }
    };
}

impl_boxed_closure!(FnMut, from_fn_mut);
impl_boxed_closure!(FnOnce, from_fn_once);

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
