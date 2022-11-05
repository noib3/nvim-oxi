use std::{
    ffi::{c_char, OsStr},
    os::unix::prelude::OsStrExt,
    path::PathBuf,
    slice,
};

#[repr(C)]
pub struct Str {
    pub(crate) data: *mut c_char,
    pub(crate) size: usize,
}

impl Str {
    fn as_bytes(&self) -> &[u8] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.data as *const u8, self.size) }
        }
    }
}

impl From<Str> for PathBuf {
    fn from(s: Str) -> Self {
        OsStr::from_bytes(s.as_bytes()).to_owned().into()
    }
}
