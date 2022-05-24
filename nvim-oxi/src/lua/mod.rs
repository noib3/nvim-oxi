//! This module constains definitions from `https://www.lua.org/source/5.1/lua.h.html`

pub(crate) mod ffi;
mod lua;
pub(crate) mod macros;

pub(crate) use lua::{lua_State, LUA};
