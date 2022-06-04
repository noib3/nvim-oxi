use crate::lua::{self, LuaFnOnce};
use crate::macros::cstr;

/// Binding to the global Lua `print` function. It uses the same syntax as
/// Rust's `format!` macro and redirects its output to the Neovim message area.
///
/// # Examples
///
/// ```rust
/// nvim_oxi::print!("Hello {planet}!", planet = "Mars");
/// ```
#[macro_export]
macro_rules! nprint {
    ($($arg:tt)*) => {{
        crate::print(::std::fmt::format(format_args!($($arg)*)));
    }}
}

pub use nprint as print;

/// Prints a message to the Neovim message area.
#[doc(hidden)]
pub fn print(text: impl Into<String>) {
    lua::with_state(move |lstate| unsafe {
        let text = text.into();
        lua::lua_getglobal(lstate, cstr!("print"));
        lua::lua_pushlstring(
            lstate,
            text.as_ptr() as *const libc::c_char,
            text.len(),
        );
        lua::lua_call(lstate, 1, 0);
    });
}

/// Binding to `vim.schedule`.
///
/// Schedules a callback to be invoked soon by the main event-loop. Useful to
/// avoid textlock or other temporary restrictions.
pub fn schedule<F>(fun: F)
where
    F: FnOnce(()) -> crate::Result<()> + 'static,
{
    // https://github.com/neovim/neovim/blob/master/src/nvim/lua/executor.c#L316
    //
    // Unfortunately the `nlua_schedule` C function is not exported, so we have
    // to call the Lua function instead.
    lua::with_state(move |lstate| unsafe {
        // Put `vim.schedule` on the stack.
        lua::lua_getglobal(lstate, cstr!("vim"));
        lua::lua_getfield(lstate, -1, cstr!("schedule"));

        // Store the function in the registry and put a reference to it on the
        // stack.
        let fun = LuaFnOnce::from(fun);
        lua::lua_rawgeti(lstate, lua::LUA_REGISTRYINDEX, fun.0);

        lua::lua_call(lstate, 1, 0);

        // Pop `vim` off the stack and remove the reference from the registry.
        lua::lua_pop(lstate, 1);
        lua::luaL_unref(lstate, lua::LUA_REGISTRYINDEX, fun.0);
    });
}
