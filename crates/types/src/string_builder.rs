use core::ffi;
use core::fmt;
use core::num::NonZeroUsize;

use crate::String as NvimString;

/// A builder that can be used to efficiently build
/// [`nvim_oxi::String`](NvimString)s.
pub struct StringBuilder {
    /// The underlying string being constructed.
    inner: NvimString,
    /// Current capacity (i.e., allocated memory) of this builder in bytes.
    cap: usize,
}

impl StringBuilder {
    /// Create a new empty `StringBuilder`.
    #[inline]
    pub fn new() -> Self {
        Self { inner: NvimString::new(), cap: 0 }
    }

    /// Push new bytes to the builder.
    ///
    /// When only pushing bytes once, prefer [`NvimString::from_bytes`] as this
    /// method may allocate extra space in the buffer.
    #[inline]
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }

        // Reallocate if pushing the bytes overflows the allocated memory.
        self.reserve(bytes.len());
        debug_assert!(self.inner.len() < self.cap);

        // Pushing the `bytes` is safe now.
        let new_len = unsafe {
            libc::memcpy(
                self.inner.as_ptr().add(self.inner.len()) as *mut ffi::c_void,
                bytes.as_ptr() as *const ffi::c_void,
                bytes.len(),
            );

            let new_len = self.inner.len() + bytes.len();

            *self.inner.as_mut_ptr().add(new_len) = 0;

            new_len
        };

        unsafe { self.inner.set_len(new_len) };

        debug_assert!(self.inner.len() < self.cap);
    }

    /// Initialize [`StringBuilder`] with capacity.
    pub fn with_capacity(cap: usize) -> Self {
        // Neovim uses `xstrdup` to clone strings, which doesn't support null
        // pointers.
        //
        // For more infos, see https://github.com/noib3/nvim-oxi/pull/211#issuecomment-2566960494
        //
        // if cap == 0 {
        //     return Self::new();
        // }
        let real_cap = cap + 1;
        let ptr = unsafe { libc::malloc(real_cap) };
        if ptr.is_null() {
            unable_to_alloc_memory();
        }
        Self {
            inner: unsafe {
                NvimString::from_raw_parts(ptr as *mut ffi::c_char, 0)
            },
            cap: real_cap,
        }
    }

    /// Reserve space for `additional` more bytes.
    ///
    /// Does not allocate if enough space is available.
    pub fn reserve(&mut self, additional: usize) {
        let Some(min_capacity) = self.min_capacity_for_additional(additional)
        else {
            return;
        };
        let new_capacity =
            min_capacity.checked_next_power_of_two().unwrap_or(min_capacity);
        self.realloc(new_capacity);
    }

    /// Reserve space for exactly `additional` more bytes.
    ///
    /// Does not allocate if enough space is available.
    pub fn reserve_exact(&mut self, additional: usize) {
        if let Some(new_capacity) =
            self.min_capacity_for_additional(additional)
        {
            self.realloc(new_capacity);
        }
    }

    /// Reallocate the string with the given capacity.
    fn realloc(&mut self, new_capacity: NonZeroUsize) {
        let ptr = unsafe {
            libc::realloc(
                self.inner.as_mut_ptr() as *mut ffi::c_void,
                new_capacity.get(),
            )
        };
        // `realloc` may return null if it is unable to allocate the requested
        // memory.
        if ptr.is_null() {
            unable_to_alloc_memory();
        }
        self.inner = unsafe {
            NvimString::from_raw_parts(
                ptr as *mut ffi::c_char,
                self.inner.len(),
            )
        };
        self.cap = new_capacity.get();
    }

    /// Finish building the [`NvimString`]
    #[inline]
    pub fn finish(mut self) -> NvimString {
        let string = unsafe {
            NvimString::from_raw_parts(
                self.inner.as_mut_ptr(),
                self.inner.len(),
            )
        };

        if string.as_ptr().is_null() {
            debug_assert!(string.is_empty());
            debug_assert_eq!(self.cap, 0);

            // The pointer of `NvimString` should never be null, and it must be
            // terminated by a null byte.
            unsafe {
                let ptr = libc::malloc(1) as *mut ffi::c_char;
                if ptr.is_null() {
                    unable_to_alloc_memory();
                }
                ptr.write(0);
                NvimString::from_raw_parts(ptr, 0)
            }
        } else {
            debug_assert!(self.cap > self.inner.len());
            // Prevent self's destructor from being called.
            std::mem::forget(self);
            string
        }
    }

    /// Returns the remaining *usable* capacity, i.e. the remaining capacity
    /// minus the space reserved for the null terminator.
    #[inline(always)]
    fn remaining_capacity(&self) -> usize {
        if self.inner.as_ptr().is_null() {
            debug_assert_eq!(self.inner.len(), 0);
            return 0;
        }
        debug_assert!(
            self.cap > 0,
            "when data ptr is not null capacity must always be larger than 0"
        );
        debug_assert!(
            self.cap > self.inner.len(),
            "allocated capacity must always be bigger than length"
        );
        self.cap - self.inner.len() - 1
    }

    /// Returns the minimum capacity needed to allocate `additional` bytes, or
    /// `None` if the current capacity is already large enough.
    #[inline]
    fn min_capacity_for_additional(
        &self,
        additional: usize,
    ) -> Option<NonZeroUsize> {
        let remaining = self.remaining_capacity();
        if remaining >= additional {
            return None;
        }
        if self.inner.as_ptr().is_null() {
            debug_assert_eq!(self.cap, 0);
            NonZeroUsize::new(additional + 1)
        } else {
            NonZeroUsize::new(self.cap + additional - remaining)
        }
    }
}

