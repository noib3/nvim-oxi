use std::ffi::c_int;

use crate::ffi::{self, lua_Integer, lua_Number, lua_State};
use crate::macros::count;

/// Trait implemented for types that can be pushed onto the Lua stack.
pub trait LuaPushable {
    /// Pushes all its values on the Lua stack, returning the number of values
    /// that it pushed.
    unsafe fn push(
        self,
        lstate: *mut lua_State,
    ) -> Result<c_int, crate::Error>;
}

impl LuaPushable for () {
    unsafe fn push(
        self,
        lstate: *mut lua_State,
    ) -> Result<c_int, crate::Error> {
        ffi::lua_pushnil(lstate);
        Ok(1)
    }
}

impl LuaPushable for bool {
    unsafe fn push(
        self,
        lstate: *mut lua_State,
    ) -> Result<c_int, crate::Error> {
        ffi::lua_pushboolean(lstate, if self { 1 } else { 0 });
        Ok(1)
    }
}

impl LuaPushable for lua_Integer {
    unsafe fn push(
        self,
        lstate: *mut lua_State,
    ) -> Result<c_int, crate::Error> {
        ffi::lua_pushinteger(lstate, self);
        Ok(1)
    }
}

/// Implements `LuaPushable` for an integer type that implements
/// `Into<lua_Integer>`.
macro_rules! push_into_integer {
    ($integer:ty) => {
        impl LuaPushable for $integer {
            unsafe fn push(
                self,
                lstate: *mut lua_State,
            ) -> Result<c_int, crate::Error> {
                let n: lua_Integer = self.into();
                n.push(lstate)
            }
        }
    };
}

/// Implements `LuaPushable` for an integer type that implements
/// `TryInto<lua_Integer>`.
macro_rules! push_try_into_integer {
    ($integer:ty) => {
        impl LuaPushable for $integer {
            unsafe fn push(
                self,
                lstate: *mut lua_State,
            ) -> Result<c_int, crate::Error> {
                let n: lua_Integer = self.try_into().map_err(
                    |err: std::num::TryFromIntError| {
                        crate::Error::push_error(
                            std::any::type_name::<$integer>(),
                            err.to_string(),
                        )
                    },
                )?;
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

impl LuaPushable for lua_Number {
    unsafe fn push(
        self,
        lstate: *mut lua_State,
    ) -> Result<c_int, crate::Error> {
        ffi::lua_pushnumber(lstate, self);
        Ok(1)
    }
}

impl LuaPushable for f32 {
    unsafe fn push(
        self,
        lstate: *mut lua_State,
    ) -> Result<c_int, crate::Error> {
        (self as lua_Number).push(lstate)
    }
}

impl<T: LuaPushable> LuaPushable for Vec<T> {
    unsafe fn push(
        self,
        lstate: *mut lua_State,
    ) -> Result<c_int, crate::Error> {
        ffi::lua_createtable(lstate, self.len() as _, 0);

        for (i, obj) in self.into_iter().enumerate() {
            obj.push(lstate)?;
            ffi::lua_rawseti(lstate, -2, (i + 1) as _);
        }

        Ok(1)
    }
}

/// Implements `LuaPushable` for a tuple `(a, b, c, ..)` where all the elements
/// in the tuple implement `LuaPushable`.
macro_rules! push_tuple {
    ($($name:ident)*) => {
        impl<$($name,)*> LuaPushable for ($($name,)*)
        where
            $($name: LuaPushable,)*
        {
            #[allow(non_snake_case)]
            unsafe fn push(
                self,
                lstate: *mut lua_State,
            ) -> Result<c_int, crate::Error> {
                let ($($name,)*) = self;
                $($name.push(lstate)?;)*
                Ok(count!($($name)*))
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
