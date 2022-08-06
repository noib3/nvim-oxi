use std::ffi::CStr;

use libc::c_int;
use nvim_types::{Array, Dictionary, Object};

use super::{ffi::*, lua::grow_stack};
use crate::object::FromObject;
use crate::{Error, Result};

trait ObjectExt: Sized {
    unsafe fn pop_one(lstate: *mut lua_State) -> Result<Self>;
}

impl ObjectExt for Object {
    unsafe fn pop_one(lstate: *mut lua_State) -> Result<Self> {
        match lua_type(lstate, -1) {
            LUA_TNIL => {
                lua_pop(lstate, 1);
                Ok(Object::nil())
            },

            LUA_TBOOLEAN => {
                let n = lua_toboolean(lstate, -1);
                lua_pop(lstate, 1);
                Ok(Object::from(n == 1))
            },

            LUA_TNUMBER => {
                let n = lua_tonumber(lstate, -1);
                lua_pop(lstate, 1);

                // This checks that the number is in the range (i32::MIN,
                // i32::MAX) and that it has no fractional component.
                if n == (n as c_int) as lua_Number {
                    Ok(Object::from(n as c_int))
                } else {
                    Ok(Object::from(n))
                }
            },

            LUA_TSTRING => {
                let mut len = 0;
                let ptr = lua_tolstring(lstate, -1, &mut len);

                let mut vec = Vec::<u8>::with_capacity(len);
                std::ptr::copy(ptr as *const u8, vec.as_mut_ptr(), len);
                vec.set_len(len);

                lua_pop(lstate, 1);

                Ok(Object::from(nvim_types::String::from_bytes(vec)))
            },

            LUA_TTABLE => {
                if is_array(lstate, -1) {
                    Array::pop_one(lstate).map(Object::from)
                } else {
                    Dictionary::pop_one(lstate).map(Object::from)
                }
            },

            LUA_TFUNCTION => {
                let luaref = luaL_ref(lstate, LUA_REGISTRYINDEX);
                Ok(Object::new_luaref(luaref))
            },

            LUA_TNONE => Err(Error::custom(
                "trying to pop an Object from an empty stack position",
            )),

            LUA_TLIGHTUSERDATA | LUA_TUSERDATA | LUA_TTHREAD => {
                let typename = CStr::from_ptr(luaL_typename(lstate, -1))
                    .to_string_lossy();

                lua_pop(lstate, 1);

                Err(Error::custom(format!(
                    "cannot construct an Object from a {typename}"
                )))
            },

            _ => unreachable!(),
        }
    }
}

/// Assumes that the value at index `index` is a table and returns whether it's
/// an array table.
unsafe fn is_array(lstate: *mut lua_State, index: c_int) -> bool {
    lua_pushnil(lstate);

    if lua_next(lstate, index - 1) == 0 {
        // Empty table.
        if lua_getmetatable(lstate, index) == 0 {
            return true;
        }
        lua_pop(lstate, 1);
        return false;
    }

    let ty = lua_type(lstate, -2);
    lua_pop(lstate, 2);
    ty == LUA_TNUMBER
}

impl ObjectExt for Array {
    unsafe fn pop_one(lstate: *mut lua_State) -> Result<Self> {
        let len = lua_objlen(lstate, -1);
        let mut items = Vec::<Object>::with_capacity(len);

        // Pushing `nil` as the first key.
        lua_pushnil(lstate);

        while lua_next(lstate, -2) != 0 {
            if lua_type(lstate, -2) != LUA_TNUMBER {
                let typename = CStr::from_ptr(luaL_typename(lstate, -2))
                    .to_string_lossy();

                return Err(Error::custom(format!(
                    "encountered a {typename} key while popping an array off \
                     the stack"
                )));
            }

            items.push(Object::pop_one(lstate)?);
        }

        lua_pop(lstate, 1);

        Ok(Array::from(items))
    }
}

impl ObjectExt for Dictionary {
    unsafe fn pop_one(lstate: *mut lua_State) -> Result<Self> {
        let len = lua_objlen(lstate, -1);
        let mut pairs =
            Vec::<(nvim_types::String, Object)>::with_capacity(len);

        // Pushing `nil` as the first key.
        lua_pushnil(lstate);

        while lua_next(lstate, -2) != 0 {
            if lua_type(lstate, -2) != LUA_TSTRING {
                let typename = CStr::from_ptr(luaL_typename(lstate, -2))
                    .to_string_lossy();

                return Err(Error::custom(format!(
                    "encountered a {typename} key while popping a dictionary \
                     off the stack"
                )));
            }

            let key = {
                let mut len = 0;
                let ptr = lua_tolstring(lstate, -2, &mut len);

                let mut vec = Vec::<u8>::with_capacity(len);
                std::ptr::copy(ptr as *const u8, vec.as_mut_ptr(), len);
                vec.set_len(len);

                nvim_types::String::from_bytes(vec)
            };

            let value = Object::pop_one(lstate)?;

            pairs.push((key, value));
        }

        lua_pop(lstate, 1);

        Ok(Dictionary::from_iter(pairs))
    }
}

#[doc(hidden)]
pub trait LuaPoppable: Sized {
    const N: c_int;

    /// Assembles itself by popping `N` values off the stack. Fails if the
    /// popped values are of the wrong type.
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self>;
}

impl<A> LuaPoppable for A
where
    A: FromObject,
{
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        grow_stack(lstate, Self::N);
        A::from_obj(Object::pop_one(lstate)?)
    }
}

macro_rules! impl_tuple {
    ($($name:ident)*) => (
        impl<$($name,)*> LuaPoppable for ($($name,)*)
            where $($name: FromObject,)*
        {
            const N: c_int = count!($($name)*);

            #[allow(non_snake_case)]
            #[inline]
            unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
                grow_stack(lstate, Self::N);
                pop_reverse!(lstate, $($name)*);
                Ok(($($name,)*))
            }
        }
    );
}

macro_rules! count {
    () => {0i32};
    ($x:tt $($xs:tt)*) => {1i32 + count!($($xs)*)};
}

macro_rules! pop_reverse {
    ($lstate:expr, $x:ident $($xs:ident)*) => {
        pop_reverse!($lstate, $($xs)*);
        let $x = $x::from_obj(Object::pop($lstate)?)?;
    };

    ($lstate:expr,) => ();
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
