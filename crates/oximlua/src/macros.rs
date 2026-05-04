use crate::lua as olua;

// Taken from https://github.com/khvzak/mlua/blob/master/src/macros.rs#L11
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const [::std::ffi::c_char]
            as *const ::std::ffi::c_char
    };
}

/// Same as [`std::dbg!`](dbg) but writes to the Neovim message area instead of
/// stdout.
///
/// [dbg]: https://doc.rust-lang.org/std/macro.dbg.html
#[macro_export]
macro_rules! dbg {
    () => {
        $crate::print!("[{}:{}]", ::core::file!(), ::core::line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::print!("[{}:{}] {} = {:#?}",
                    ::core::file!(), ::core::line!(), ::core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

/// Same as [`std::print!`](print) but writes to the Neovim message area
/// instead of stdout.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi as nvim;
///
/// nvim::print!("Goodbye {}..", "Earth");
/// nvim::print!("Hello {planet}!", planet = "Mars");
/// ```
///
/// [print]: https://doc.rust-lang.org/std/macro.print.html
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::__print(::std::fmt::format(format_args!($($arg)*)));
    }}
}

/// Prints a message to the Neovim message area.
#[doc(hidden)]
pub fn __print(text: impl Into<String>) {
    let lua = olua::get_lua();
    let print: mlua::Function = lua.globals().get("print").unwrap();
    print.call::<()>(text.into()).unwrap();
}
