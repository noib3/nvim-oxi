use std::mem::ManuallyDrop;

use libc::{c_char, c_int};
use nvim_types::object::Object;

use super::ffi::*;
use crate::object::ToObject;
use crate::Result;

#[doc(hidden)]
pub trait LuaPushable {
    /// Pushes all its values on the Lua stack, returning the number of values
    /// that it pushed.
    unsafe fn push(self, lstate: *mut lua_State) -> Result<c_int>;
}

impl<T: ToObject> LuaPushable for T {
    unsafe fn push(self, lstate: *mut lua_State) -> Result<c_int> {
        push_obj(self.to_obj()?, lstate)?;
        Ok(1)
    }
}

unsafe fn push_obj(obj: Object, lstate: *mut lua_State) -> Result<()> {
    use nvim_types::object::ObjectType::*;
    match obj.r#type {
        kObjectTypeNil => lua_pushnil(lstate),

        kObjectTypeBoolean => {
            let n = if obj.data.boolean { 1 } else { 0 };
            lua_pushboolean(lstate, n);
        },

        kObjectTypeInteger => {
            let n = obj.data.integer.try_into()?;
            lua_pushinteger(lstate, n);
        },

        kObjectTypeFloat => {
            lua_pushnumber(lstate, obj.data.float);
        },

        kObjectTypeString => {
            let string = ManuallyDrop::into_inner(obj.data.string);
            lua_pushlstring(lstate, string.data as *const c_char, string.size);
        },

        kObjectTypeArray => {
            let array = ManuallyDrop::into_inner(obj.data.array);
            lua_createtable(lstate, array.len().try_into()?, 0);

            for (i, obj) in array.into_iter().enumerate() {
                push_obj(obj, lstate)?;
                lua_rawseti(lstate, -2, (i + 1).try_into()?);
            }
        },

        kObjectTypeDictionary => {
            let dict = ManuallyDrop::into_inner(obj.data.dictionary);
            lua_createtable(lstate, 0, dict.len().try_into()?);

            for (key, value) in dict {
                lua_pushlstring(lstate, key.data as *const c_char, key.size);
                push_obj(value, lstate)?;
                lua_rawset(lstate, -3);
            }
        },

        kObjectTypeLuaRef => {
            lua_rawgeti(lstate, LUA_REGISTRYINDEX, obj.data.luaref);
        },
    };

    Ok(())
}
