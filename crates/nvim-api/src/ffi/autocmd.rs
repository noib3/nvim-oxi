use nvim_types::{Array, Error, Integer, NonOwning, Object, String};

use crate::opts::{
    KeyDict_clear_autocmds,
    KeyDict_create_augroup,
    KeyDict_create_autocmd,
    KeyDict_exec_autocmds,
    KeyDict_get_autocmds,
};

extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L570
    pub(crate) fn nvim_clear_autocmds(
        opts: *const KeyDict_clear_autocmds,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L649
    pub(crate) fn nvim_create_augroup(
        channel_id: u64,
        name: NonOwning<String>,
        opts: *const KeyDict_create_augroup,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L409
    pub(crate) fn nvim_create_autocmd(
        channel_id: u64,
        event: NonOwning<Object>,
        opts: *const KeyDict_create_autocmd,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L683
    pub(crate) fn nvim_del_augroup_by_id(id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L698
    pub(crate) fn nvim_del_augroup_by_name(
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L538
    pub(crate) fn nvim_del_autocmd(id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L721
    pub(crate) fn nvim_exec_autocmds(
        event: NonOwning<Object>,
        opts: *const KeyDict_exec_autocmds,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L90
    pub(crate) fn nvim_get_autocmds(
        opts: *const KeyDict_get_autocmds,
        err: *mut Error,
    ) -> Array;
}
