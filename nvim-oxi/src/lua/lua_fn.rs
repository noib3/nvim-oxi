use std::marker::PhantomData;
use std::result::Result as StdResult;
use std::{fmt, mem, ptr};

use libc::{c_char, c_int};
use nvim_types::{LuaRef, Object, ObjectData, ObjectType};
use serde::{de, ser};

use super::ffi::*;
use crate::Result;

macro_rules! define {
    ($name:ident) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name<A, R>(
            pub(crate) LuaRef,
            PhantomData<A>,
            PhantomData<R>,
        )
        where
            A: super::LuaPoppable,
            R: super::LuaPushable;
    };
}

define!(LuaFn);
define!(LuaFnMut);
define!(LuaFnOnce);

macro_rules! debug {
    ($name:ident, $nm:literal) => {
        impl<A, R> fmt::Debug for $name<A, R>
        where
            A: super::LuaPoppable,
            R: super::LuaPushable,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_tuple($nm).field(&self.0).finish()
            }
        }
    };
}

debug!(LuaFn, "LuaFn");
debug!(LuaFnMut, "LuaFnMut");
debug!(LuaFnOnce, "LuaFnOnce");

macro_rules! from_fn_for_object {
    ($name:ident) => {
        impl<A, R> From<$name<A, R>> for Object
        where
            A: super::LuaPoppable,
            R: super::LuaPushable,
        {
            fn from(fun: $name<A, R>) -> Self {
                Self {
                    r#type: ObjectType::kObjectTypeLuaRef,
                    data: ObjectData { luaref: fun.0 },
                }
            }
        }
    };
}

from_fn_for_object!(LuaFn);
from_fn_for_object!(LuaFnMut);
from_fn_for_object!(LuaFnOnce);

macro_rules! deserialize {
    ($name:ident, $visitor:ident) => {
        impl<'de, A, R> de::Deserialize<'de> for $name<A, R>
        where
            A: super::LuaPoppable,
            R: super::LuaPushable,
        {
            fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct $visitor<A, R>(PhantomData<A>, PhantomData<R>);

                impl<'de, A, R> de::Visitor<'de> for $visitor<A, R>
                where
                    A: super::LuaPoppable,
                    R: super::LuaPushable,
                {
                    type Value = $name<A, R>;

                    fn expecting(
                        &self,
                        f: &mut fmt::Formatter,
                    ) -> fmt::Result {
                        f.write_str("an f32 representing a Lua reference")
                    }

                    fn visit_f32<E>(
                        self,
                        value: f32,
                    ) -> StdResult<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Ok($name(value as i32, PhantomData, PhantomData))
                    }
                }

                deserializer
                    .deserialize_f32($visitor(PhantomData, PhantomData))
            }
        }
    };
}

deserialize!(LuaFn, LuaFnVisitor);
deserialize!(LuaFnMut, LuaFnMutVisitor);
deserialize!(LuaFnOnce, LuaFnOnceVisitor);

macro_rules! serialize {
    ($name:ident) => {
        impl<A, R> ser::Serialize for $name<A, R>
        where
            A: super::LuaPoppable,
            R: super::LuaPushable,
        {
            fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                serializer.serialize_f32(self.0 as f32)
            }
        }
    };
}

serialize!(LuaFn);
serialize!(LuaFnMut);
serialize!(LuaFnOnce);

macro_rules! create_ref {
    ($lstate:ident, $fun:ident, $cb:ident) => {
        super::with_state(move |$lstate| unsafe {
            let fun = Box::new(move |l| $fun(A::pop(l)?)?.push(l));
            let ud = lua_newuserdata($lstate, mem::size_of::<$cb>());
            ptr::write(ud as *mut $cb, fun);
            lua_pushcclosure($lstate, c_fun, 1);
            luaL_ref($lstate, LUA_REGISTRYINDEX)
        })
    };
}

impl<A, R, F> From<F> for LuaFn<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
    F: Fn(A) -> Result<R> + 'static,
{
    fn from(fun: F) -> Self {
        type Cb = Box<dyn Fn(*mut lua_State) -> Result<c_int> + 'static>;

        unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
            let fun = {
                let idx = lua_upvalueindex(1);
                let upv = lua_touserdata(lstate, idx) as *mut Cb;
                &**upv
            };

            fun(lstate).unwrap_or_else(|err| handle_error(lstate, err))
        }

        let r#ref = create_ref!(lstate, fun, Cb);

        Self(r#ref, PhantomData, PhantomData)
    }
}

impl<A, R, F> From<F> for LuaFnMut<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
    F: FnMut(A) -> Result<R> + 'static,
{
    fn from(mut fun: F) -> Self {
        type CbMut = Box<dyn FnMut(*mut lua_State) -> Result<c_int> + 'static>;

        unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
            let fun = {
                let idx = lua_upvalueindex(1);
                let upv = lua_touserdata(lstate, idx) as *mut CbMut;
                &mut **upv
            };

            fun(lstate).unwrap_or_else(|err| handle_error(lstate, err))
        }

        let r#ref = create_ref!(lstate, fun, CbMut);

        Self(r#ref, PhantomData, PhantomData)
    }
}

impl<A, R, F> From<F> for LuaFnOnce<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
    F: FnOnce(A) -> Result<R> + 'static,
{
    fn from(fun: F) -> Self {
        type CbOnce =
            Box<dyn FnOnce(*mut lua_State) -> Result<c_int> + 'static>;

        unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
            let fun = {
                let idx = lua_upvalueindex(1);
                let upv = lua_touserdata(lstate, idx) as *mut CbOnce;
                Box::from_raw(&mut **upv)
            };

            fun(lstate).unwrap_or_else(|err| handle_error(lstate, err))
        }

        let r#ref = create_ref!(lstate, fun, CbOnce);

        Self(r#ref, PhantomData, PhantomData)
    }
}

unsafe fn handle_error(lstate: *mut lua_State, err: crate::Error) -> ! {
    let msg = err.to_string();
    lua_pushlstring(lstate, msg.as_ptr() as *const c_char, msg.len());
    lua_error(lstate);
}

macro_rules! call_body {
    ($self:ident, $args:ident) => {
        todo!()
    };
}

impl<A, R> LuaFn<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    pub fn _call(&self, _args: A) -> crate::Result<R> {
        call_body!(self, _args)
    }
}

impl<A, R> LuaFnMut<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    pub fn _call(&mut self, _args: A) -> crate::Result<R> {
        call_body!(self, _args)
    }
}

impl<A, R> LuaFnOnce<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    pub fn _call(self, _args: A) -> crate::Result<R> {
        call_body!(self, _args)
    }

    /// Removes the stored reference from the Lua registry.
    pub(crate) fn unref(self) {
        super::with_state(move |lstate| unsafe {
            luaL_unref(lstate, LUA_REGISTRYINDEX, self.0);
        })
    }
}
