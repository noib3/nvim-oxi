use libc::c_int;
use nvim_types::{self as nvim, Object};

use super::{ffi::lua_State, lua::grow_stack};
use crate::object::FromObject;
use crate::Result;

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/lua/converter.c#L1048
    fn nlua_pop_Object(
        lstate: *const lua_State,
        r#ref: bool,
        err: *mut nvim::Error,
    ) -> Object;
}

trait ObjectExt: Sized {
    unsafe fn pop_obj(lstate: *mut lua_State) -> Result<Self>;
}

impl ObjectExt for Object {
    unsafe fn pop_obj(lstate: *mut lua_State) -> Result<Self> {
        let mut err = nvim::Error::new();
        let obj = nlua_pop_Object(lstate, true, &mut err);
        err.into_err_or_else(|| obj)
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
        A::from_obj(Object::pop_obj(lstate)?)
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
