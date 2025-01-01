//! This module contains the binding to Neovim's `String` type.

use alloc::borrow::Cow;
use alloc::string::String as StdString;
use core::str::{self, Utf8Error};
use core::{ffi, fmt, ptr, slice};
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use luajit as lua;

use crate::NonOwning;

/// Binding to the string type used by Neovim.
///
/// Unlike Rust's `String`, this type is not guaranteed to contain valid UTF-8
/// byte sequences, it can contain null bytes, and it is null-terminated.
//
// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L79-L82
#[derive(Eq, Ord, PartialOrd)]
#[repr(C)]
pub struct String {
    pub(super) data: *mut ffi::c_char,
    pub(super) len: usize,
}

/// A builder that can be used to efficiently build a [`nvim_oxi::String`](String).
pub struct StringBuilder {
    /// The underlying string being constructed.
    pub(super) inner: String,
    /// Current capacity (i.e., allocated memory) of this builder in bytes.
    pub(super) cap: usize,
}

impl Default for String {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for String {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_string_lossy().as_ref().fmt(f)
    }
}

impl core::fmt::Display for String {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_string_lossy().as_ref().fmt(f)
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

impl String {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        if self.data.is_null() {
            &[]
        } else {
            assert!(self.len() <= isize::MAX as usize);
            unsafe { slice::from_raw_parts(self.data as *const u8, self.len) }
        }
    }

    /// Returns a pointer to the `String`'s buffer.
    #[inline]
    pub fn as_ptr(&self) -> *const ffi::c_char {
        self.data as _
    }

    /// Creates a `String` from a byte slice by allocating `bytes.len() + 1`
    /// bytes of memory and copying the contents of `bytes` into it, followed
    /// by a null byte.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut s = StringBuilder::with_capacity(bytes.len());
        s.push_bytes(bytes);

        s.finish()
    }

    /// Returns `true` if the `String` has a length of zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of the `String`, *not* including the final null byte.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Creates a new, empty `String`.
    #[inline]
    pub fn new() -> Self {
        Self { data: ptr::null_mut(), len: 0 }
    }

    /// Makes a non-owning version of this `String`.
    #[inline]
    #[doc(hidden)]
    pub fn non_owning(&self) -> NonOwning<'_, String> {
        NonOwning::new(Self { ..*self })
    }

    /// Yields a string slice if the [`String`]'s contents are valid UTF-8.
    #[inline]
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts the `String` into Rust's `std::string::String`. If it already
    /// holds a valid UTF-8 byte sequence no allocation is made. If it doesn't
    /// the `String` is copied and all invalid sequences are replaced with `�`.
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        std::string::String::from_utf8_lossy(self.as_bytes())
    }
}

impl StringBuilder {
    /// Create a new empty `StringBuilder`.
    #[inline]
    pub fn new() -> Self {
        Self { inner: String::new(), cap: 0 }
    }

    /// Push new bytes to the builder.
    ///
    /// When only pushing bytes once, prefer [`String::from_bytes`] as this method may allocate
    /// extra space in the buffer.
    #[inline]
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        let slice_len = bytes.len();
        if slice_len == 0 {
            return;
        }

        // Reallocate if pushing the bytes overflows the allocated memory.
        self.reserve(bytes.len());
        debug_assert!(self.inner.len < self.cap);

        // Pushing the `bytes` is safe now.
        let new_len = unsafe {
            libc::memcpy(
                self.inner.data.add(self.inner.len) as *mut ffi::c_void,
                bytes.as_ptr() as *const ffi::c_void,
                slice_len,
            );

            let new_len = self.inner.len + slice_len;

            *self.inner.data.add(new_len) = 0;

            new_len
        };

