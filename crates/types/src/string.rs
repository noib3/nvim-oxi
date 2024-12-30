//! This module contains the binding to Neovim's `String` type.

use alloc::borrow::Cow;
use alloc::string::String as StdString;
use core::str::{self, Utf8Error};
use core::{ffi, fmt, ptr, slice};
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
        let mut s = StringBuilder::new();
        s.reserve_exact(bytes.len());
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
        let required_cap = self.inner.len + slice_len + 1;

        // Reallocate if pushing the bytes overflows the allocated memory.
        self.reserve(required_cap);
        debug_assert!(self.inner.len < self.cap);

        // Pushing the `bytes` is safe now.
        let new_len = unsafe {
            libc::memcpy(
                self.inner.data.add(self.inner.len) as *mut _,
                bytes.as_ptr() as *const _,
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
        let mut s = Self::new();
        s.reserve(cap);
        s
    }

    /// Reserve space for N more bytes.
    ///
    /// Does not allocate if enough space is available.
    pub fn reserve(&mut self, cap: usize) {
        // + 1 for the null byte
        if self.cap - self.inner.len() < cap + 1 {
            let n = (cap - 1).ilog2() + 1;
            let new_cap = 2_usize.pow(n).max(4);
            self.reserve_exact(new_cap);
        }
    }

    /// Reserve space for exactly N more bytes.
    ///
    /// Does not allocate if enough space is available.
    pub fn reserve_exact(&mut self, cap: usize) {
        // + 1 for the null byte
        if self.cap - self.inner.len() < cap + 1 {
            // SAFETY: realloc is legal with null pointers, no need for an extra check.
            self.inner.data = unsafe {
                libc::realloc(
                    self.inner.data as *mut _,
                    self.inner.len() + 1 + cap,
                ) as *mut ffi::c_char
            };
            self.cap = self.inner.len() + cap;
        }
    }

    /// Build the `String`.
    #[inline]
    pub fn finish(self) -> String {
        let s = String { data: self.inner.data, len: self.inner.len };

        // Prevent self's destructor from being called.
        std::mem::forget(self);

        s
    }
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
            unsafe { libc::free(self.inner.data as *mut _) }
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
        assert_eq!(s.as_bytes(), &[b'h', b'e', b'l', b'l', b'o'][..]);
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
        let str = "foo bar";
        let bytes = b"baz foo bar";

        let mut s = StringBuilder::new();
        s.push_bytes(str.as_bytes());
        s.push_bytes(bytes);

        assert_eq!(s.inner.len, str.len() + bytes.len());
        assert_eq!(s.cap, 32); // Allocation size
        assert_eq!(unsafe { *s.inner.data.add(s.inner.len) }, 0); // Null termination

        let s = s.finish();
        assert_eq!(s.len(), str.len() + bytes.len());
    }
}
