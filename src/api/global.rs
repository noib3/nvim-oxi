use super::buffer::Buffer;
use crate::types::{BufHandle, NvimError};
use crate::Result;

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1057
    fn nvim_create_buf(
        listed: bool,
        scratch: bool,
        err: *mut NvimError,
    ) -> BufHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L963
    fn nvim_get_current_buf() -> BufHandle;
}

/// Binding to `vim.api.nvim_create_buf`.
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = NvimError::default();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    err.into_err_or(|| Buffer::from(handle))
}

/// Binding to `vim.api.nvim_get_current_buf`.
pub fn get_current_buf() -> Buffer {
    Buffer::from(unsafe { nvim_get_current_buf() })
}