        self.inner.len = new_len;
        debug_assert!(self.inner.len < self.cap);
    }

    /// Initialize [`StringBuilder`] with capacity.
    pub fn with_capacity(cap: usize) -> Self {
        if cap == 0 {
            return Self::new();
        }
        let real_cap = cap + 1;
        let ptr = unsafe { libc::malloc(real_cap) };
        if ptr.is_null() {
            unable_to_alloc_memory();
        }
        Self {
            inner: String { len: 0, data: ptr as *mut ffi::c_char },
            cap: real_cap,
        }
    }

    /// Reserve space for N more bytes.
    ///
    /// Does not allocate if enough space is available.
    pub fn reserve(&mut self, additional: usize) {
        let Some(min_capacity) = self.min_capacity_for_additional(additional)
        else {
            return;
        };
        let new_capacity =
            min_capacity.checked_next_power_of_two().unwrap_or(min_capacity);
        self.reserve_raw(new_capacity);
    }

    /// Reserve space for exactly N more bytes.
    ///
    /// Does not allocate if enough space is available.
    pub fn reserve_exact(&mut self, additional: usize) {
        if let Some(new_capacity) = self.min_capacity_for_additional(additional)
        {
            self.reserve_raw(new_capacity);
        }
    }

    /// Allocate new_capacity bytes.
    fn reserve_raw(&mut self, new_capacity: NonZeroUsize) {
        let ptr = unsafe {
            libc::realloc(
                self.inner.data as *mut ffi::c_void,
                new_capacity.get(),
            )
        };
        // realloc may return null if it is unable to allocate the requested memory
        if ptr.is_null() {
            unable_to_alloc_memory();
        }
        self.inner.data = ptr as *mut ffi::c_char;
        self.cap = new_capacity.get();
    }

    /// Finish building the [`String`]
    #[inline]
    fn finish(self) -> String {
        let s = String { data: self.inner.data, len: self.inner.len() };

        // extra sanity check
        if s.data.is_null() {
            debug_assert!(s.is_empty());
            debug_assert_eq!(self.cap, 0);
        } else {
            debug_assert!(self.cap > self.inner.len());
        }

        // Prevent self's destructor from being called.
        std::mem::forget(self);
        s
    }

    /// Returns the remaining *usable* capacity, i.e. the remaining capacity minus
    /// the space reserved for the null terminator.
    #[inline(always)]
    fn remaining_capacity(&self) -> usize {
        if self.inner.data.is_null() {
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

    #[inline]
    fn min_capacity_for_additional(
        &self,
        additional: usize,
    ) -> Option<NonZeroUsize> {
        let remaining = self.remaining_capacity();
        if remaining >= additional {
            return None;
        }
        if self.inner.data.is_null() {
            debug_assert_eq!(self.cap, 0);
            NonZeroUsize::new(additional + 1)
        } else {
            NonZeroUsize::new(self.cap + additional - remaining)
        }
    }
}

#[cold]
#[inline(never)]
fn unable_to_alloc_memory() {
    panic!("unable to alloc memory with libc::realloc")
}

impl Clone for String {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_bytes(self.as_bytes())
    }
}

impl Drop for String {
    fn drop(&mut self) {
        // There's no way to know if the pointer we get from Neovim
        // points to some `malloc`ed memory or to a static/borrowed string.
        //
        // TODO: we're leaking memory here if the pointer points to allocated
        // memory.
    }
}

impl Drop for StringBuilder {
    fn drop(&mut self) {
        if !self.inner.data.is_null() {
            unsafe { libc::free(self.inner.data as *mut ffi::c_void) }
        }
    }
}

impl From<&str> for String {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_bytes(s.as_bytes())
    }
}

impl From<StdString> for String {
    #[inline]
    fn from(s: StdString) -> Self {
        s.as_str().into()
    }
}

impl From<char> for String {
    #[inline]
    fn from(ch: char) -> Self {
        ch.to_string().into()
    }
}

impl From<Cow<'_, str>> for String {
    #[inline]
    fn from(moo: Cow<'_, str>) -> Self {
        moo.as_ref().into()
    }
}

impl From<&Path> for String {
    #[inline]
    fn from(path: &Path) -> Self {
        path.display().to_string().into()
    }
}

#[cfg(not(windows))]
impl From<String> for PathBuf {
    #[inline]
    fn from(nstr: String) -> Self {
        use std::os::unix::ffi::OsStrExt;
        std::ffi::OsStr::from_bytes(nstr.as_bytes()).to_owned().into()
    }
}

#[cfg(windows)]
impl From<String> for PathBuf {
    #[inline]
    fn from(nstr: String) -> Self {
        StdString::from_utf8_lossy(nstr.as_bytes()).into_owned().into()
    }
}

impl PartialEq<Self> for String {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<str> for String {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<String> for str {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        other == self
    }
}

impl PartialEq<&str> for String {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<String> for &str {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        other == self
    }
}

impl PartialEq<std::string::String> for String {
    #[inline]
    fn eq(&self, other: &std::string::String) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl core::hash::Hash for String {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
        self.len.hash(state);
    }
}

impl lua::Pushable for String {
    #[inline]
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
    ) -> Result<ffi::c_int, lua::Error> {
        lua::ffi::lua_pushlstring(lstate, self.as_ptr(), self.len());
        Ok(1)
    }
}

