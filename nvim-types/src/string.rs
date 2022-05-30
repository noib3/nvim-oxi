use std::{
    borrow::Cow,
    ffi::{CStr, CString, OsStr},
    fmt,
    mem,
    os::unix::ffi::OsStrExt,
    path::PathBuf,
    ptr,
    string::String as StdString,
};

use libc::{c_char, size_t};

use super::object::{Object, ObjectType};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L77
#[derive(Eq)]
#[repr(C)]
pub struct String {
    pub data: *mut c_char,
    pub size: size_t,
}

impl String {
    /// TODO: docs
    #[inline]
    pub fn from_bytes<Raw: Into<Vec<u8>>>(raw: Raw) -> Self {
        let mut bytes = raw.into();
        let string = Self {
            data: bytes.as_mut_ptr().cast::<c_char>(),
            size: bytes.len(),
        };
        mem::forget(bytes);
        string
    }

    #[inline]
    pub fn from_c_string(c_string: CString) -> Self {
        let mut bytes = c_string.into_bytes();
        let mut uninit = std::mem::MaybeUninit::<String>::uninit();

        let ptr = uninit.as_mut_ptr();

        unsafe {
            ptr::addr_of_mut!((*ptr).size).write(bytes.len());
        }

        unsafe {
            ptr::addr_of_mut!((*ptr).data).write(bytes.as_mut_ptr().cast());
        }

        mem::forget(bytes);

        unsafe { uninit.assume_init() }
    }

    #[allow(dead_code)]
    #[inline]
    pub unsafe fn new_unchecked<Raw: Into<Vec<u8>>>(raw: Raw) -> Self {
        Self::from_c_string(CString::from_vec_unchecked(raw.into()))
    }

    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.data) }
    }

    /// TODO: docs
    #[allow(dead_code)]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// TODO: docs
    #[allow(dead_code)]
    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    /// TODO: docs
    #[allow(dead_code)]
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }

    /// TODO: docs
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        StdString::from_utf8_lossy(self.as_bytes())
    }

    /// Converts an `NvimString` into a byte vector, consuming the string.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        let bytes = unsafe {
            Vec::from_raw_parts(self.data.cast::<u8>(), self.size, self.size)
        };
        mem::forget(self);
        bytes
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
        let size = self.len();
        let mut data = Vec::with_capacity(size);

        unsafe { ptr::copy(self.data, data.as_mut_ptr(), size) };
        unsafe { data.set_len(size) };

        let new = Self { data: data.as_mut_ptr(), size };
        mem::forget(data);

        new
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

impl<'a> From<&'a str> for String {
    #[inline]
    fn from(str: &'a str) -> Self {
        Self::from_bytes(str)
    }
}

impl From<StdString> for String {
    #[inline]
    fn from(string: StdString) -> Self {
        Self::from_bytes(string)
    }
}

impl TryFrom<Object> for String {
    type Error = ();

    #[inline]
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        if !matches!(obj.r#type, ObjectType::kObjectTypeString) {
            return Err(());
        }

        let string = Self {
            data: unsafe { obj.data.string.data },
            size: unsafe { obj.data.string.size },
        };

        mem::forget(obj);

        Ok(string)
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
