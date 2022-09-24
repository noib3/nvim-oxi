// Taken from https://github.com/khvzak/mlua/blob/master/src/macros.rs#L11
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const [::std::ffi::c_char]
            as *const ::std::ffi::c_char
    };
}

pub(crate) use cstr;
