use std::error::Error as StdError;
use std::iter::FusedIterator;

use luajit::{Poppable, Pushable};
use types::{Array, Function, LuaRef, Object};
use types::{HlGroupId, Integer};

use crate::IntoResult;

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

impl<A, R, F, O> ToFunction<A, R> for F
where
    A: Poppable,
    R: Pushable,
    F: FnMut(A) -> O + 'static,
    O: IntoResult<R>,
    O::Error: StdError + 'static,
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

impl<F, A, R, O> StringOrFunction<A, R> for F
where
    F: FnMut(A) -> O + 'static,
    A: Poppable,
    R: Pushable,
    O: IntoResult<R>,
    O::Error: StdError + 'static,
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

/// A trait implemented by types that can be converted to a highlight group ID.
pub trait HlGroup: sealed::Sealed {
    type Error;

    fn to_hl_id(&self) -> Result<HlGroupId, Self::Error>;
}

impl HlGroup for Integer {
    type Error = core::convert::Infallible;

    #[inline(always)]
    fn to_hl_id(&self) -> Result<HlGroupId, Self::Error> {
        Ok(*self)
    }
}

impl HlGroup for &str {
    type Error = crate::Error;

    #[inline]
    fn to_hl_id(&self) -> Result<HlGroupId, Self::Error> {
        let obj = types::String::from(*self).into();
        let mut err = types::Error::default();
        let hl_id = unsafe {
            crate::ffi::helpers::object_to_hl_id(
                obj,
                c"hl_group".as_ptr() as *const _,
                &mut err,
            )
        };
        if err.is_err() { Err(err.into()) } else { Ok(hl_id) }
    }
}

/// A trait implemented by types that can be passed to
/// [`SetExtmarkOptsBuilder::hl_group`](crate::opts::SetExtmarkOptsBuilder::hl_group).
#[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
#[cfg_attr(docsrs, doc(cfg(feature = "neovim-0-11")))]
pub trait SetExtmarkHlGroup {
    fn into_object(self) -> Object;
}

#[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
impl SetExtmarkHlGroup for Integer {
    #[inline]
    fn into_object(self) -> Object {
        self.into()
    }
}

#[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
impl SetExtmarkHlGroup for &str {
    #[inline]
    fn into_object(self) -> Object {
        self.into()
    }
}

#[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
impl<T: StringOrInt> SetExtmarkHlGroup for Vec<T> {
    #[inline]
    fn into_object(self) -> Object {
        self.into_iter().map(StringOrInt::to_object).collect::<Array>().into()
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for types::Integer {}

    impl Sealed for &str {}
}
