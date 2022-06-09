#![allow(dead_code)]

use libc::uint64_t;
use nvim_types::{
    array::Array, dictionary::Dictionary, error::Error, object::Object,
    string::String, BufHandle, Integer, TabHandle, WinHandle,
};

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/autocmd.c#L597
    pub(crate) fn nvim_clear_autocmds(
        opts: *const Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/autocmd.c#L701
    pub(crate) fn nvim_create_augroup(
        channel_id: u64,
        name: String,
        opts: *const Dictionary,
        err: *mut Error,
    ) -> u32;
}
