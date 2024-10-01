use std::collections::HashMap;
use std::hash::Hash;

use crate::ffi::*;
use crate::macros::count;
use crate::Error;

/// Trait implemented for types that can be popped off the Lua stack.
pub trait Poppable: Sized {
    /// Pops the value at the top of the stack.
    unsafe fn pop(lua_state: *mut State) -> Result<Self, Error>;
}

impl Poppable for () {
    #[inline(always)]
    unsafe fn pop(state: *mut State) -> Result<Self, crate::Error> {
        if lua_gettop(state) == 0 {
            Ok(())
        } else if lua_type(state, -1) == LUA_TNIL {
            lua_pop(state, 1);
            Ok(())
        } else {
            Err(Error::pop_wrong_type::<Self>(LUA_TNIL, lua_type(state, -1)))
        }
    }
}

impl Poppable for bool {
    unsafe fn pop(state: *mut State) -> Result<Self, Error> {
        if lua_gettop(state) == 0 {
            return Err(Error::PopEmptyStack);
        }

        match lua_type(state, -1) {
            LUA_TBOOLEAN => {
                let b = lua_toboolean(state, -1) == 1;
                lua_pop(state, 1);
                Ok(b)
            },
            other => Err(Error::pop_wrong_type::<Self>(LUA_TBOOLEAN, other)),
        }
    }
}

impl Poppable for Integer {
    unsafe fn pop(state: *mut State) -> Result<Self, crate::Error> {
        if lua_gettop(state) == 0 {
            return Err(Error::PopEmptyStack);
        }

        match lua_type(state, -1) {
            LUA_TNUMBER => {
                let n = lua_tointeger(state, -1);
                lua_pop(state, 1);
                Ok(n)
            },
            other => Err(Error::pop_wrong_type::<Self>(LUA_TNUMBER, other)),
        }
    }
}

/// Implements `Poppable` for a integer types that implement
/// `TryFrom<Integer>`.
macro_rules! pop_try_from_integer {
    ($integer:ty) => {
        impl Poppable for $integer {
            unsafe fn pop(lstate: *mut State) -> Result<Self, crate::Error> {
                Integer::pop(lstate)?
                    .try_into()
                    .map_err(Error::pop_error_from_err::<Self, _>)
            }
        }
    };
}

pop_try_from_integer!(i8);
pop_try_from_integer!(u8);
pop_try_from_integer!(i16);
pop_try_from_integer!(u16);
pop_try_from_integer!(i32);
pop_try_from_integer!(u32);
pop_try_from_integer!(i64);
pop_try_from_integer!(u64);
pop_try_from_integer!(usize);

impl Poppable for Number {
    unsafe fn pop(state: *mut State) -> Result<Self, crate::Error> {
        if lua_gettop(state) == 0 {
            return Err(Error::PopEmptyStack);
        }

        match lua_type(state, -1) {
            LUA_TNUMBER => {
                let n = lua_tonumber(state, -1);
                lua_pop(state, 1);
                Ok(n)
            },
            other => Err(Error::pop_wrong_type::<Self>(LUA_TNUMBER, other)),
        }
    }
}

impl Poppable for f32 {
    unsafe fn pop(state: *mut State) -> Result<Self, crate::Error> {
        Number::pop(state).map(|n| n as f32)
    }
}

impl Poppable for String {
    unsafe fn pop(state: *mut State) -> Result<Self, Error> {
        if lua_gettop(state) == 0 {
            return Err(Error::PopEmptyStack);
        }

        match lua_type(state, -1) {
            LUA_TSTRING | LUA_TNUMBER => {
                let mut len = 0;
                let ptr = lua_tolstring(state, -1, &mut len);

                // NOTE: `ptr` should never be null if the value at the top of
                // the stack is a string or a number.
                assert!(!ptr.is_null());

                let slice = std::slice::from_raw_parts(ptr as *const u8, len);
                let str = String::from_utf8_lossy(slice).to_string();

                lua_pop(state, 1);

                Ok(str)
            },
            other => Err(Error::pop_wrong_type::<Self>(LUA_TSTRING, other)),
        }
    }
}

