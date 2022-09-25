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
