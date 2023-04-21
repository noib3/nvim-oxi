use oxi_luajit::{self as lua, ffi::*, macros::cstr};
use oxi_types::Function;

use crate::Result;

/// Binding to [`vim.schedule()`][1].
///
/// Schedules a callback to be invoked soon by the main event-loop. Useful to
/// avoid [`textlock`][2] or other temporary restrictions.
///
/// [1]: https://neovim.io/doc/user/lua.html#vim.schedule()
/// [2]: https://neovim.io/doc/user/eval.html#textlock
pub fn schedule<F>(fun: F)
where
    F: FnOnce(()) -> Result<()> + 'static,
{
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/lua/executor.c#L363
    //
    // Unfortunately the `nlua_schedule` C function is not exported, so we have
    // to call the Lua function instead.
    unsafe {
        lua::with_state(move |lstate| {
            // Put `vim.schedule` on the stack.
            lua_getglobal(lstate, cstr!("vim"));
            lua_getfield(lstate, -1, cstr!("schedule"));

            // Store the function in the registry and put a reference to it on
            // the stack.
            let fun = Function::from_fn_once(fun);
            lua_rawgeti(lstate, LUA_REGISTRYINDEX, fun.lua_ref());

            lua_call(lstate, 1, 0);

            // Pop `vim` off the stack and remove the function from the registry.
            lua_pop(lstate, 1);
            luaL_unref(lstate, LUA_REGISTRYINDEX, fun.lua_ref());
        })
    };
}
