impl<A, R> FromObject for lua::LuaFun<A, R>
where
    A: lua::LuaPoppable,
    R: lua::LuaPushable,
{
    fn from_obj(obj: Object) -> StdResult<Self, FromObjectError> {
        (matches!(obj.r#type, ObjectType::kObjectTypeLuaRef))
            .then(|| lua::LuaFun::from(unsafe { obj.data.luaref }))
            .ok_or_else(|| FromObjectError::Primitive {
                expected: ObjectType::kObjectTypeLuaRef,
                actual: obj.r#type,
            })
    }
}
