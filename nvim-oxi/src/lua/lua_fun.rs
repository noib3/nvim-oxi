use std::marker::PhantomData;
use std::{mem, ptr};

use libc::{c_char, c_int};
use nvim_types::{
    object::{Object, ObjectData, ObjectType},
    LuaRef,
};

use super::ffi::*;
use crate::Result;

/// TODO: docs
#[derive(Clone, Debug)]
pub(crate) struct LuaFun<A, R>(
    pub(crate) LuaRef,
    PhantomData<A>,
    PhantomData<R>,
)
where
    A: super::LuaPoppable,
    R: super::LuaPushable;

impl<A, R> From<LuaRef> for LuaFun<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    fn from(luaref: LuaRef) -> Self {
        Self(luaref, PhantomData, PhantomData)
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

type CbMut = Box<dyn FnMut(*mut lua_State) -> Result<c_int> + 'static>;
type CbOnce = Box<dyn FnOnce(*mut lua_State) -> Result<c_int> + 'static>;

impl<A, R> LuaFun<A, R>
where
    A: super::LuaPoppable,
    R: super::LuaPushable,
{
    pub fn from_fn_mut<F>(mut fun: F) -> Self
    where
        F: FnMut(A) -> crate::Result<R> + 'static,
    {
        unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
            let fun = {
                let idx = lua_upvalueindex(1);
                let upv = lua_touserdata(lstate, idx) as *mut CbMut;
                &mut **upv
            };

            fun(lstate).unwrap_or_else(|err| handle_error(lstate, err))
        }

        let r#ref = super::with_state(move |lstate| unsafe {
            let fun = Box::new(move |l| fun(A::pop(l)?)?.push(l));
            let ud = lua_newuserdata(lstate, mem::size_of::<CbMut>());
            ptr::write(ud as *mut CbMut, fun);
            lua_pushcclosure(lstate, c_fun, 1);
            luaL_ref(lstate, LUA_REGISTRYINDEX)
        });

        Self(r#ref, PhantomData, PhantomData)
    }

    pub fn from_fn_once<F>(fun: F) -> Self
    where
        F: FnOnce(A) -> crate::Result<R> + 'static,
    {
        unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
            let fun = {
                let idx = lua_upvalueindex(1);
                let upv = lua_touserdata(lstate, idx) as *mut CbOnce;
                Box::from_raw(&mut **upv)
            };

            fun(lstate).unwrap_or_else(|err| handle_error(lstate, err))
        }

        let r#ref = super::with_state(move |lstate| unsafe {
            let fun = Box::new(move |l| fun(A::pop(l)?)?.push(l));
            let ud = lua_newuserdata(lstate, mem::size_of::<CbOnce>());
            ptr::write(ud as *mut CbOnce, fun);
            lua_pushcclosure(lstate, c_fun, 1);
            luaL_ref(lstate, LUA_REGISTRYINDEX)
        });

        Self(r#ref, PhantomData, PhantomData)
    }

    pub fn _call(_args: A) -> crate::Result<R> {
        todo!()
    }

    /// Removes the stored reference from the Lua registry.
    pub(crate) fn unref(self) {
        super::with_state(move |lstate| unsafe {
            luaL_unref(lstate, LUA_REGISTRYINDEX, self.0);
        })
    }
}

// TODO: better error reporting. Look at
// https://github.com/khvzak/mlua/blob/b065db37c2dd9e9c1d5483509bbd1bcc355f4fef/src/lua.rs#L2971
unsafe fn handle_error(lstate: *mut lua_State, err: crate::Error) -> ! {
    let msg = err.to_string();
    lua_pushlstring(lstate, msg.as_ptr() as *const c_char, msg.len());
    lua_error(lstate);
}
