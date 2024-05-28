use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L570
    pub(crate) fn nvim_clear_autocmds(
        opts: *const ClearAutocmdsOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L649
    pub(crate) fn nvim_create_augroup(
        channel_id: u64,
        name: NonOwning<String>,
        opts: *const CreateAugroupOpts,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L409
    pub(crate) fn nvim_create_autocmd(
        channel_id: u64,
        event: NonOwning<Object>,
        opts: *const CreateAutocmdOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
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
        opts: *const ExecAutocmdsOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/autocmd.c#L90
    pub(crate) fn nvim_get_autocmds(
        opts: *const GetAutocmdsOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;
}
