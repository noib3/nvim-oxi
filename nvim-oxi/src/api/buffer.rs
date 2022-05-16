use std::fmt;

use nvim_types::{BufHandle, Error as NvimError, String as NvimString};

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/private/helpers.c#L411
    fn find_buffer_by_handle(
        buf: BufHandle,
        err: *mut NvimError,
    ) -> *const buf_T;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1086
    fn nvim_buf_get_name(buf: BufHandle, err: *mut NvimError) -> NvimString;
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct buf_T {
    _inner: [u8; 0],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Buffer(BufHandle);

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer({})", self.0)
    }
}

// I'd really like to write this as
// `impl<T: Into<BufHandle>> TryFrom<T> for Buffer {..}`
// but can't because of https://github.com/rust-lang/rust/issues/50133, aaargh.
impl TryFrom<BufHandle> for Buffer {
    type Error = crate::error::Error;

    fn try_from(handle: BufHandle) -> Result<Self, Self::Error> {
        let mut err = NvimError::default();
        let _ = unsafe { find_buffer_by_handle(handle, &mut err) };
        err.into_err_or_else(|| Buffer(handle))
    }
}

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
