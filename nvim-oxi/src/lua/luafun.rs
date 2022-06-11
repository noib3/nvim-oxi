use std::marker::PhantomData;
use std::result::Result as StdResult;
use std::{fmt, mem, ptr};

use libc::{c_char, c_int};
use nvim_types::{LuaRef, Object, ObjectData, ObjectType};
use serde::{de, ser};

use super::ffi::*;
use crate::object::ToObject;
use crate::Result;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct LuaFun<A, R>(pub(crate) LuaRef, PhantomData<A>, PhantomData<R>)
where
    A: super::LuaPoppable,
    R: super::LuaPushable;

impl<A, R> fmt::Debug for LuaFun<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("LuaFun").field(&self.0).finish()
    }
}

impl<A, R> From<LuaFun<A, R>> for Object
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    fn from(fun: LuaFun<A, R>) -> Self {
        Self {
            r#type: ObjectType::kObjectTypeLuaRef,
            data: ObjectData { luaref: fun.0 },
        }
    }
}

impl<A, R> ToObject for LuaFun<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    fn to_obj(self) -> Result<Object> {
        Ok(self.into())
    }
}

impl<'de, A, R> de::Deserialize<'de> for LuaFun<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct LuaFunVisitor<A, R>(PhantomData<A>, PhantomData<R>);

        impl<'de, A, R> de::Visitor<'de> for LuaFunVisitor<A, R>
        where
            A: super::LuaPoppable,
            R: super::LuaPushable,
        {
            type Value = LuaFun<A, R>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an f32 representing a Lua reference")
            }

            fn visit_f32<E>(self, value: f32) -> StdResult<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LuaFun(value as i32, PhantomData, PhantomData))
            }
        }

        deserializer.deserialize_f32(LuaFunVisitor(PhantomData, PhantomData))
    }
}

impl<A, R> ser::Serialize for LuaFun<A, R>
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

macro_rules! create_ref {
    ($lstate:ident, $fun:ident, $cb:ty) => {
        super::with_state(move |$lstate| unsafe {
            let fun = Box::new(move |l| $fun(A::pop(l)?)?.push(l));
            let ud = lua_newuserdata($lstate, mem::size_of::<$cb>());
            ptr::write(ud as *mut $cb, fun);
            lua_pushcclosure($lstate, c_fun, 1);
            luaL_ref($lstate, LUA_REGISTRYINDEX)
        })
    };
}

impl<A, R> LuaFun<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    pub fn from_fn<F>(fun: F) -> Self
    where
        F: Fn(A) -> Result<R> + 'static,
    {
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

    pub fn from_fn_mut<F>(mut fun: F) -> Self
    where
        F: FnMut(A) -> Result<R> + 'static,
    {
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

    pub fn from_fn_once<F>(fun: F) -> Self
    where
        F: FnOnce(A) -> Result<R> + 'static,
    {
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

    pub fn _call(&self, _args: A) -> Result<R> {
        todo!()
    }
}

unsafe fn handle_error(lstate: *mut lua_State, err: crate::Error) -> ! {
    let msg = err.to_string();
    lua_pushlstring(lstate, msg.as_ptr() as *const c_char, msg.len());
    lua_error(lstate);
}

impl<A, R> Drop for LuaFun<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    fn drop(&mut self) {
        // Remove the reference stored in the Lua registry when the function
        // gets dropped.
        super::with_state(move |lstate| unsafe {
            luaL_unref(lstate, LUA_REGISTRYINDEX, self.0);
        })
    }
}
