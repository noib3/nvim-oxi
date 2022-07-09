use nvim_types::{Array, Object};

pub trait StringOrInt: Sized {
    fn to_obj(self) -> Object;
}

pub trait StringOrListOfStrings: Sized {
    fn to_obj(self) -> Object;
}

macro_rules! impl_into {
    ($trait:ident, $type:ty) => {
        impl $trait for $type {
            fn to_obj(self) -> Object {
                self.into()
            }
        }
    };
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

impl_into!(StringOrListOfStrings, &str);
impl_into!(StringOrListOfStrings, String);

// Here I'd like to use `IntoIterator` instead of `Vec`, but without
// specilization that'd cause conflicting impls.
impl<S: Into<String>> StringOrListOfStrings for Vec<S> {
    fn to_obj(self) -> Object {
        Array::from_iter(self.into_iter().map(Into::into)).into()
    }
}
