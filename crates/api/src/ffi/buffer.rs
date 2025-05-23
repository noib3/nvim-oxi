use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
unsafe extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L164
    pub(crate) fn nvim_buf_attach(
        channel_id: u64,
        buf: BufHandle,
        send_buffer: bool,
        opts: *const BufAttachOpts,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L1203
    pub(crate) fn nvim_buf_call(
        buf: BufHandle,
        fun: LuaRef,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L898
    pub(crate) fn nvim_buf_del_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: NvimStr,
        lhs: NvimStr,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L1071
    pub(crate) fn nvim_buf_del_mark(
        buf: BufHandle,
        name: NvimStr,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L928
    pub(crate) fn nvim_buf_del_var(
        buf: BufHandle,
        name: NvimStr,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L1020
    pub(crate) fn nvim_buf_delete(
        buf: BufHandle,
        opts: *const BufDeleteOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L850
    pub(crate) fn nvim_buf_get_changedtick(
        buf: BufHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.8.3/src/nvim/api/buffer.c#L869
    pub(crate) fn nvim_buf_get_keymap(
        buf: BufHandle,
        mode: NvimStr,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L249
    pub(crate) fn nvim_buf_get_lines(
        channel_id: u64,
        buf: BufHandle,
        start: Integer,
        end: Integer,
        strict_indexing: bool,
        arena: *mut Arena,
        lstate: *mut luajit::ffi::State,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L1149
    pub(crate) fn nvim_buf_get_mark(
        buf: BufHandle,
        name: NvimStr,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L945
    pub(crate) fn nvim_buf_get_name(
        buf: BufHandle,
        arena: *mut Arena,
        err: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L806
    pub(crate) fn nvim_buf_get_offset(
        buf: BufHandle,
        index: Integer,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L716
    pub(crate) fn nvim_buf_get_text(
        channel_id: u64,
        buf: BufHandle,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        opts: *const GetTextOpts,
        arena: *mut Arena,
        lstate: *mut luajit::ffi::State,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L832
    pub(crate) fn nvim_buf_get_var(
        buf: BufHandle,
        name: NvimStr,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L1005
    pub(crate) fn nvim_buf_is_loaded(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L1053
    pub(crate) fn nvim_buf_is_valid(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L74
    pub(crate) fn nvim_buf_line_count(
        buf: BufHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L886
    pub(crate) fn nvim_buf_set_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: NvimStr,
        lhs: NvimStr,
        rhs: NvimStr,
        opts: *const SetKeymapOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L315
    pub(crate) fn nvim_buf_set_lines(
        channel_id: u64,
        buf: BufHandle,
        start: Integer,
        end: Integer,
        strict_indexing: bool,
        replacement: NonOwning<Array>,
        arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L1116
    pub(crate) fn nvim_buf_set_mark(
        buf: BufHandle,
        name: NvimStr,
        line: Integer,
        col: Integer,
        opts: *const SetMarkOpts,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L963
    pub(crate) fn nvim_buf_set_name(
        buf: BufHandle,
        name: NvimStr,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L480
    pub(crate) fn nvim_buf_set_text(
        channel_id: u64,
        buf: BufHandle,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        replacement: NonOwning<Array>,
        arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L911
    pub(crate) fn nvim_buf_set_var(
        buf: BufHandle,
        name: NvimStr,
        value: NonOwning<Object>,
        err: *mut Error,
    );
}
