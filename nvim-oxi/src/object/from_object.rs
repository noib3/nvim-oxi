use std::string::String as StdString;

use nvim_types::{Array, Object, String as NvimString};

use crate::Result;

pub trait FromObject: Sized {
    fn from_obj(obj: Object) -> Result<Self>;
}

/// Implements `FromObject` for `TryFrom<Object>` types.
macro_rules! impl_try_from {
    ($type:ty) => {
        impl FromObject for $type {
            #[inline(always)]
            fn from_obj(obj: Object) -> Result<Self> {
                Self::try_from(obj).map_err(From::from)
            }
        }
    };
}

impl_try_from!(());
impl_try_from!(bool);
impl_try_from!(i8);
impl_try_from!(u8);
impl_try_from!(i16);
impl_try_from!(u16);
impl_try_from!(i32);
impl_try_from!(u32);
impl_try_from!(i64);
impl_try_from!(u64);
impl_try_from!(i128);
impl_try_from!(u128);
impl_try_from!(isize);
impl_try_from!(usize);
impl_try_from!(f64);
impl_try_from!(StdString);
impl_try_from!(NvimString);

impl<T> FromObject for Option<T>
where
    T: FromObject,
{
    fn from_obj(obj: Object) -> Result<Self> {
        (!obj.is_nil()).then(|| T::from_obj(obj)).transpose()
    }
}

impl<T> FromObject for Vec<T>
where
    T: FromObject,
{
    fn from_obj(obj: Object) -> Result<Self> {
        Array::try_from(obj)?.into_iter().map(FromObject::from_obj).collect()
    }
}
