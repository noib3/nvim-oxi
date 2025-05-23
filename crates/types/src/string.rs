//! This module contains the binding to Neovim's `String` type.

use alloc::borrow::Cow;
use alloc::string::String as StdString;
use core::str::{self, Utf8Error};
use core::{ffi, fmt};
use std::path::{Path, PathBuf};

use luajit as lua;

use crate::{conversion, NvimStr, Object, ObjectKind, StringBuilder};

/// Binding to the string type used by Neovim.
///
/// Unlike Rust's `String`, this type is not guaranteed to contain valid UTF-8
/// byte sequences, it can contain null bytes, and it is null-terminated.
//
// https://github.com/neovim/neovim/blob/v0.11.0/src/nvim/api/private/defs.h#L80-L83
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct String {
    inner: NvimStr<'static>,
}

impl String {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    /// Returns a pointer to the `String`'s buffer.
    #[inline]
    pub fn as_ptr(&self) -> *const ffi::c_char {
        self.inner.as_ptr()
    }

    /// Returns an [`NvimStr`] view of this `String`.
    #[inline]
    pub fn as_nvim_str(&self) -> NvimStr<'_> {
        self.inner.reborrow()
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

    /// Creates a `String` from a pointer to the underlying data and a length.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid for `len + 1`
    /// elements and that the last element is a null byte.
    #[inline]
    pub unsafe fn from_raw_parts(
        data: *const ffi::c_char,
        len: usize,
    ) -> Self {
        Self { inner: NvimStr::from_raw_parts(data, len) }
    }

    /// Returns `true` if the `String` has a length of zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of the `String`, *not* including the final null
    /// byte.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Creates a new, empty `String`.
    #[inline]
    pub fn new() -> Self {
        Self::from_bytes(&[])
    }

    /// Forces the length of the string to be `new_len`.
    ///
    /// # Safety
    ///
    /// Same as [`NvimStr::set_len`].
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len);
    }

    /// Yields a string slice if the [`String`]'s contents are valid UTF-8.
    #[inline]
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        self.inner.to_str()
    }

    /// Converts the `String` into Rust's `std::string::String`. If it already
    /// holds a valid UTF-8 byte sequence no allocation is made. If it doesn't
    /// the `String` is copied and all invalid sequences are replaced with `�`.
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.inner.to_string_lossy()
    }
}

impl Default for String {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl fmt::Display for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
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

impl TryFrom<Object> for String {
    type Error = conversion::Error;

    #[inline]
    fn try_from(obj: crate::Object) -> Result<Self, Self::Error> {
        match obj.kind() {
            ObjectKind::String => Ok(unsafe { obj.into_string_unchecked() }),
            other => Err(conversion::Error::FromWrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
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
        assert!(!s.as_ptr().is_null());
    }

    #[test]
    fn from_bytes() {
        let s = String::from_bytes(b"Hello World!");
        assert_eq!(s.len(), 12);
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
}
