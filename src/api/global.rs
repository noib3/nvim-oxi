use super::buffer::{BufHandle, Buffer};

extern "C" {
    fn nvim_get_current_buf() -> BufHandle;
}

/// Binding to `vim.api.nvim_get_current_buf`.
pub fn get_current_buf() -> Buffer {
    Buffer::from(unsafe { nvim_get_current_buf() })
}
