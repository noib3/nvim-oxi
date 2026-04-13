use core::ffi::{c_char, c_int};

use crate::ffi::{self, Integer, Number, State};
use crate::macros::count;
use crate::utils::{self, push_error};

/// Trait implemented for types that can be pushed onto the Lua stack.
pub trait Pushable {
    /// Pushes all its values on the Lua stack, returning the number of values
    /// that it pushed.
    unsafe fn push(self, lstate: *mut State) -> c_int;
}

impl Pushable for () {
    unsafe fn push(self, lstate: *mut State) -> c_int {
        ffi::lua_pushnil(lstate);
        1
    }
}

impl Pushable for bool {
    unsafe fn push(self, lstate: *mut State) -> c_int {
        ffi::lua_pushboolean(lstate, self as _);
        1
    }
}

impl Pushable for Integer {
    unsafe fn push(self, lstate: *mut State) -> c_int {
        ffi::lua_pushinteger(lstate, self);
        1
    }
}

/// Implements `LuaPushable` for an integer type that implements
/// `Into<Integer>`.
macro_rules! push_into_integer {
    ($integer:ty) => {
        impl Pushable for $integer {
            unsafe fn push(self, lstate: *mut State) -> c_int {
                let n: Integer = self.into();
                n.push(lstate)
            }
        }
    };
}

/// Implements `LuaPushable` for an integer type that implements
/// `TryInto<Integer>`.
macro_rules! push_try_into_integer {
    ($integer:ty) => {
        impl Pushable for $integer {
            unsafe fn push(self, lstate: *mut State) -> c_int {
                let n: Result<Integer, _> = self.try_into().map_err(
                    |err: std::num::TryFromIntError| {
                        crate::Error::push_error(
                            std::any::type_name::<$integer>(),
                            err.to_string(),
                        )
                    },
                );
                n.push(lstate)
            }
        }
    };
}

push_into_integer!(i8);
push_into_integer!(u8);
push_into_integer!(i16);
push_try_into_integer!(u16);
push_try_into_integer!(i32);
push_try_into_integer!(u32);
push_try_into_integer!(i64);
push_try_into_integer!(u64);
push_try_into_integer!(usize);

impl Pushable for Number {
    unsafe fn push(self, lstate: *mut State) -> c_int {
        ffi::lua_pushnumber(lstate, self);
        1
    }
}

impl Pushable for f32 {
    unsafe fn push(self, lstate: *mut State) -> c_int {
        (self as Number).push(lstate)
    }
}

impl Pushable for String {
    unsafe fn push(self, lstate: *mut State) -> c_int {
        ffi::lua_pushlstring(
            lstate,
            self.as_ptr() as *const c_char,
            self.len(),
        );
        1
    }
}

impl<T> Pushable for Option<T>
where
    T: Pushable,
{
    unsafe fn push(self, lstate: *mut State) -> c_int {
        match self {
            Some(t) => t.push(lstate),
            None => ().push(lstate),
        }
    }
}

impl<T> Pushable for Vec<T>
where
    T: Pushable,
{
    unsafe fn push(self, lstate: *mut State) -> c_int {
        ffi::lua_createtable(lstate, self.len() as _, 0);

        for (i, obj) in self.into_iter().enumerate() {
            obj.push(lstate);
            ffi::lua_rawseti(lstate, -2, (i + 1) as _);
        }

        1
    }
}

impl<T, E> Pushable for Result<T, E>
where
    T: Pushable,
    E: std::error::Error,
{
    #[inline]
    unsafe fn push(self, lstate: *mut State) -> c_int {
        match self {
            Ok(value) => value.push(lstate),
            Err(err) => push_error(&err, lstate),
        }
    }
}

/// Implements `LuaPushable` for a tuple `(a, b, c, ..)` where all the elements
/// in the tuple implement `LuaPushable`.
macro_rules! push_tuple {
    ($($name:ident)*) => {
        impl<$($name,)*> Pushable for ($($name,)*)
        where
            $($name: Pushable,)*
        {
            #[allow(non_snake_case)]
            unsafe fn push(
                self,
                lstate: *mut State,
            ) -> c_int {
                let ($($name,)*) = self;
                $($name.push(lstate);)*
                count!($($name)*)
            }
        }
    }
}

push_tuple!(A);
push_tuple!(A B);
push_tuple!(A B C);
push_tuple!(A B C D);
push_tuple!(A B C D E);
push_tuple!(A B C D E F);
push_tuple!(A B C D E F G);
push_tuple!(A B C D E F G H);
push_tuple!(A B C D E F G H I);
push_tuple!(A B C D E F G H I J);
push_tuple!(A B C D E F G H I J K);
push_tuple!(A B C D E F G H I J K L);
push_tuple!(A B C D E F G H I J K L M);
push_tuple!(A B C D E F G H I J K L M N);
push_tuple!(A B C D E F G H I J K L M N O);
push_tuple!(A B C D E F G H I J K L M N O P);
