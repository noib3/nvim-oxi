use core::marker::PhantomData;
use core::str::Utf8Error;
use core::{cmp, ffi, fmt, hash, slice};
use std::borrow::Cow;

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
        self.as_bytes_inner(false)
    }

    /// Converts the [`NvimStr`] into a byte slice, including the final
    /// null byte.
    ///
    /// If you don't want the final null byte to be included in the slice, use
    /// [`as_bytes`](Self::as_bytes) instead.
    #[inline]
    pub const fn as_bytes_with_nul(&self) -> &'a [u8] {
        self.as_bytes_inner(false)
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

    /// Yields a string slice if the [`NvimStr`]'s contents are valid UTF-8.
    #[inline]
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts the [`NvimStr`] into a [`String`].
    ///
    /// If it already holds a valid UTF-8 byte sequence no allocation is made.
    /// If it doesn't, the contents of the [`NvimStr`] are is copied and all
    /// invalid sequences are replaced with `�`.
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        std::string::String::from_utf8_lossy(self.as_bytes())
    }

    #[inline]
    pub(crate) fn reborrow(&self) -> NvimStr<'_> {
        NvimStr { ..*self }
    }

    #[inline]
    const fn as_bytes_inner(&self, with_nul: bool) -> &'a [u8] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe {
                slice::from_raw_parts(
                    self.as_ptr() as *const u8,
                    self.len + with_nul as usize,
                )
            }
        }
    }
}

impl fmt::Debug for NvimStr<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.to_string_lossy(), f)
    }
}

impl fmt::Display for NvimStr<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.to_string_lossy(), f)
    }
}

impl hash::Hash for NvimStr<'_> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_bytes_with_nul().hash(state);
    }
}

impl PartialEq for NvimStr<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl Eq for NvimStr<'_> {}

impl PartialOrd for NvimStr<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NvimStr<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_bytes_with_nul().cmp(other.as_bytes_with_nul())
    }
}

impl<'a> From<&'a NvimString> for NvimStr<'a> {
    #[inline]
    fn from(string: &'a NvimString) -> Self {
        string.as_nvim_str()
    }
}
