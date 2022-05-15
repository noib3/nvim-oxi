use std::fmt;

use crate::types::{BufHandle, Error, String as NvString};

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1086
    fn nvim_buf_get_name(buf: BufHandle, err: *mut Error) -> NvString;
}

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
    /// Binding to `vim.api.nvim_buf_get_name`.
    pub fn get_name(&self) -> String {
        let mut err = Error::default();
        let name = unsafe { nvim_buf_get_name(self.0, &mut err) };
        // TODO: check value of `err`.
        name.to_string_lossy().into_owned()
    }
}
