use std::string::String as StdString;

use nvim_types::{Array, Object, String as NvimString};

use crate::lua;
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

impl<'a> ToObject for &'a str {
    fn to_obj(self) -> Result<Object> {
        Ok(NvimString::from(self).into())
    }
}

impl<'a> ToObject for std::borrow::Cow<'a, str> {
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

// Damn I wish I could do this.
//
// macro_rules! impl_closure {
//     ($fn_trait:ident, $from_fn:ident) => {
//         impl<A, R, F> ToObject for F
//         where
//             A: lua::LuaPoppable + 'static,
//             R: lua::LuaPushable + 'static,
//             F: $fn_trait(A) -> Result<R> + 'static,
//         {
//             fn to_obj(self) -> Result<Object> {
//                 lua::LuaFun::$from_fn(self).to_obj()
//             }
//         }
//     };
// }

// impl_closure!(Fn, from_fn);
// impl_closure!(FnMut, from_fn_mut);
// impl_closure!(FnOnce, from_fn_once);

macro_rules! impl_boxed_closure {
    ($fn_trait:ident, $from_fn:ident) => {
        impl<A, R> ToObject for Box<dyn $fn_trait(A) -> Result<R> + 'static>
        where
            A: lua::LuaPoppable + 'static,
            R: lua::LuaPushable + 'static,
        {
            fn to_obj(self) -> Result<Object> {
                lua::LuaFun::$from_fn(self).to_obj()
            }
        }
    };
}

impl_boxed_closure!(Fn, from_fn);
impl_boxed_closure!(FnMut, from_fn_mut);
impl_boxed_closure!(FnOnce, from_fn_once);
