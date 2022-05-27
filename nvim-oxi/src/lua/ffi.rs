#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::marker::{PhantomData, PhantomPinned};

use libc::{c_char, c_double, c_int, c_void, size_t};

// Pseudo-indices.
pub(crate) const LUA_REGISTRYINDEX: c_int = -10000;
pub(crate) const LUA_ENVIRONINDEX: c_int = -10001;
pub(crate) const LUA_GLOBALSINDEX: c_int = -10002;

pub(crate) const fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_GLOBALSINDEX - i
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct lua_State {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

// https://www.lua.org/manual/5.1/manual.html#lua_CFunction
pub(crate) type lua_CFunction =
    unsafe extern "C" fn(L: *mut lua_State) -> c_int;

// https://www.lua.org/manual/5.1/manual.html#lua_Integer
pub(crate) type lua_Integer = isize;

// https://www.lua.org/manual/5.1/manual.html#lua_Number
pub(crate) type lua_Number = c_double;

extern "C" {
    // https://www.lua.org/manual/5.1/manual.html#lua_call
    pub(crate) fn lua_call(L: *mut lua_State, nargs: c_int, nresults: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_call
    pub(crate) fn lua_getfield(
        L: *mut lua_State,
        index: c_int,
        k: *const c_char,
    );

    // https://www.lua.org/manual/5.1/manual.html#lua_gettop
    pub(crate) fn lua_gettop(L: *mut lua_State) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_rawgeti
    pub(crate) fn lua_rawgeti(L: *mut lua_State, index: c_int, n: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_newuserdata
    pub(crate) fn lua_newuserdata(
        L: *mut lua_State,
        size: size_t,
    ) -> *mut c_void;

    // https://www.lua.org/manual/5.1/manual.html#lua_pushinteger
    pub(crate) fn lua_pushboolean(L: *mut lua_State, n: lua_Integer);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushcclosure
    pub(crate) fn lua_pushcclosure(
        L: *mut lua_State,
        r#fn: lua_CFunction,
        n: c_int,
    );

    // https://www.lua.org/manual/5.1/manual.html#lua_pushinteger
    pub(crate) fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushlightuserdata
    pub(crate) fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushlstring
    pub(crate) fn lua_pushlstring(
        L: *mut lua_State,
        s: *const c_char,
        len: size_t,
    );

    // https://www.lua.org/manual/5.1/manual.html#lua_pushnil
    pub(crate) fn lua_pushnil(L: *mut lua_State);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushnumber
    pub(crate) fn lua_pushnumber(L: *mut lua_State, n: lua_Number);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushstring
    pub(crate) fn lua_pushstring(L: *mut lua_State, s: *const c_char);

    // https://www.lua.org/manual/5.1/manual.html#lua_rawseti
    pub(crate) fn lua_rawseti(L: *mut lua_State, index: c_int, n: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_settop
    pub(crate) fn lua_settop(L: *mut lua_State, index: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_touserdata
    pub(crate) fn lua_touserdata(
        L: *mut lua_State,
        index: c_int,
    ) -> *mut c_void;
}

// https://www.lua.org/manual/5.1/manual.html#lua_getglobal
#[inline(always)]
pub(crate) unsafe fn lua_getglobal(L: *mut lua_State, name: *const c_char) {
    lua_getfield(L, LUA_GLOBALSINDEX, name)
}

// https://www.lua.org/manual/5.1/manual.html#lua_pop
#[inline(always)]
pub(crate) unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -n - 1)
}

// https://www.lua.org/manual/5.1/manual.html#lua_pushcfunction
#[inline(always)]
pub(crate) unsafe fn lua_pushcfunction(
    L: *mut lua_State,
    r#fn: lua_CFunction,
) {
    lua_pushcclosure(L, r#fn, 0)
}

// Lua auxiliary library.
extern "C" {
    // https://www.lua.org/manual/5.1/manual.html#luaL_ref
    pub(crate) fn luaL_ref(L: *mut lua_State, t: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#luaL_unref
    pub(crate) fn luaL_unref(L: *mut lua_State, t: c_int, r#ref: c_int);
}
