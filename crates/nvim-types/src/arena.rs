// https://github.com/neovim/neovim/blob/v0.8.3/src/nvim/memory.h#L50
#[repr(C)]
pub struct Arena {
    cur_blk: *mut std::ffi::c_char,
    pos: usize,
    size: usize,
}

impl Arena {
    pub fn empty() -> Self {
        Self { cur_blk: std::ptr::null_mut(), pos: 0, size: 0 }
    }
}
