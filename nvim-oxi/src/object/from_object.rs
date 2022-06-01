use nvim_types::object::{FromObjectError, Object, ObjectType};
use serde::de;

use crate::lua;
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

impl<A, R> FromObject for lua::LuaFun<A, R>
where
    A: lua::LuaPoppable,
    R: lua::LuaPushable,
{
    fn from_obj(obj: Object) -> Result<Self> {
        (matches!(obj.r#type, ObjectType::kObjectTypeLuaRef))
            .then(|| lua::LuaFun::from(unsafe { obj.data.luaref }))
            .ok_or_else(|| FromObjectError::Primitive {
                expected: ObjectType::kObjectTypeLuaRef,
                actual: obj.r#type,
            })
            .map_err(crate::Error::from)
    }
}
