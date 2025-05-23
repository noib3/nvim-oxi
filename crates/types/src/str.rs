use core::marker::PhantomData;
use core::{ffi, slice};

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
    /// If you need a byte slice that includes the final null byte, use
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

    /// Returns a pointer to the [`NvimStr`]'s buffer.
    #[inline]
    pub const fn as_ptr(&self) -> *const ffi::c_char {
        self.data
    }

    /// Returns the length of the [`NvimStr`], *not* including the final null
    /// byte.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }
}
