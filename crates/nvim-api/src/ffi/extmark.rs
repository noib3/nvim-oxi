use nvim_types::{
    Array,
    BufHandle,
    Dictionary,
    Error,
    Integer,
    NonOwning,
    Object,
    String,
};

#[cfg(not(feature = "neovim-0-7"))]
use crate::opts::KeyDict_set_decoration_provider;
use crate::opts::KeyDict_set_extmark;

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L863
    pub(crate) fn nvim_buf_add_highlight(
        buf: BufHandle,
        ns_id: Integer,
        hl_group: NonOwning<String>,
        line: Integer,
        col_start: Integer,
        col_end: Integer,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L926
    pub(crate) fn nvim_buf_clear_namespace(
        buf: BufHandle,
        ns_id: Integer,
        line_start: Integer,
        line_end: Integer,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L805
    pub(crate) fn nvim_buf_del_extmark(
        buf: BufHandle,
        ns_id: Integer,
        id: Integer,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L199
    pub(crate) fn nvim_buf_get_extmark_by_id(
        buf: BufHandle,
        ns_id: Integer,
        id: Integer,
        opts: NonOwning<Dictionary>,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L286
    pub(crate) fn nvim_buf_get_extmarks(
        buf: BufHandle,
        ns_id: Integer,
        start: Object,
        end: Object,
        opts: NonOwning<Dictionary>,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L482
    pub(crate) fn nvim_buf_set_extmark(
        buf: BufHandle,
        ns_id: Integer,
        line: Integer,
        col: Integer,
        opts: *const KeyDict_set_extmark,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/extmark.c#L45
    pub(crate) fn nvim_create_namespace(name: NonOwning<String>) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L63
    pub(crate) fn nvim_get_namespaces() -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.8.3/src/nvim/api/extmark.c#L1000
    pub(crate) fn nvim_set_decoration_provider(
        ns_id: Integer,
        #[cfg(feature = "neovim-0-7")] opts: NonOwning<Dictionary>,
        #[cfg(not(feature = "neovim-0-7"))]
        opts: *const KeyDict_set_decoration_provider,
        err: *mut Error,
    );
}
