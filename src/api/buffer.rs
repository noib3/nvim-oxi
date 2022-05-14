// use std::ffi::CString;
use std::fmt;

extern "C" {
    // fn nvim_buf_get_name(handle: BufHandle) -> CString;
}

pub(crate) type BufHandle = std::os::raw::c_int;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Buffer(BufHandle);

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer({})", self.0)
    }
}

impl<T: Into<BufHandle>> From<T> for Buffer {
    fn from(handle: T) -> Self {
        Buffer(handle.into())
    }
}

impl Buffer {
    // /// Binding to `vim.api.nvim_buf_get_name`.
    // pub fn get_name(&self) -> String {
    //     unsafe { nvim_buf_get_name(self.0) }.into_string().unwrap()
    // }
}
