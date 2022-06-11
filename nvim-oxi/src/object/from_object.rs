use std::string::String as StdString;

use nvim_types::{Object, String as NvimString};

use crate::Result;

pub trait FromObject: Sized {
    fn from_obj(obj: Object) -> Result<Self>;
}

/// Implements `FromObject` for a `TryFrom<Object>` type.
macro_rules! from_try_from {
    ($type:ty) => {
        impl FromObject for $type {
            #[inline(always)]
            fn from_obj(obj: Object) -> Result<Self> {
                Self::try_from(obj).map_err(crate::Error::from)
            }
        }
    };
}

from_try_from!(());
from_try_from!(bool);
from_try_from!(i8);
from_try_from!(u8);
from_try_from!(i16);
from_try_from!(u16);
from_try_from!(i32);
from_try_from!(u32);
from_try_from!(i64);
from_try_from!(u64);
from_try_from!(i128);
from_try_from!(u128);
from_try_from!(isize);
from_try_from!(usize);
from_try_from!(f64);
from_try_from!(StdString);
from_try_from!(NvimString);

impl<T> FromObject for Option<T>
where
    T: FromObject,
{
    fn from_obj(obj: Object) -> Result<Self> {
        (!obj.is_nil()).then(|| T::from_obj(obj)).transpose()
    }
}
