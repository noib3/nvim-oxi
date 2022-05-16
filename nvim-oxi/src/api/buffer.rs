use std::fmt;

use nvim_types::{BufHandle, Error as NvimError, String as NvimString};

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
    /// Creates a `Buffer` from a `BufHandle`. It's only available inside the
    /// crate to disallow creating `Buffer`s explicitely. This way a lot of the
    /// following methods don't have to return a `Result`, since most of the
    /// `nvim_buf_*` Neovim functions only fail when passing invalid
    /// `BufHandle`s.
    pub(crate) fn from(handle: BufHandle) -> Self {
        Buffer(handle)
    }

    /// Binding to `vim.api.nvim_buf_get_name`.
    ///
    /// Returns the full filepath of the buffer, replacing all invalid UTF-8
    /// byte sequences in the path with `U+FFFD REPLACEMENT CHARACTER` (�).
    pub fn get_name(&self) -> String {
        unsafe { nvim_buf_get_name(self.0, &mut NvimError::default()) }
            .to_string_lossy()
            .into_owned()
    }
}
