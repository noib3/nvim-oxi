use crate::lua::{self, ffi, macros::*};
use crate::Result;

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
        let _ = self::print(::std::fmt::format(format_args!($($arg)*)));
    }}
}

pub use nprint as print;

/// Prints a message to the Neovim message area. Fails if the provided string
/// constains a null byte.
#[doc(hidden)]
pub fn print(text: impl Into<String>) -> Result<()> {
    let text = std::ffi::CString::new(text.into())?;

    lua::with_state(move |lstate| unsafe {
        ffi::lua_getglobal(lstate, cstr!("print"));
        ffi::lua_pushstring(lstate, text.as_ptr());
        ffi::lua_call(lstate, 1, 0);
    });

    Ok(())
}
