use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::fmt;
use std::mem;
use std::ptr;

use libc::{c_char, size_t};

use super::object::{Object, ObjectType};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L77
#[derive(Eq)]
#[repr(C)]
pub struct NvimString {
    data: *mut c_char,
    size: size_t,
}

impl NvimString {
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
        let mut uninit = std::mem::MaybeUninit::<NvimString>::uninit();

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

    #[allow(dead_code)]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    #[allow(dead_code)]
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }

    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        std::string::String::from_utf8_lossy(self.as_bytes())
    }
}

impl fmt::Debug for NvimString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NvimString")
            .field("data", &self.to_string_lossy())
            .field("size", &self.size)
            .finish()
    }
}

impl Clone for NvimString {
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

impl PartialEq<Self> for NvimString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<str> for NvimString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<NvimString> for str {
    #[inline]
    fn eq(&self, other: &NvimString) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<'a> PartialEq<&'a str> for NvimString {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<String> for NvimString {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<'a> From<&'a str> for NvimString {
    #[inline]
    fn from(str: &'a str) -> Self {
        Self::from_bytes(str)
    }
}

impl From<std::string::String> for NvimString {
    #[inline]
    fn from(string: std::string::String) -> Self {
        Self::from_bytes(string)
    }
}

impl TryFrom<Object> for NvimString {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq() {
        let lhs = NvimString::from("foo bar baz");
        let rhs = NvimString::from("foo bar baz");
        assert_eq!(lhs, rhs);

        let lhs = NvimString::from("foo bar baz");
        let rhs = NvimString::from("bar foo baz");
        assert_ne!(lhs, rhs);

        let lhs = NvimString::from("€");
        let rhs = "€";
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn clone() {
        let lhs = NvimString::from("abc");
        let rhs = lhs.clone();

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn from_string() {
        let foo = std::string::String::from("foo bar baz");

        let lhs = NvimString::from(foo.as_ref());
        let rhs = NvimString::from(foo);

        assert_eq!(lhs, rhs);
    }
}