impl Default for StringBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for StringBuilder {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_bytes(s.as_bytes());
        Ok(())
    }
}

impl Drop for StringBuilder {
    fn drop(&mut self) {
        if !self.inner.as_ptr().is_null() {
            unsafe { libc::free(self.inner.as_mut_ptr() as *mut ffi::c_void) }
        }
    }
}

#[cold]
#[inline(never)]
fn unable_to_alloc_memory() {
    panic!("unable to alloc memory with libc::realloc")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        let s = "foo bar";
        let bytes = b"baz foo bar";

        let mut sb = StringBuilder::new();
        sb.push_bytes(s.as_bytes());
        sb.push_bytes(bytes);

        assert_eq!(sb.inner.len(), s.len() + bytes.len());
        assert_eq!(sb.cap, 32); // Allocation size
        assert_eq!(unsafe { *sb.inner.as_ptr().add(sb.inner.len()) }, 0); // Null termination

        let nv_str = sb.finish();
        assert_eq!(nv_str.len(), s.len() + bytes.len());
    }

    #[test]
    fn with_capacity() {
        let s = StringBuilder::with_capacity(0);
        assert!(!s.inner.as_ptr().is_null());
        assert_eq!(s.cap, 1);
        assert_eq!(s.inner.len(), 0);
        s.finish();
        let s = StringBuilder::with_capacity(1);
        assert_eq!(s.cap, 2);
        assert_eq!(s.inner.len(), 0);
        s.finish();
        let s = StringBuilder::with_capacity(5);
        assert_eq!(s.cap, 6);
        assert_eq!(s.inner.len(), 0);
        s.finish();
    }

    #[test]
    fn reserve() {
        let mut sb = StringBuilder::new();
        assert_eq!(sb.cap, 0);
        sb.reserve(10);
        assert_eq!(sb.cap, 16);

        // Shouldn't change the pointer address as we have enough space.
        sb.reserve(10);
        assert_eq!(sb.cap, 16);
        let ptr = sb.inner.as_ptr();
        sb.push_bytes(b"Hello World!");
        // We already allocated the space needed the push above shouldn't
        // change the pointer.
        assert_eq!(sb.inner.as_ptr(), ptr);
        sb.push_bytes(&[b'a'; 55]);
        // We shouldn't check the pointer again as the block might be extended
        // instead of being moved to a different address.
        assert_eq!(unsafe { *sb.inner.as_ptr().add(sb.inner.len()) }, 0);
        assert_eq!(sb.cap, 128);
    }

    #[test]
    fn reserve_exact() {
        let mut sb = StringBuilder::new();
        sb.reserve_exact(10);
        assert_eq!(sb.cap, 11);
        let ptr = sb.inner.as_ptr();
        sb.push_bytes(b"hi");
        assert_eq!(sb.inner.len(), 2);

        // The space is already allocated, pushing bytes shouldn't change the
        // ptr address.
        assert_eq!(ptr, sb.inner.as_ptr());
        sb.push_bytes(b"Hello World!");
        assert_eq!(sb.cap, 16);
        assert_eq!(sb.inner.len(), 14);
        let ptr = sb.inner.as_ptr();
        sb.push_bytes(b"c");
        assert_eq!(sb.inner.as_ptr(), ptr);
    }
}
