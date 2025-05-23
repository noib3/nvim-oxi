#![allow(non_snake_case)]

use core::ffi::{c_char, c_double, c_int, c_void};
use core::marker::{PhantomData, PhantomPinned};

#[repr(C)]
pub struct State {
    _data: [u8; 0],

    /// This marker ensures the struct is not `Send`, `Sync` and `Unpin` (the
    /// raw pointer is neither `Send` nor `Sync`, `PhantomPinned` is not
    /// `Unpin`).
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

// Pseudo-indices.
pub const LUA_REGISTRYINDEX: c_int = -10000;
pub const LUA_ENVIRONINDEX: c_int = -10001;
pub const LUA_GLOBALSINDEX: c_int = -10002;

pub const fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_GLOBALSINDEX - i
}

// Thread status.
pub const LUA_OK: c_int = 0;
pub const LUA_ERRRUN: c_int = 2;
pub const LUA_ERRMEM: c_int = 4;
pub const LUA_ERRERR: c_int = 5;

// Type codes.
pub const LUA_TNONE: c_int = -1;
pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TSTRING: c_int = 4;
pub const LUA_TTABLE: c_int = 5;
pub const LUA_TFUNCTION: c_int = 6;
pub const LUA_TUSERDATA: c_int = 7;
pub const LUA_TTHREAD: c_int = 8;

// https://www.lua.org/manual/5.1/manual.html#lua_CFunction
pub type CFunction = unsafe extern "C" fn(L: *mut State) -> c_int;

// https://www.lua.org/manual/5.1/manual.html#lua_Integer
pub type Integer = isize;

// https://www.lua.org/manual/5.1/manual.html#lua_Number
pub type Number = c_double;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "lua51", kind = "raw-dylib")
)]
unsafe extern "C" {
    // https://www.lua.org/manual/5.1/manual.html#lua_call
    pub fn lua_call(L: *mut State, nargs: c_int, nresults: c_int);

    /// Binding to [`lua_createtable()`] (-0, +1).
    ///
    /// Creates a new empty table and pushes it onto the stack. The new table
    /// has space pre-allocated for `narr` array elements and `nrec` non-array
    /// elements.
    ///
    /// This pre-allocation is useful when you know exactly how many
    /// elements the table will have. Otherwise you can use the function
    /// `lua_newtable`.
    ///
    /// [`lua_createtable`]: https://www.lua.org/manual/5.1/manual.html#lua_createtable
    pub fn lua_createtable(L: *mut State, narr: c_int, nrec: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_error
    pub fn lua_error(L: *mut State) -> !;

    // https://www.lua.org/manual/5.1/manual.html#lua_call
    pub fn lua_getfield(L: *mut State, index: c_int, k: *const c_char);

    // https://www.lua.org/manual/5.1/manual.html#lua_getmetatable
    pub fn lua_getmetatable(L: *mut State, index: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_gettop
    pub fn lua_gettop(L: *mut State) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_newuserdata
    pub fn lua_newuserdata(L: *mut State, size: usize) -> *mut c_void;

    // https://www.lua.org/manual/5.1/manual.html#lua_next
    pub fn lua_next(L: *mut State, index: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_objlen
    pub fn lua_objlen(L: *mut State, index: c_int) -> usize;

    // https://www.lua.org/manual/5.1/manual.html#lua_pcall
    pub fn lua_pcall(
        L: *mut State,
        nargs: c_int,
        nresults: c_int,
        errorfunc: c_int,
    ) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_pushinteger
    pub fn lua_pushboolean(L: *mut State, n: Integer);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushcclosure
    pub fn lua_pushcclosure(L: *mut State, r#fn: CFunction, n: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushinteger
    pub fn lua_pushinteger(L: *mut State, n: Integer);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushlightuserdata
    pub fn lua_pushlightuserdata(L: *mut State, p: *mut c_void);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushlstring
    pub fn lua_pushlstring(L: *mut State, s: *const c_char, len: usize);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushnil
    pub fn lua_pushnil(L: *mut State);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushnumber
    pub fn lua_pushnumber(L: *mut State, n: Number);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushstring
    pub fn lua_pushstring(L: *mut State, s: *const c_char);

    // https://www.lua.org/manual/5.1/manual.html#lua_pushvalue
    pub fn lua_pushvalue(L: *mut State, index: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_rawgeti
    pub fn lua_rawgeti(L: *mut State, index: c_int, n: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_rawset
    pub fn lua_rawset(L: *mut State, index: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_rawseti
    pub fn lua_rawseti(L: *mut State, index: c_int, n: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_settop
    pub fn lua_settop(L: *mut State, index: c_int);

    // https://www.lua.org/manual/5.1/manual.html#lua_toboolean
    pub fn lua_toboolean(L: *mut State, index: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_tointeger
    pub fn lua_tointeger(L: *mut State, index: c_int) -> Integer;

    // https://www.lua.org/manual/5.1/manual.html#lua_tolstring
    pub fn lua_tolstring(
        L: *mut State,
        index: c_int,
        len: *mut usize,
    ) -> *const c_char;

    // https://www.lua.org/manual/5.1/manual.html#lua_tonumber
    pub fn lua_tonumber(L: *mut State, index: c_int) -> Number;

    // https://www.lua.org/manual/5.1/manual.html#lua_touserdata
    pub fn lua_touserdata(L: *mut State, index: c_int) -> *mut c_void;

    // https://www.lua.org/manual/5.1/manual.html#lua_type
    pub fn lua_type(L: *mut State, index: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#lua_typename
    pub fn lua_typename(L: *mut State, tp: c_int) -> *const c_char;

    // Lua auxiliary library.

    // https://www.lua.org/manual/5.1/manual.html#luaL_error
    pub fn luaL_error(L: *mut State, fmt: *const c_char, ...) -> !;

    // https://www.lua.org/manual/5.1/manual.html#luaL_ref
    pub fn luaL_ref(L: *mut State, t: c_int) -> c_int;

    // https://www.lua.org/manual/5.1/manual.html#luaL_unref
    pub fn luaL_unref(L: *mut State, t: c_int, r#ref: c_int);
}

// https://www.lua.org/manual/5.1/manual.html#lua_getglobal
pub unsafe fn lua_getglobal(L: *mut State, name: *const c_char) {
    lua_getfield(L, LUA_GLOBALSINDEX, name)
}

// https://www.lua.org/manual/5.1/manual.html#lua_pop
pub unsafe fn lua_pop(L: *mut State, n: c_int) {
    lua_settop(L, -n - 1)
}

// https://www.lua.org/manual/5.1/manual.html#lua_pushcfunction
pub unsafe fn lua_pushcfunction(L: *mut State, r#fn: CFunction) {
    lua_pushcclosure(L, r#fn, 0)
}

// https://www.lua.org/manual/5.1/manual.html#lua_tostring
pub unsafe fn lua_tostring(L: *mut State, index: c_int) -> *const c_char {
    lua_tolstring(L, index, std::ptr::null_mut())
}

// https://www.lua.org/manual/5.1/manual.html#luaL_typename
pub unsafe fn luaL_typename(L: *mut State, index: c_int) -> *const c_char {
    lua_typename(L, lua_type(L, index))
}
