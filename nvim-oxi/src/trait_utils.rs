use nvim_types::{Array, Object};

use crate::lua::{Function, LuaPoppable, LuaPushable};

macro_rules! impl_into {
    ($trait:ident, $type:ty) => {
        impl $trait for $type {
            fn to_obj(self) -> Object {
                self.into()
            }
        }
    };
}

/// A string or an integer.
pub trait StringOrInt {
    fn to_obj(self) -> Object;
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

/// A string or a list of strings.
pub trait StringOrListOfStrings {
    fn to_obj(self) -> Object;
}

impl_into!(StringOrListOfStrings, &str);
impl_into!(StringOrListOfStrings, String);

// Here I'd like to use `IntoIterator` instead of `Vec`, but without
// specilization that'd cause conflicting impls.
impl<S: Into<String>> StringOrListOfStrings for Vec<S> {
    fn to_obj(self) -> Object {
        Array::from_iter(self.into_iter().map(Into::into)).into()
    }
}

pub trait StringOrFunction<A, R> {
    fn to_obj(self) -> Object;
}

impl<A, R> StringOrFunction<A, R> for &str {
    fn to_obj(self) -> Object {
        self.into()
    }
}

impl<A, R> StringOrFunction<A, R> for String {
    fn to_obj(self) -> Object {
        self.into()
    }
}

impl<A, R, F> StringOrFunction<A, R> for F
where
    A: LuaPoppable,
    R: LuaPushable,
    F: FnMut(A) -> crate::Result<R> + 'static,
{
    fn to_obj(self) -> Object {
        Function::from_fn_mut(self).into()
    }
}

impl<A, R> StringOrFunction<A, R> for Function<A, R> {
    fn to_obj(self) -> Object {
        self.into()
    }
}
