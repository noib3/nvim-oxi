use std::{mem, ptr};

use libc::c_int;
use nvim_types::LuaRef as NvimLuaRef;

use super::ffi::*;
use crate::Result;

/// TODO: docs
pub(crate) struct LuaRef(pub(crate) NvimLuaRef);

impl From<NvimLuaRef> for LuaRef {
    fn from(orig: NvimLuaRef) -> Self {
        Self(orig)
    }
}

type LuaFnMut = Box<dyn FnMut(*mut lua_State) -> Result<c_int> + 'static>;
type LuaFnOnce = Box<dyn FnOnce(*mut lua_State) -> Result<c_int> + 'static>;

impl LuaRef {
    pub(crate) fn from_fn_mut<A, R, F>(mut fun: F) -> Self
    where
        A: super::LuaPoppable,
        R: super::LuaPushable,
        F: FnMut(A) -> crate::Result<R> + 'static,
    {
        unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
            let fun = {
                let idx = lua_upvalueindex(1);
                let upv = lua_touserdata(lstate, idx) as *mut LuaFnMut;
                &mut **upv
            };

            fun(lstate).unwrap_or_else(|_err| {
                // TODO
                panic!("what to do here?");
            })
        }

        let r#ref = super::with_state(move |lstate| unsafe {
            let fun =
                Box::new(move |lstate| fun(A::pop(lstate)?)?.push(lstate));
            let ud = lua_newuserdata(lstate, mem::size_of::<LuaFnMut>());
            ptr::write(ud as *mut LuaFnMut, fun);
            lua_pushcclosure(lstate, c_fun, 1);
            luaL_ref(lstate, LUA_REGISTRYINDEX)
        });

        Self(r#ref)
    }

    pub(crate) fn from_fn_once<A, R, F>(mut fun: F) -> Self
    where
        A: super::LuaPoppable,
        R: super::LuaPushable,
        F: FnOnce(A) -> crate::Result<R> + 'static,
    {
        unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
            let fun = {
                let idx = lua_upvalueindex(1);
                let upv = lua_touserdata(lstate, idx) as *mut LuaFnOnce;
                Box::from_raw(&mut **upv)
            };

            fun(lstate).unwrap_or_else(|_err| {
                // TODO
                panic!("what to do here?");
            })
        }

        let r#ref = super::with_state(move |lstate| unsafe {
            let fun = Box::new(move |l| fun(A::pop(l)?)?.push(l));
            let ud = lua_newuserdata(lstate, mem::size_of::<LuaFnOnce>());
            ptr::write(ud as *mut LuaFnOnce, fun);
            lua_pushcclosure(lstate, c_fun, 1);
            luaL_ref(lstate, LUA_REGISTRYINDEX)
        });

        Self(r#ref)
    }

    /// Removes the stored reference from the Lua registry.
    pub(crate) fn unref(self) {
        super::with_state(move |lstate| unsafe {
            luaL_unref(lstate, LUA_REGISTRYINDEX, self.0);
        })
    }
}
