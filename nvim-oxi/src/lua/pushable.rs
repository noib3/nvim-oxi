use libc::{c_char, c_int};
use nvim_types::Object;

use super::ffi::*;
use crate::object::ToObject;
use crate::Result;

trait ObjectExt {
    unsafe fn push(self, lstate: *mut lua_State) -> Result<()>;
}

impl ObjectExt for Object {
    unsafe fn push(self, lstate: *mut lua_State) -> Result<()> {
        use nvim_types::ObjectType::*;
        match self.r#type {
            kObjectTypeNil => lua_pushnil(lstate),

            kObjectTypeBoolean => {
                let n = if self.data.boolean { 1 } else { 0 };
                lua_pushboolean(lstate, n);
            },

            kObjectTypeInteger => {
                let n = self.data.integer.try_into()?;
                lua_pushinteger(lstate, n);
            },

            kObjectTypeFloat => {
                lua_pushnumber(lstate, self.data.float);
            },

            kObjectTypeString => {
                let string = self.into_string_unchecked();
                lua_pushlstring(
                    lstate,
                    string.data as *const c_char,
                    string.size,
                );
            },

            kObjectTypeArray => {
                let array = self.into_array_unchecked();
                lua_createtable(lstate, array.len().try_into()?, 0);

                for (i, obj) in array.into_iter().enumerate() {
                    obj.push(lstate)?;
                    lua_rawseti(lstate, -2, (i + 1).try_into()?);
                }
            },

            kObjectTypeDictionary => {
                let dict = self.into_dict_unchecked();
                lua_createtable(lstate, 0, dict.len().try_into()?);

                for (key, value) in dict {
                    lua_pushlstring(
                        lstate,
                        key.data as *const c_char,
                        key.size,
                    );
                    value.push(lstate)?;
                    lua_rawset(lstate, -3);
                }
            },

            kObjectTypeLuaRef => {
                lua_rawgeti(lstate, LUA_REGISTRYINDEX, self.data.luaref);
            },
        };

        Ok(())
    }
}

#[doc(hidden)]
pub trait LuaPushable {
    /// Pushes all its values on the Lua stack, returning the number of values
    /// that it pushed.
    unsafe fn push(self, lstate: *mut lua_State) -> Result<c_int>;
}

impl<A> LuaPushable for A
where
    A: ToObject,
{
    unsafe fn push(self, lstate: *mut lua_State) -> Result<c_int> {
        self.to_obj()?.push(lstate)?;
        Ok(1)
    }
}

macro_rules! impl_tuple {
    ($($name:ident)*) => (
        impl<$($name,)*> LuaPushable for ($($name,)*)
            where $($name: ToObject,)*
        {
            #[allow(non_snake_case)]
            #[inline]
            unsafe fn push(self, lstate: *mut lua_State) -> Result<c_int> {
                let ($($name,)*) = self;
                $($name.to_obj()?.push(lstate)?;)*
                Ok(count!($($name)*))
            }
        }
    );
}

macro_rules! count {
    () => {0i32};
    ($x:tt $($xs:tt)*) => {1i32 + count!($($xs)*)};
}

impl_tuple!(A);
impl_tuple!(A B);
impl_tuple!(A B C);
impl_tuple!(A B C D);
impl_tuple!(A B C D E);
impl_tuple!(A B C D E F);
impl_tuple!(A B C D E F G);
impl_tuple!(A B C D E F G H);
impl_tuple!(A B C D E F G H I);
impl_tuple!(A B C D E F G H I J);
impl_tuple!(A B C D E F G H I J K);
impl_tuple!(A B C D E F G H I J K L);
impl_tuple!(A B C D E F G H I J K L M);
impl_tuple!(A B C D E F G H I J K L M N);
impl_tuple!(A B C D E F G H I J K L M N O);
impl_tuple!(A B C D E F G H I J K L M N O P);
