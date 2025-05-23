use core::marker::PhantomData;
use core::{ffi, slice};

use crate::String as NvimString;

/// TODO: docs.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NvimStr<'a> {
    data: *mut ffi::c_char,
    len: usize,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> NvimStr<'a> {
    /// Converts the [`NvimStr`] into a byte slice, *not* including the final
    /// null byte.
    ///
    /// If you want the final null byte to be included in the slice, use
    /// [`as_bytes_with_nul`](Self::as_bytes_with_nul) instead.
    #[inline]
    pub const fn as_bytes(&self) -> &'a [u8] {
        let bytes = self.as_bytes_with_nul();
        unsafe { slice::from_raw_parts(bytes.as_ptr(), bytes.len() - 1) }
    }

    /// Converts the [`NvimStr`] into a byte slice, including the final
    /// null byte.
    ///
    /// If you don't want the final null byte to be included in the slice, use
    /// [`as_bytes`](Self::as_bytes) instead.
    #[inline]
    pub const fn as_bytes_with_nul(&self) -> &'a [u8] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe {
                slice::from_raw_parts(self.as_ptr() as *const u8, self.len + 1)
            }
        }
    }

    /// Returns a raw pointer to the [`NvimStr`]'s buffer.
    #[inline]
    pub const fn as_ptr(&self) -> *const ffi::c_char {
        self.data as *const ffi::c_char
    }

    /// Returns a raw pointer to the [`NvimStr`]'s buffer.
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut ffi::c_char {
        self.data
    }

    /// Creates an `NvimStr` from a pointer to the underlying data and a
    /// length.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid for `len + 1`
    /// elements and that the last element is a null byte.
    #[inline]
    pub unsafe fn from_raw_parts(data: *mut ffi::c_char, len: usize) -> Self {
        Self { data, len, _lifetime: PhantomData }
    }

    /// Returns `true` if the [`NvimStr`] has a length of zero, not including
    /// the final null byte.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of the [`NvimStr`], *not* including the final null
    /// byte.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns the length of the [`NvimStr`], *not* including the final null
    /// byte.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes at `old_len..new_len` are
    /// initialized.
    #[inline]
    pub const unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }
}

impl From<NvimString> for NvimStr<'_> {
    #[inline]
    fn from(string: NvimString) -> Self {
        string.into_nvim_str()
    }
}

impl<'a> From<&'a NvimString> for NvimStr<'a> {
    #[inline]
    fn from(string: &'a NvimString) -> Self {
        string.as_nvim_str()
    }
}
