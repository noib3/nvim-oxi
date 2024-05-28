use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L152
    pub(crate) fn nvim_buf_attach(
        channel_id: u64,
        buf: BufHandle,
        send_buffer: bool,
        #[cfg(not(feature = "neovim-nightly"))] opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const BufAttachOpts,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L1258
    pub(crate) fn nvim_buf_call(
        buf: BufHandle,
        fun: LuaRef,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L949
    pub(crate) fn nvim_buf_del_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: NonOwning<String>,
        lhs: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L1127
    pub(crate) fn nvim_buf_del_mark(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L979
    pub(crate) fn nvim_buf_del_var(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L1060
    pub(crate) fn nvim_buf_delete(
        buf: BufHandle,
        #[cfg(not(feature = "neovim-nightly"))] opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const BufDeleteOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L901
    pub(crate) fn nvim_buf_get_changedtick(
        buf: BufHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.8.3/src/nvim/api/buffer.c#L920
    pub(crate) fn nvim_buf_get_keymap(
        buf: BufHandle,
        mode: NonOwning<String>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L274
    pub(crate) fn nvim_buf_get_lines(
        channel_id: u64,
        buf: BufHandle,
        start: Integer,
        end: Integer,
        strict_indexing: bool,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        lstate: *mut luajit::ffi::lua_State,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L1204
    pub(crate) fn nvim_buf_get_mark(
        buf: BufHandle,
        name: NonOwning<String>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L996
    pub(crate) fn nvim_buf_get_name(
        buf: BufHandle,
        arena: *mut Arena,
        err: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L857
    pub(crate) fn nvim_buf_get_offset(
        buf: BufHandle,
        index: Integer,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L757
    pub(crate) fn nvim_buf_get_text(
        channel_id: u64,
        buf: BufHandle,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        #[cfg(not(feature = "neovim-nightly"))] opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const GetTextOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        lstate: *mut luajit::ffi::lua_State,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L883
    pub(crate) fn nvim_buf_get_var(
        buf: BufHandle,
        name: NonOwning<String>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L1045
    pub(crate) fn nvim_buf_is_loaded(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L1109
    pub(crate) fn nvim_buf_is_valid(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L67
    pub(crate) fn nvim_buf_line_count(
        buf: BufHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L937
    pub(crate) fn nvim_buf_set_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: NonOwning<String>,
        lhs: NonOwning<String>,
        rhs: NonOwning<String>,
        opts: *const SetKeymapOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L347
    pub(crate) fn nvim_buf_set_lines(
        channel_id: u64,
        buf: BufHandle,
        start: Integer,
        end: Integer,
        strict_indexing: bool,
        replacement: NonOwning<Array>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L1172
    pub(crate) fn nvim_buf_set_mark(
        buf: BufHandle,
        name: NonOwning<String>,
        line: Integer,
        col: Integer,
        #[cfg(not(feature = "neovim-nightly"))] opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const SetMarkOpts,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L1014
    pub(crate) fn nvim_buf_set_name(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L515
    pub(crate) fn nvim_buf_set_text(
        channel_id: u64,
        buf: BufHandle,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        replacement: NonOwning<Array>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/buffer.c#L962
    pub(crate) fn nvim_buf_set_var(
        buf: BufHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );
}
