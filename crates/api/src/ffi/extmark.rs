use types::{
    Array,
    BufHandle,
    Dictionary,
    Error,
    Integer,
    NonOwning,
    Object,
    String,
};

use crate::opts::*;

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
    pub(crate) fn nvim_get_namespaces() -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/extmark.c#L1074
    pub(crate) fn nvim_set_decoration_provider(
        ns_id: Integer,
        opts: *const DecorationProviderOpts,
        err: *mut Error,
    );
}
