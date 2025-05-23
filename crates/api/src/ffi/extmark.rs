use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/extmark.c#L919
    pub(crate) fn nvim_buf_add_highlight(
        buf: BufHandle,
        ns_id: Integer,
        hl_group: NvimStr,
        line: Integer,
        col_start: Integer,
        col_end: Integer,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/extmark.c#L979
    pub(crate) fn nvim_buf_clear_namespace(
        buf: BufHandle,
        ns_id: Integer,
        line_start: Integer,
        line_end: Integer,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/extmark.c#L863
    pub(crate) fn nvim_buf_del_extmark(
        buf: BufHandle,
        ns_id: Integer,
        id: Integer,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/extmark.c#L205
    pub(crate) fn nvim_buf_get_extmark_by_id(
        buf: BufHandle,
        ns_id: Integer,
        id: Integer,
        opts: *const GetExtmarkByIdOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/extmark.c#L288
    pub(crate) fn nvim_buf_get_extmarks(
        buf: BufHandle,
        ns_id: Integer,
        start: Object,
        end: Object,
        opts: *const GetExtmarksOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/extmark.c#L497
    pub(crate) fn nvim_buf_set_extmark(
        buf: BufHandle,
        ns_id: Integer,
        line: Integer,
        col: Integer,
        opts: *const SetExtmarkOpts,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/extmark.c#L57
    pub(crate) fn nvim_create_namespace(name: NvimStr) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/extmark.c#L75
    pub(crate) fn nvim_get_namespaces(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/extmark.c#L1052
    pub(crate) fn nvim_set_decoration_provider(
        ns_id: Integer,
        opts: *const DecorationProviderOpts,
        err: *mut Error,
    );
}
