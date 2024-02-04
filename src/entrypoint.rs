use core::ffi::c_int;

use luajit::{ffi::lua_State, Pushable};

/// The entrypoint of the plugin.
///
/// Initializes the Lua state, executes the entrypoint function and pushes the
/// result on the stack.
#[inline(always)]
pub unsafe fn entrypoint<T>(
    lua_state: *mut lua_State,
    body: fn() -> T,
) -> c_int
where
    T: Pushable,
{
    luajit::init(lua_state);

    #[cfg(feature = "libuv")]
    libuv::init(lua_state);

    match body().push(lua_state) {
        Ok(num_pushed) => num_pushed,
        Err(lua_err) => luajit::utils::push_error(&lua_err, lua_state),
    }
}
