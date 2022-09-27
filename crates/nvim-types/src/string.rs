use std::borrow::Cow;
use std::ffi::{c_char, c_int, OsStr};
use std::mem::ManuallyDrop;
use std::path::PathBuf;
use std::string::{self, String as StdString};
use std::{fmt, slice, str};

use lua::{ffi::*, LuaPoppable, LuaPushable};
use luajit_bindings as lua;

use crate::NonOwning;

// Neovim's `String`s:
//   - are null-terminated;
//   - *can* contain null bytes (confirmed by bfredl on matrix);
//   - they store a `size` field just like Rust strings, which *doesn't*
//     include the last `\0`;
//   - unlike Rust strings, they are *not* guaranteed to always contain valid
//     UTF-8 byte sequences;
//
// See https://github.com/neovim/neovim/blob/master/src/nvim/api/private/helpers.c#L478
// for how a C string gets converted into a Neovim string.
//
// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L77
//
/// A particular string type used internally by Neovim.
#[derive(Eq, Ord, PartialOrd)]
#[repr(C)]
pub struct String {
    pub(crate) data: *mut c_char,
    pub(crate) size: usize,
}

impl String {
    #[inline]
    /// Creates a new empty string.
    pub fn new() -> Self {
        Self { data: std::ptr::null_mut(), size: 0 }
    }

    /// Creates a [`String`] from a byte vector.
    #[inline]
    pub fn from_bytes(mut vec: Vec<u8>) -> Self {
        vec.reserve_exact(1);
        vec.push(0);

        let size = vec.len() - 1;
        let data = vec.leak().as_mut_ptr() as *mut c_char;

        Self { data, size }
    }

    /// Returns `true` if the `String` has a length of zero, and `false`
    /// otherwise.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the byte length of the `String`, *not* including the final null
    /// byte.
    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Returns a pointer pointing to the byte of the string.
    #[inline]
    pub const fn as_ptr(&self) -> *const c_char {
        self.data as *const c_char
    }

    /// Returns a byte slice of this `String`'s contents.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.data as *const u8, self.size) }
        }
    }

    /// Returns a string slice of this `String`'s contents. Fails if it doesn't
    /// contain a valid UTF-8 byte sequence.
    #[inline]
    pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts the `String` into Rust's `std::string::String`. If it already
    /// holds a valid UTF-8 byte sequence no allocation is made. If it doesn't
    /// the `String` is copied and all invalid sequences are replaced with `�`.
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        StdString::from_utf8_lossy(self.as_bytes())
    }

    /// Converts the `String` into a byte vector, consuming it.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        if self.data.is_null() {
            Vec::new()
        } else {
            unsafe {
                let mdrop = ManuallyDrop::new(self);
                Vec::from_raw_parts(
                    mdrop.data.cast::<u8>(),
                    mdrop.size,
                    mdrop.size,
                )
            }
        }
    }

    /// Converts the `String` into Rust's `std::string::String`, consuming it.
    /// Fails if it doesn't contain a valid UTF-8 byte sequence.
    #[inline]
    pub fn into_string(self) -> Result<StdString, string::FromUtf8Error> {
        StdString::from_utf8(self.into_bytes())
    }

    /// Makes a non-owning version of this `String`.
    #[inline]
    #[doc(hidden)]
    pub fn non_owning(&self) -> NonOwning<'_, String> {
        NonOwning::new(Self { ..*self })
    }
}

impl Default for String {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string_lossy())
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        Self::from_bytes(self.as_bytes().to_owned())
    }
}

impl Drop for String {
    fn drop(&mut self) {
        // One extra for null terminator.
        let _ = unsafe {
            Vec::from_raw_parts(self.data, self.size + 1, self.size + 1)
        };
    }
}

impl From<StdString> for String {
    #[inline]
    fn from(string: StdString) -> Self {
        Self::from_bytes(string.into_bytes())
    }
}

impl From<&str> for String {
    #[inline]
    fn from(str: &str) -> Self {
        Self::from_bytes(str.as_bytes().to_owned())
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
        moo.into_owned().into()
    }
}

impl From<Vec<u8>> for String {
    #[inline]
    fn from(vec: Vec<u8>) -> Self {
        Self::from_bytes(vec)
    }
}

impl From<PathBuf> for String {
    #[inline]
    fn from(path: PathBuf) -> Self {
        path.display().to_string().into()
    }
}

#[cfg(not(windows))]
impl From<String> for PathBuf {
    #[inline]
    fn from(nstr: String) -> Self {
        use std::os::unix::ffi::OsStrExt;
        OsStr::from_bytes(nstr.as_bytes()).to_owned().into()
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

impl PartialEq<&str> for String {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<StdString> for String {
    #[inline]
    fn eq(&self, other: &StdString) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl TryFrom<String> for StdString {
    type Error = std::string::FromUtf8Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        StdString::from_utf8(s.into_bytes())
    }
}

impl LuaPushable for String {
    unsafe fn push(self, lstate: *mut lua_State) -> Result<c_int, lua::Error> {
        lua::ffi::lua_pushlstring(lstate, self.as_ptr(), self.len());
        Ok(1)
    }
}

impl LuaPoppable for String {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, lua::Error> {
        if lua::ffi::lua_type(lstate, -1) != lua::ffi::LUA_TSTRING
            || lua::utils::is_table_array(lstate, -1)
        {
            return Err(lua::Error::pop_wrong_type_at_idx::<Self>(lstate, -1));
        }

        let vec = LuaPoppable::pop(lstate)?;
        Ok(Self::from_bytes(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq() {
        let lhs = String::from("foo bar baz");
        let rhs = String::from("foo bar baz");
        assert_eq!(lhs, rhs);

        let lhs = String::from("foo bar baz");
        let rhs = String::from("bar foo baz");
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

        let lhs = String::from(foo.as_ref());
        let rhs = String::from(foo);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn to_bytes() {
        let s = String::from("hello");
        let bytes = s.into_bytes();
        assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
    }
}
