use nvim_types::object::Object;
use serde::de;

use crate::Result;

pub trait FromObject: Sized {
    fn from_obj(obj: Object) -> Result<Self>;
}

impl<'de, T> FromObject for T
where
    T: de::Deserialize<'de>,
{
    fn from_obj(obj: Object) -> Result<Self> {
        T::deserialize(super::Deserializer { obj })
    }
}