impl<T> Poppable for Option<T>
where
    T: Poppable,
{
    unsafe fn pop(state: *mut State) -> Result<Self, Error> {
        if lua_gettop(state) == 0 {
            return Ok(None);
        }

        match lua_type(state, -1) {
            LUA_TNIL => {
                lua_pop(state, 1);
                Ok(None)
            },
            _ => T::pop(state).map(Some),
        }
    }
}

impl<T> Poppable for Vec<T>
where
    T: Poppable,
{
    unsafe fn pop(state: *mut State) -> Result<Self, Error> {
        if lua_gettop(state) == 0 {
            return Err(Error::PopEmptyStack);
        }

        match lua_type(state, -1) {
            LUA_TTABLE => {
                // TODO: check that the table is an array-like table and not a
                // dictionary-like one.

                let mut vec = Vec::with_capacity(lua_objlen(state, -1));

                lua_pushnil(state);

                while lua_next(state, -2) != 0 {
                    vec.push(T::pop(state)?);
                }

                // Pop the table.
                lua_pop(state, 1);

                Ok(vec)
            },

            other => Err(Error::pop_wrong_type::<Self>(LUA_TTABLE, other)),
        }
    }
}

impl<K, V> Poppable for HashMap<K, V>
where
    K: Poppable + Eq + Hash,
    V: Poppable,
{
    unsafe fn pop(state: *mut State) -> Result<Self, Error> {
        if lua_gettop(state) == 0 {
            return Err(Error::PopEmptyStack);
        }

        match lua_type(state, -1) {
            LUA_TTABLE => {
                // TODO: check that the table is an dictionary-like table and
                // not an array-like one.

                let mut map = HashMap::with_capacity(lua_objlen(state, -1));

                lua_pushnil(state);

                while lua_next(state, -2) != 0 {
                    let value = V::pop(state)?;

                    // NOTE: the following `K::pop` will pop the key, so we
                    // push another copy of the key on the stack for the next
                    // iteration.
                    lua_pushvalue(state, -1);

                    let key = K::pop(state)?;

                    map.insert(key, value);
                }

                // Pop the table.
                lua_pop(state, 1);

                Ok(map)
            },

            other => Err(Error::pop_wrong_type::<Self>(LUA_TTABLE, other)),
        }
    }
}

/// Implements `Poppable` for a tuple `(a, b, c, ..)` where all the elements
/// in the tuple implement `Poppable`.
macro_rules! pop_tuple {
    ($($name:ident)*) => (
        impl<$($name,)*> Poppable for ($($name,)*)
        where
            $($name: Poppable,)*
        {
            #[allow(non_snake_case)]
            unsafe fn pop(state: *mut State) -> Result<Self, crate::Error> {
                crate::utils::grow_stack(state, count!($($name)*));
                pop_reverse!(state, $($name)*);
                Ok(($($name,)*))
            }
        }
    );
}

macro_rules! pop_reverse {
    ($lua_state:expr, $x:ident $($xs:ident)*) => {
        pop_reverse!($lua_state, $($xs)*);
        let $x = $x::pop($lua_state)?;
    };

    ($lstate:expr,) => ();
}

pop_tuple!(A);
pop_tuple!(A B);
pop_tuple!(A B C);
pop_tuple!(A B C D);
pop_tuple!(A B C D E);
pop_tuple!(A B C D E F);
pop_tuple!(A B C D E F G);
pop_tuple!(A B C D E F G H);
pop_tuple!(A B C D E F G H I);
pop_tuple!(A B C D E F G H I J);
pop_tuple!(A B C D E F G H I J K);
pop_tuple!(A B C D E F G H I J K L);
pop_tuple!(A B C D E F G H I J K L M);
pop_tuple!(A B C D E F G H I J K L M N);
pop_tuple!(A B C D E F G H I J K L M N O);
pop_tuple!(A B C D E F G H I J K L M N O P);
