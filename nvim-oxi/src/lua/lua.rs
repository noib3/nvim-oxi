use std::{mem, ptr};

use libc::{c_int, c_void};
use nvim_types::{Integer, LuaRef, Object};
use once_cell::unsync::OnceCell;

use super::ffi::{self, lua_Integer, lua_State};
use crate::Result;

thread_local! {
    static LUA: OnceCell<*mut lua_State> = OnceCell::new();
}

pub(crate) type CallbackMut<A, R> = Box<dyn FnMut(A) -> Result<R> + 'static>;
pub(crate) type CallbackOnce<A, R> = Box<dyn FnOnce(A) -> Result<R> + 'static>;

/// TODO: docs
#[inline(always)]
pub(crate) fn init_state(lstate: *mut lua_State) {
    LUA.with(|lua| lua.set(lstate).expect("couldn't initialize Lua state"));
}

/// TODO: docs
#[inline(always)]
pub(crate) fn with_state<F, R>(fun: F) -> R
where
    F: FnOnce(*mut lua_State) -> R,
{
    LUA.with(move |lua| unsafe { fun(*(lua.get().unwrap_unchecked())) })
}

/// TODO: docs
pub(crate) fn to_ref_mut<R>(
    fun: CallbackMut<(), R>,
    _nargs: c_int,
) -> Result<c_int> {
    unsafe extern "C" fn test<R>(lstate: *mut lua_State) -> c_int {
        let fun = {
            let idx = ffi::lua_upvalueindex(1);
            let upvalue =
                ffi::lua_touserdata(lstate, idx) as *mut CallbackMut<(), R>;
            &mut **upvalue
        };
        let result = Box::new(fun(()));
        ffi::lua_pushlightuserdata(
            lstate,
            Box::leak(result) as *mut Result<R> as *mut c_void,
        );
        let r#ref = ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX);
        ffi::lua_pushinteger(lstate, r#ref as lua_Integer);
        1
    }

    let r#ref = self::with_state(move |lstate| unsafe {
        let ud =
            ffi::lua_newuserdata(lstate, mem::size_of::<CallbackMut<(), R>>())
                as *mut CallbackMut<(), R>;
        ptr::write(ud, fun);
        ffi::lua_pushcclosure(lstate, test::<R>, 1);
        ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX)
    });

    Ok(r#ref)
}

/// TODO: docs
pub(crate) fn to_ref_once<R>(
    fun: CallbackOnce<(), R>,
    _nargs: c_int,
) -> Result<c_int> {
    unsafe extern "C" fn test<R>(lstate: *mut lua_State) -> c_int {
        let fun = {
            let idx = ffi::lua_upvalueindex(1);
            let upvalue =
                ffi::lua_touserdata(lstate, idx) as *mut CallbackOnce<(), R>;
            Box::from_raw(&mut **upvalue as *mut (dyn FnOnce(()) -> Result<R>))
        };
        let result = Box::new(fun(()));
        ffi::lua_pushlightuserdata(
            lstate,
            Box::leak(result) as *mut Result<R> as *mut c_void,
        );
        let r#ref = ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX);
        ffi::lua_pushinteger(lstate, r#ref as lua_Integer);
        1
    }

    let r#ref = self::with_state(move |lstate| unsafe {
        let ud = ffi::lua_newuserdata(
            lstate,
            mem::size_of::<CallbackOnce<(), R>>(),
        ) as *mut CallbackOnce<(), R>;
        ptr::write(ud, fun);
        ffi::lua_pushcclosure(lstate, test::<R>, 1);
        ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX)
    });

    Ok(r#ref)
}

pub(crate) fn to_result<R>(obj: Object) -> Result<R> {
    // `obj` is usually the result of calling a Neovim C function and should
    // contain a number which is the index in the Lua registry where the
    // pointer pointing to the results to retrieve is stored.
    let r#ref: LuaRef = Integer::try_from(obj)?.try_into()?;
    self::with_state(|lstate| unsafe {
        ffi::lua_rawgeti(lstate, ffi::LUA_REGISTRYINDEX, r#ref);
        let ud = ffi::lua_touserdata(lstate, -1) as *mut Result<R>;
        ffi::lua_pop(lstate, 1);
        let results = (*Box::from_raw(ud))?;
        ffi::luaL_unref(lstate, ffi::LUA_REGISTRYINDEX, r#ref);
        Ok(results)
    })
}
