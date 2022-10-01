use std::ffi::c_char;

use crate::ffi::*;

// Taken from https://github.com/khvzak/mlua/blob/master/src/macros.rs#L11
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const [::std::ffi::c_char]
            as *const ::std::ffi::c_char
    };
}

pub use cstr;

macro_rules! count {
    () => {0i32};
    ($x:tt $($xs:tt)*) => {1i32 + count!($($xs)*)};
}

pub(crate) use count;

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
    unsafe {
        crate::with_state(move |lstate| {
            let text = text.into();
            lua_getglobal(lstate, cstr!("print"));
            lua_pushlstring(
                lstate,
                text.as_ptr() as *const c_char,
                text.len(),
            );
            lua_call(lstate, 1, 0);
        })
    };
}
