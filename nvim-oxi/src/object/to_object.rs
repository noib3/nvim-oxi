use std::string::String as StdString;

use nvim_types::{Array, Object, String as NvimString};

use crate::Result;

pub trait ToObject {
    fn to_obj(self) -> Result<Object>;
}

/// Implements `ToObject` for `Into<Object>` types.
macro_rules! to_into {
    ($type:ty) => {
        impl ToObject for $type {
            fn to_obj(self) -> Result<Object> {
                Ok(self.into())
            }
        }
    };
}

/// Implements `ToObject` for "big integer" types.
macro_rules! to_bigint {
    ($type:ty) => {
        impl ToObject for $type {
            fn to_obj(self) -> Result<Object> {
                Ok(i64::try_from(self)?.into())
            }
        }
    };
}

to_into!(());
to_into!(bool);
to_into!(i8);
to_into!(u8);
to_into!(i16);
to_into!(u16);
to_into!(i32);
to_into!(u32);
to_into!(i64);
to_into!(f64);
to_into!(StdString);
to_into!(NvimString);

to_bigint!(u64);
to_bigint!(i128);
to_bigint!(u128);
to_bigint!(isize);
to_bigint!(usize);

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
