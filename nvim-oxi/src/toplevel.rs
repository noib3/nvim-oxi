use crate::lua::{self, ffi::*, Function};
use crate::macros::cstr;
use crate::Result;

/// Binding to the global Lua `print` function. It uses the same syntax as
/// Rust's `format!` macro and redirects its output to the Neovim message area.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi as nvim;
///
/// nvim::print!("Goodbye {}..", String::from("Earth"));
/// nvim::print!("Hello {planet}!", planet = "Mars");
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::__print(::std::fmt::format(format_args!($($arg)*)));
    }}
}

/// Prints a message to the Neovim message area.
#[doc(hidden)]
pub fn __print(text: impl Into<String>) {
    lua::with_state(move |lstate| unsafe {
        let text = text.into();
        lua_getglobal(lstate, cstr!("print"));
        lua_pushlstring(
            lstate,
            text.as_ptr() as *const libc::c_char,
            text.len(),
        );
        lua_call(lstate, 1, 0);
    });
}

/// Binding to `vim.schedule`.
///
/// Schedules a callback to be invoked soon by the main event-loop. Useful to
/// avoid [`textlock`](https://neovim.io/doc/user/eval.html#textlock) or other
/// temporary restrictions.
pub fn schedule<F>(fun: F)
where
    F: FnOnce(()) -> Result<()> + 'static,
{
    // https://github.com/neovim/neovim/blob/master/src/nvim/lua/executor.c#L316
    //
    // Unfortunately the `nlua_schedule` C function is not exported, so we have
    // to call the Lua function instead.
    lua::with_state(move |lstate| unsafe {
        // Put `vim.schedule` on the stack.
        lua_getglobal(lstate, cstr!("vim"));
        lua_getfield(lstate, -1, cstr!("schedule"));

        // Store the function in the registry and put a reference to it on the
        // stack.
        let fun = Function::from_fn_once(fun);
        lua_rawgeti(lstate, LUA_REGISTRYINDEX, fun.0);

        lua_call(lstate, 1, 0);

        // Pop `vim` off the stack and remove the function from the registry.
        lua_pop(lstate, 1);
        luaL_unref(lstate, LUA_REGISTRYINDEX, fun.0);
    });
}
