use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
unsafe extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/autocmd.c#L555
    pub(crate) fn nvim_clear_autocmds(
        opts: *const ClearAutocmdsOpts,
        arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/autocmd.c#L629
    pub(crate) fn nvim_create_augroup(
        channel_id: u64,
        name: NvimStr,
        opts: *const CreateAugroupOpts,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/autocmd.c#L403
    pub(crate) fn nvim_create_autocmd(
        channel_id: u64,
        event: NonOwning<Object>,
        opts: *const CreateAutocmdOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/autocmd.c#L663
    pub(crate) fn nvim_del_augroup_by_id(id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/autocmd.c#L678
    pub(crate) fn nvim_del_augroup_by_name(name: NvimStr, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/autocmd.c#L523
    pub(crate) fn nvim_del_autocmd(id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/autocmd.c#L701
    pub(crate) fn nvim_exec_autocmds(
        event: NonOwning<Object>,
        opts: *const ExecAutocmdsOpts,
        arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/autocmd.c#L92
    pub(crate) fn nvim_get_autocmds(
        opts: *const GetAutocmdsOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;
}
