#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_double, c_int, c_void};
use std::marker::{PhantomData, PhantomPinned};

#[repr(C)]
#[doc(hidden)]
pub struct lua_State {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

// Pseudo-indices.
pub(crate) const LUA_REGISTRYINDEX: c_int = -10000;
pub(crate) const LUA_ENVIRONINDEX: c_int = -10001;
pub(crate) const LUA_GLOBALSINDEX: c_int = -10002;

pub(crate) const fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_GLOBALSINDEX - i
}

// Thread status.
pub(crate) const LUA_OK: c_int = 0;
pub(crate) const LUA_ERRRUN: c_int = 2;
pub(crate) const LUA_ERRMEM: c_int = 4;
pub(crate) const LUA_ERRERR: c_int = 5;

// Type codes.
pub(crate) const LUA_TNONE: c_int = -1;
pub(crate) const LUA_TNIL: c_int = 0;
pub(crate) const LUA_TBOOLEAN: c_int = 1;
pub(crate) const LUA_TLIGHTUSERDATA: c_int = 2;
pub(crate) const LUA_TNUMBER: c_int = 3;
pub(crate) const LUA_TSTRING: c_int = 4;
pub(crate) const LUA_TTABLE: c_int = 5;
pub(crate) const LUA_TFUNCTION: c_int = 6;
pub(crate) const LUA_TUSERDATA: c_int = 7;
pub(crate) const LUA_TTHREAD: c_int = 8;

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

    // https://www.lua.org/manual/5.1/manual.html#lua_createtable
    pub(crate) fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_error
    pub(crate) fn lua_error(L: *mut lua_State) -> !;

    // https://www.lua.org/manual/5.1/manual.html#lua_call
    pub(crate) fn lua_getfield(
        L: *mut lua_State,
        index: c_int,
        k: *const c_char,
    );

    // https://www.lua.org/manual/5.1/manual.html#lua_getmetatable
    pub(crate) fn lua_getmetatable(L: *mut lua_State, index: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_gettop
    pub(crate) fn lua_gettop(L: *mut lua_State) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_newuserdata
    pub(crate) fn lua_newuserdata(
        L: *mut lua_State,
        size: usize,
    ) -> *mut c_void;

    // https://www.lua.org/manual/5.1/manual.html#lua_next
    pub(crate) fn lua_next(L: *mut lua_State, index: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_objlen
    pub(crate) fn lua_objlen(L: *mut lua_State, index: c_int) -> usize;

    // https://www.lua.org/manual/5.1/manual.html#lua_pcall
    pub(crate) fn lua_pcall(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        errorfunc: c_int,
    ) -> c_int;

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
        len: usize,
    );

    // https://www.lua.org/manual/5.1/manual.html#lua_pushnil
    pub(crate) fn lua_pushnil(L: *mut lua_State);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushnumber
    pub(crate) fn lua_pushnumber(L: *mut lua_State, n: lua_Number);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushstring
    pub(crate) fn lua_pushstring(L: *mut lua_State, s: *const c_char);

    // https://www.lua.org/manual/5.1/manual.html#lua_rawgeti
    pub(crate) fn lua_rawgeti(L: *mut lua_State, index: c_int, n: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_rawset
    pub(crate) fn lua_rawset(L: *mut lua_State, index: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_rawseti
    pub(crate) fn lua_rawseti(L: *mut lua_State, index: c_int, n: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_settop
    pub(crate) fn lua_settop(L: *mut lua_State, index: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_toboolean
    pub(crate) fn lua_toboolean(L: *mut lua_State, index: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_tointeger
    pub(crate) fn lua_tointeger(
        L: *mut lua_State,
        index: c_int,
    ) -> lua_Integer;

    // https://www.lua.org/manual/5.1/manual.html#lua_tolstring
    pub(crate) fn lua_tolstring(
        L: *mut lua_State,
        index: c_int,
        len: *mut usize,
    ) -> *const c_char;

    // https://www.lua.org/manual/5.1/manual.html#lua_tonumber
    pub(crate) fn lua_tonumber(L: *mut lua_State, index: c_int) -> lua_Number;

    // https://www.lua.org/manual/5.1/manual.html#lua_touserdata
    pub(crate) fn lua_touserdata(
        L: *mut lua_State,
        index: c_int,
    ) -> *mut c_void;

    // https://www.lua.org/manual/5.1/manual.html#lua_type
    pub(crate) fn lua_type(L: *mut lua_State, index: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_typename
    pub(crate) fn lua_typename(L: *mut lua_State, tp: c_int) -> *const c_char;
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

// https://www.lua.org/manual/5.1/manual.html#lua_tostring
#[inline(always)]
pub(crate) unsafe fn lua_tostring(
    L: *mut lua_State,
    index: c_int,
) -> *const c_char {
    lua_tolstring(L, index, std::ptr::null_mut())
}

// Lua auxiliary library.
extern "C" {
    // https://www.lua.org/manual/5.1/manual.html#luaL_error
    pub(crate) fn luaL_error(L: *mut lua_State, fmt: *const c_char, ...) -> !;

    // https://www.lua.org/manual/5.1/manual.html#luaL_ref
    pub(crate) fn luaL_ref(L: *mut lua_State, t: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#luaL_unref
    pub(crate) fn luaL_unref(L: *mut lua_State, t: c_int, r#ref: c_int);
}

// https://www.lua.org/manual/5.1/manual.html#luaL_typename
#[inline(always)]
pub(crate) unsafe fn luaL_typename(
    L: *mut lua_State,
    index: c_int,
) -> *const c_char {
    lua_typename(L, lua_type(L, index))
}
