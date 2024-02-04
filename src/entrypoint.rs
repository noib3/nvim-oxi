//! Contains the entrypoint of the final plugin.

use std::ffi::c_int;

use luajit::{self as lua, ffi::lua_State, Pushable};

/// The entrypoint of the plugin.
///
/// Initializes the Lua state, executes the entrypoint function and pushes the
/// result on the stack.
#[doc(hidden)]
pub unsafe fn entrypoint<R, E>(
    lua_state: *mut lua_State,
    body: fn() -> Result<R, E>,
) -> c_int
where
    R: Pushable,
    E: std::error::Error,
{
    lua::init(lua_state);

    #[cfg(feature = "libuv")]
    libuv::init(lua_state);

    match body() {
        Ok(api) => api.push(lua_state).unwrap(),
        Err(err) => lua::utils::handle_error(lua_state, &err),
    }
}
