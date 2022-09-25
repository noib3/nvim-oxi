//! Contains the entrypoint of the final plugin.

use std::ffi::c_int;

use luajit_bindings::{self as lua, ffi::lua_State, LuaPushable};

/// The entrypoint of the plugin.
///
/// Initializes the Lua state, executes the entrypoint function and pushes the
/// result on the stack.
#[doc(hidden)]
pub unsafe fn entrypoint<R>(
    lstate: *mut lua_State,
    body: fn() -> crate::Result<R>,
) -> c_int
where
    R: LuaPushable,
{
    lua::init(lstate);

    #[cfg(feature = "libuv")]
    libuv_bindings::init(lstate);

    match body() {
        Ok(api) => api.push(lstate).unwrap(),
        Err(err) => lua::utils::handle_error(lstate, &err),
    }
}
