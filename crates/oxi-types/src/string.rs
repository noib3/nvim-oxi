//! This module contains the binding to Neovim's `String` type.

use core::{ffi, slice};
use std::borrow::Cow;

use oxi_luajit as lua;

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
    pub(super) size: usize,
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
        f.write_str(self.to_string_lossy().as_ref())
    }
}

impl String {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        if self.data.is_null() {
            &[]
        } else {
            assert!(self.len() <= isize::MAX as usize);
            unsafe { slice::from_raw_parts(self.data as *const u8, self.size) }
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
        let data =
            unsafe { libc::malloc(bytes.len() + 1) as *mut ffi::c_char };

        unsafe {
            libc::memcpy(
                data as *mut _,
                bytes.as_ptr() as *const _,
                bytes.len(),
            )
        };

        unsafe { *data.add(bytes.len()) = 0 };

        Self { data: data as *mut _, size: bytes.len() }
    }

    /// Returns `true` if the `String` has a length of zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of the `String`, *not* including the final null byte.
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Creates a new, empty `String`.
    #[inline]
    pub fn new() -> Self {
        Self { data: core::ptr::null_mut(), size: 0 }
    }

    /// Makes a non-owning version of this `String`.
    #[inline]
    #[doc(hidden)]
    pub fn non_owning(&self) -> NonOwning<'_, String> {
        NonOwning::new(Self { ..*self })
    }

    /// Converts the `String` into Rust's `std::string::String`. If it already
    /// holds a valid UTF-8 byte sequence no allocation is made. If it doesn't
    /// the `String` is copied and all invalid sequences are replaced with `�`.
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        std::string::String::from_utf8_lossy(self.as_bytes())
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

impl From<char> for String {
    #[inline]
    fn from(ch: char) -> Self {
        ch.to_string().as_str().into()
    }
}

impl From<&std::path::Path> for String {
    #[inline]
    fn from(path: &std::path::Path) -> Self {
        path.display().to_string().as_str().into()
    }
}

#[cfg(not(windows))]
impl From<String> for std::path::PathBuf {
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
        std::string::String::from_utf8_lossy(nstr.as_bytes())
            .into_owned()
            .into()
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
        self.size.hash(state);
    }
}

impl lua::Pushable for String {
    #[inline]
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<ffi::c_int, lua::Error> {
        lua::ffi::lua_pushlstring(lstate, self.as_ptr(), self.len());
        Ok(1)
    }
}

impl lua::Poppable for String {
    #[inline]
    unsafe fn pop(
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<Self, lua::Error> {
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

            impl<'de> Visitor<'de> for StringVisitor {
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
        let rhs = std::string::String::from("bar foo baz");
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
        let foo = std::string::String::from("foo bar baz");

        let lhs = String::from(foo.as_str());
        let rhs = String::from(foo.as_str());

        assert_eq!(lhs, rhs);
    }
}
