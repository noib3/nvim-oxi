use std::borrow::Cow;
use std::ffi::{CStr, CString, NulError};
use std::mem;
use std::ptr;

use libc::{c_char, size_t};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L77
#[repr(C)]
pub(crate) struct NvimString {
    data: *mut c_char,
    size: size_t,
}

impl NvimString {
    #[inline]
    pub fn from_c_string(c_string: CString) -> Self {
        let mut bytes = c_string.into_bytes();
        let mut uninit = std::mem::MaybeUninit::<NvimString>::uninit();

        let ptr = uninit.as_mut_ptr();

        unsafe {
            ptr::addr_of_mut!((*ptr).size).write(bytes.len());
        }

        unsafe {
            ptr::addr_of_mut!((*ptr).data).write(bytes.as_mut_ptr().cast());
        }

        mem::forget(bytes);

        unsafe { uninit.assume_init() }
    }

    #[inline]
    pub fn new<Raw: Into<Vec<u8>>>(raw: Raw) -> Result<Self, NulError> {
        Ok(Self::from_c_string(CString::new(raw)?))
    }

    #[allow(dead_code)]
    #[inline]
    pub unsafe fn new_unchecked<Raw: Into<Vec<u8>>>(raw: Raw) -> Self {
        Self::from_c_string(CString::from_vec_unchecked(raw.into()))
    }

    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.data) }
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    #[allow(dead_code)]
    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }

    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        std::string::String::from_utf8_lossy(self.as_c_str().to_bytes())
    }
}
