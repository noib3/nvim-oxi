use std::iter::FusedIterator;

use oxi_luajit::{Poppable, Pushable};
use oxi_types::{Array, Function, LuaRef, Object};

/// A super trait of most common traits implemented on iterators.
pub trait SuperIterator<I>:
    Iterator<Item = I> + ExactSizeIterator + DoubleEndedIterator + FusedIterator
{
}

impl<I, T> SuperIterator<I> for T where
    T: Iterator<Item = I>
        + ExactSizeIterator
        + DoubleEndedIterator
        + FusedIterator
{
}

macro_rules! impl_into {
    ($trait:ident, $type:ty) => {
        impl $trait for $type {
            fn to_object(self) -> Object {
                self.into()
            }
        }
    };
}

/// A trait implemented by strings and integers.
pub trait StringOrInt {
    fn to_object(self) -> Object;
}

impl_into!(StringOrInt, &str);
impl_into!(StringOrInt, String);
impl_into!(StringOrInt, i8);
impl_into!(StringOrInt, u8);
impl_into!(StringOrInt, i16);
impl_into!(StringOrInt, u16);
impl_into!(StringOrInt, i32);
impl_into!(StringOrInt, u32);
impl_into!(StringOrInt, i64);

/// A trait implemented by strings and list of strings.
pub trait StringOrListOfStrings {
    fn to_object(self) -> Object;
}

impl_into!(StringOrListOfStrings, &str);
impl_into!(StringOrListOfStrings, String);

// Here I'd like to use `IntoIterator` instead of `Vec`, but without
// specilization that'd cause conflicting impls.
impl<S: Into<String>> StringOrListOfStrings for Vec<S> {
    #[inline]
    fn to_object(self) -> Object {
        Array::from_iter(self.into_iter().map(Into::into)).into()
    }
}

/// A trait implemented by closures and [`Function`]s.
pub trait ToFunction<A, R> {
    fn into_luaref(self) -> LuaRef;
}

impl<A, R, F> ToFunction<A, R> for F
where
    A: Poppable,
    R: Pushable,
    F: FnMut(A) -> crate::Result<R> + 'static,
{
    #[inline]
    fn into_luaref(self) -> LuaRef {
        Function::from_fn_mut(self).lua_ref()
    }
}

impl<A, R> ToFunction<A, R> for Function<A, R> {
    #[inline]
    fn into_luaref(self) -> LuaRef {
        self.lua_ref()
    }
}

/// A trait implemented by closures, [`Function`]s and strings.
pub trait StringOrFunction<A, R> {
    fn to_object(self) -> Object;
}

impl<A, R> StringOrFunction<A, R> for &str {
    #[inline]
    fn to_object(self) -> Object {
        self.into()
    }
}

impl<A, R> StringOrFunction<A, R> for String {
    #[inline]
    fn to_object(self) -> Object {
        self.into()
    }
}

impl<A, R, F> StringOrFunction<A, R> for F
where
    A: Poppable,
    R: Pushable,
    F: FnMut(A) -> crate::Result<R> + 'static,
{
    #[inline]
    fn to_object(self) -> Object {
        Function::from_fn_mut(self).into()
    }
}

impl<A, R> StringOrFunction<A, R> for Function<A, R> {
    #[inline]
    fn to_object(self) -> Object {
        self.into()
    }
}
