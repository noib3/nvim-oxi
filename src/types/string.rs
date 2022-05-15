use libc::{c_char, size_t};
use std::borrow::Cow;
use std::ffi::CStr;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L77
#[repr(C)]
pub struct String {
    data: *mut c_char,
    // Using `usize` here instead of `std::os::raw::c_size_t` until
    // https://github.com/rust-lang/rust/issues/88345 gets stabilized.
    size: size_t,
}

impl String {
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.data) }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }

    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        std::string::String::from_utf8_lossy(self.as_c_str().to_bytes())
    }
}
