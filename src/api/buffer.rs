use std::fmt;

use crate::types::{BufHandle, NvimError, NvimString};

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1086
    fn nvim_buf_get_name(buf: BufHandle, err: *mut NvimError) -> NvimString;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Buffer(BufHandle);

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer({})", self.0)
    }
}

// TODO: find a way to check if a buffer handle has a buffer. Maybe
// `find_buffer_by_handle`?
//impl<T: Into<BufHandle>> TryFrom<T> for Buffer {
//    type Error = ();
//
//    fn try_from(handle: T) -> Result<Self, Self::Error> {
//        Ok(Buffer(handle.into()))
//    }
//}

impl Buffer {
    pub(crate) fn from(handle: BufHandle) -> Self {
        Buffer(handle)
    }

    /// Binding to `vim.api.nvim_buf_get_name`.
    ///
    /// Returns the full filepath of the buffer, replacing all invalid UTF-8
    /// byte sequences in the path with
    /// [`U+FFFD REPLACEMENT CHARACTER`](https://doc.rust-lang.org/nightly/core/char/constant.REPLACEMENT_CHARACTER.html).
    pub fn get_name(&self) -> String {
        unsafe { nvim_buf_get_name(self.0, &mut NvimError::default()) }
            .to_string_lossy()
            .into_owned()
    }

    pub const fn handle(&self) -> BufHandle {
        self.0
    }
}
