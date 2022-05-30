use libc::{c_char, c_int};

use super::ffi::*;
use crate::object::ToObject;

pub(crate) trait LuaPushable {
    /// Pushes all its values on the Lua stack, returning the number of values
    /// that it pushed.
    unsafe fn push(self, lstate: *mut lua_State) -> crate::Result<c_int>;
}

impl<T: ToObject> LuaPushable for T {
    unsafe fn push(self, lstate: *mut lua_State) -> crate::Result<c_int> {
        let obj = self.to_obj();

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
                let string = &obj.data.string;
                lua_pushlstring(
                    lstate,
                    string.data as *const c_char,
                    string.size,
                );
            },

            kObjectTypeArray => todo!(),

            kObjectTypeDictionary => todo!(),

            kObjectTypeLuaRef => panic!("trying to return Lua function"),
        }

        Ok(1)
    }
}
