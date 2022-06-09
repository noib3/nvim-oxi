use std::borrow::Cow;
use std::ffi::OsStr;
#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_family = "windows")]
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::string::{self, String as StdString};
use std::{fmt, slice, str};

use libc::{c_char, size_t};

/// Neovim's `String`s:
///   - are null-terminated;
///   - *can* contain null bytes (confirmed by bfredl on matrix);
///   - they store a `size` field just like Rust strings, which *doesn't*
///     include the last `\0`;
///   - unlike Rust strings, they are *not* guaranteed to always contain valid
///     UTF-8 byte sequences;
///
/// See https://github.com/neovim/neovim/blob/master/src/nvim/api/private/helpers.c#L478
/// for how a C string gets converted into a Neovim string.
///
/// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L77
#[derive(Eq)]
#[repr(C)]
pub struct String {
    pub data: *mut c_char,
    pub size: size_t,
}

impl String {
    /// TODO: docs
    #[inline]
    pub fn from_bytes(mut vec: Vec<u8>) -> Self {
        vec.reserve_exact(1);
        vec.push(0);

        let size = vec.len() - 1;
        let data = vec.leak().as_mut_ptr() as *mut c_char;

        Self { data, size }
    }

    /// TODO: docs
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// TODO: docs
    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    /// TODO: docs
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.data as *const u8, self.size) }
    }

    /// TODO: docs
    #[inline]
    pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// TODO: docs
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        StdString::from_utf8_lossy(self.as_bytes())
    }

    /// Converts an `NvimString` into a byte vector, consuming the string.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(self.data.cast::<u8>(), self.size, self.size)
        }
    }

    /// TODO: docs
    #[inline]
    pub fn into_string(self) -> Result<StdString, string::FromUtf8Error> {
        StdString::from_utf8(self.into_bytes())
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NvimString")
            .field("data", &self.to_string_lossy())
            .field("size", &self.size)
            .finish()
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        Self::from_bytes(self.as_bytes().to_owned())
    }
}

impl From<StdString> for String {
    #[inline]
    fn from(string: StdString) -> Self {
        Self::from_bytes(string.into_bytes())
    }
}

impl<'a> From<&'a str> for String {
    #[inline]
    fn from(str: &'a str) -> Self {
        Self::from_bytes(str.as_bytes().to_owned())
    }
}

impl From<char> for String {
    #[inline]
    fn from(ch: char) -> Self {
        ch.to_string().into()
    }
}

#[cfg(not(windows))]
impl From<String> for PathBuf {
    #[inline]
    fn from(nstr: String) -> Self {
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

impl<'a> PartialEq<&'a str> for String {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
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