impl lua::Poppable for String {
    #[inline]
    unsafe fn pop(lstate: *mut lua::ffi::State) -> Result<Self, lua::Error> {
        use lua::ffi::*;

        if lua_gettop(lstate) < 0 {
            return Err(lua::Error::PopEmptyStack);
        }

        let ty = lua_type(lstate, -1);

        if ty != LUA_TSTRING && ty != LUA_TNUMBER {
            return Err(lua::Error::pop_wrong_type::<Self>(LUA_TSTRING, ty));
        }

        let mut len = 0;
        let ptr = lua_tolstring(lstate, -1, &mut len);

        // The pointer shouldn't be null if the type value at the top of thr
        // stack is a string or a number, but we'll check anyway.
        assert!(!ptr.is_null());

        let slice = std::slice::from_raw_parts(ptr as *const u8, len);
        let s = String::from_bytes(slice);

        lua_pop(lstate, 1);

        Ok(s)
    }
}

#[cfg(feature = "serde")]
mod serde {
    use std::fmt;

    use serde::de::{self, Deserialize, Deserializer, Visitor};

    impl<'de> Deserialize<'de> for super::String {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct StringVisitor;

            impl Visitor<'_> for StringVisitor {
                type Value = crate::String;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str("either a string of a byte vector")
                }

                fn visit_bytes<E>(self, b: &[u8]) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(crate::String::from_bytes(b))
                }

                fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(crate::String::from(s))
                }
            }

            deserializer.deserialize_str(StringVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_bytes() {
        let s = String::from("hello");
        assert_eq!(s.as_bytes(), b"hello");
    }

    #[test]
    fn empty_from_bytes() {
        let s = String::from_bytes(b"");
        assert_eq!(s.len(), 0);
        assert_eq!(s.data, ptr::null_mut());
    }

    #[test]
    fn from_bytes() {
        let s = String::from_bytes(b"Hello World!");
        assert_eq!(s.len(), 12);
    }

    #[test]
    fn with_capacity() {
        let s = StringBuilder::with_capacity(0);
        assert!(s.inner.data.is_null());
        assert_eq!(s.cap, 0);
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

        // shouldn't change the pointer address as we have enough space
        sb.reserve(10);
        assert_eq!(sb.cap, 16);
        let ptr = sb.inner.data;
        sb.push_bytes(b"Hello World!");
        // we already allocated the space needed the push above shouldn't change the pointer
        assert_eq!(sb.inner.data, ptr);
        sb.push_bytes(&[b'a'; 55]);
        // we shouldn't check the pointer again as the block might be extended instead of being
        // moved to a different address
        assert_eq!(unsafe { *sb.inner.data.add(sb.inner.len) }, 0);
        assert_eq!(sb.cap, 128);
    }

    #[test]
    fn reserve_exact() {
        let mut sb = StringBuilder::new();
        sb.reserve_exact(10);
        assert_eq!(sb.cap, 11);
        let ptr = sb.inner.data;
        sb.push_bytes(b"hi");
        assert_eq!(sb.inner.len(), 2);

        // the space is already allocated, pushing bytes shouldn't change the ptr address
        assert_eq!(ptr, sb.inner.data);
        sb.push_bytes(b"Hello World!");
        assert_eq!(sb.cap, 16);
        assert_eq!(sb.inner.len(), 14);
        let ptr = sb.inner.data;
        sb.push_bytes(b"c");
        assert_eq!(sb.inner.data, ptr);
    }

    #[test]
    fn partial_eq() {
        let lhs = String::from("foo bar baz");
        let rhs = String::from("foo bar baz");
        assert_eq!(lhs, rhs);

        let lhs = String::from("foo bar baz");
        let rhs = StdString::from("bar foo baz");
        assert_ne!(lhs, rhs);

        let lhs = String::from("€");
        let rhs = "€";
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn clone() {
        let lhs = String::from("abc");
        let rhs = lhs.clone();

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn from_string() {
        let foo = StdString::from("foo bar baz");

        let lhs = String::from(foo.as_str());
        let rhs = String::from(foo.as_str());

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn builder() {
        let s = "foo bar";
        let bytes = b"baz foo bar";

        let mut sb = StringBuilder::new();
        sb.push_bytes(s.as_bytes());
        sb.push_bytes(bytes);

        assert_eq!(sb.inner.len, s.len() + bytes.len());
        assert_eq!(sb.cap, 32); // Allocation size
        assert_eq!(unsafe { *sb.inner.data.add(sb.inner.len) }, 0); // Null termination

        let nv_str = sb.finish();
        assert_eq!(nv_str.len(), s.len() + bytes.len());
    }
}
