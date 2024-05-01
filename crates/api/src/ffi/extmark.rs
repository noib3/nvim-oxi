use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L950
    pub(crate) fn nvim_buf_add_highlight(
        buf: BufHandle,
        ns_id: Integer,
        hl_group: NonOwning<String>,
        line: Integer,
        col_start: Integer,
        col_end: Integer,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L1012
    pub(crate) fn nvim_buf_clear_namespace(
        buf: BufHandle,
        ns_id: Integer,
        line_start: Integer,
        line_end: Integer,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L894
    pub(crate) fn nvim_buf_del_extmark(
        buf: BufHandle,
        ns_id: Integer,
        id: Integer,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L252
    pub(crate) fn nvim_buf_get_extmark_by_id(
        buf: BufHandle,
        ns_id: Integer,
        id: Integer,
        #[cfg(not(feature = "neovim-nightly"))] opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const GetExtmarkByIdOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L341
    pub(crate) fn nvim_buf_get_extmarks(
        buf: BufHandle,
        ns_id: Integer,
        start: Object,
        end: Object,
        #[cfg(not(feature = "neovim-nightly"))] opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const GetExtmarksOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L565
    pub(crate) fn nvim_buf_set_extmark(
        buf: BufHandle,
        ns_id: Integer,
        line: Integer,
        col: Integer,
        opts: *const SetExtmarkOpts,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L55
    pub(crate) fn nvim_create_namespace(name: NonOwning<String>) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L73
    pub(crate) fn nvim_get_namespaces(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L1074
    pub(crate) fn nvim_set_decoration_provider(
        ns_id: Integer,
        opts: *const DecorationProviderOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L1223
    #[cfg(feature = "neovim-nightly")]
    pub(crate) fn nvim_win_add_ns(
        window: WinHandle,
        ns_id: Integer,
        err: *mut Error,
    ) -> Boolean;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L1246
    #[cfg(feature = "neovim-nightly")]
    pub(crate) fn nvim_win_get_ns(
        window: WinHandle,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L1268
    #[cfg(feature = "neovim-nightly")]
    pub(crate) fn nvim_win_remove_ns(
        window: WinHandle,
        ns_id: Integer,
        err: *mut Error,
    ) -> Boolean;
}
