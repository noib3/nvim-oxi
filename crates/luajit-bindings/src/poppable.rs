use std::ffi::c_int;

use crate::ffi::*;
use crate::macros::count;

/// Trait implemented for types that can be popped off the Lua stack.
pub trait LuaPoppable: Sized {
    /// The number of elements that will be popped off the stack.
    const N: c_int;

    /// Assembles itself by popping `N` values off the stack.
    unsafe fn pop(lua_state: *mut lua_State) -> Result<Self, crate::Error>;
}

unsafe fn pop_impl<T, F>(
    lstate: *mut lua_State,
    expected: c_int,
    if_expected: F,
) -> Result<T, crate::Error>
where
    F: FnOnce(*mut lua_State) -> Result<T, crate::Error>,
{
    match lua_type(lstate, -1) {
        t if t == expected => {
            let res = if_expected(lstate);
            lua_pop(lstate, -1);
            res
        },

        LUA_TNONE => Err(crate::Error::pop_error(
            std::any::type_name::<T>(),
            "stack is empty",
        )),

        _ => Err(crate::Error::pop_wrong_type_at_idx::<T>(lstate, -1)),
    }
}

impl LuaPoppable for () {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, crate::Error> {
        // NOTE:
        //
        // pop_impl(lstate, LUA_TNIL, |_| Ok(()))

        lua_pop(lstate, -1);
        Ok(())
    }
}

impl LuaPoppable for bool {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, crate::Error> {
        pop_impl(lstate, LUA_TBOOLEAN, |lstate| {
            Ok(lua_toboolean(lstate, -1) == 1)
        })
    }
}

impl LuaPoppable for lua_Integer {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, crate::Error> {
        pop_impl(lstate, LUA_TNUMBER, |lstate| Ok(lua_tointeger(lstate, -1)))
    }
}

/// Implements `LuaPoppable` for an integer type that implements
/// `TryFrom<lua_Integer>`.
macro_rules! pop_try_from_integer {
    ($integer:ty) => {
        impl LuaPoppable for $integer {
            const N: c_int = 1;

            unsafe fn pop(
                lstate: *mut lua_State,
            ) -> Result<Self, crate::Error> {
                let n = lua_Integer::pop(lstate)?;
                let n = n.try_into();
                n.map_err(|err: std::num::TryFromIntError| {
                    crate::Error::pop_error(
                        std::any::type_name::<$integer>(),
                        err.to_string(),
                    )
                })
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

impl LuaPoppable for lua_Number {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, crate::Error> {
        pop_impl(lstate, LUA_TNUMBER, |lstate| Ok(lua_tonumber(lstate, -1)))
    }
}

impl LuaPoppable for f32 {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, crate::Error> {
        Ok(lua_Number::pop(lstate)? as _)
    }
}

impl LuaPoppable for Vec<u8> {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, crate::Error> {
        pop_impl(lstate, LUA_TSTRING, |lstate| {
            let mut len = 0;
            let ptr = lua_tolstring(lstate, -1, &mut len);

            let mut vec = Vec::<u8>::with_capacity(len);
            std::ptr::copy(ptr as *const u8, vec.as_mut_ptr(), len);
            vec.set_len(len);
            Ok(vec)
        })
    }
}

impl LuaPoppable for String {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, crate::Error> {
        pop_impl(lstate, LUA_TSTRING, |lstate| {
            let vec = LuaPoppable::pop(lstate)?;

            String::from_utf8(vec).map_err(|err| {
                crate::Error::pop_error(
                    std::any::type_name::<String>(),
                    err.to_string(),
                )
            })
        })
    }
}

impl<T: LuaPoppable> LuaPoppable for Option<T> {
    // TODO: T::N could also be an option.
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, crate::Error> {
        match lua_type(lstate, -1) {
            LUA_TNIL => {
                lua_pop(lstate, -1);
                Ok(None)
            },
            _ => T::pop(lstate).map(Some),
        }
    }
}

/// Implements `LuaPoppable` for a tuple `(a, b, c, ..)` where all the elements
/// in the tuple implement `LuaPoppable`.
macro_rules! pop_tuple {
    ($($name:ident)*) => (
        impl<$($name,)*> LuaPoppable for ($($name,)*)
        where
            $($name: LuaPoppable,)*
        {
            const N: c_int = count!($($name)*);

            #[allow(non_snake_case)]
            unsafe fn pop(lstate: *mut lua_State) -> Result<Self, crate::Error> {
                crate::utils::grow_stack(lstate, Self::N);
                pop_reverse!(lstate, $($name)*);
                Ok(($($name,)*))
            }
        }
    );
}

macro_rules! pop_reverse {
    ($lstate:expr, $x:ident $($xs:ident)*) => {
        pop_reverse!($lstate, $($xs)*);
        let $x = $x::pop($lstate)?;
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
