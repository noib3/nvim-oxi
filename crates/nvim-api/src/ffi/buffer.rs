use nvim_types::{
    Array,
    BufHandle,
    Dictionary,
    Error,
    Integer,
    LuaRef,
    NonOwning,
    Object,
    String,
};

use crate::opts::{
    KeyDict_get_commands,
    KeyDict_keymap,
    KeyDict_user_command,
};

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L145
    pub(crate) fn nvim_buf_attach(
        channel_id: u64,
        buf: BufHandle,
        send_buffer: bool,
        opts: NonOwning<Dictionary>,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1358
    pub(crate) fn nvim_buf_call(
        buf: BufHandle,
        fun: LuaRef,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1383
    pub(crate) fn nvim_buf_create_user_command(
        buf: BufHandle,
        name: NonOwning<String>,
        command: NonOwning<Object>,
        opts: *const KeyDict_user_command,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L969
    pub(crate) fn nvim_buf_del_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: NonOwning<String>,
        lhs: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1216
    pub(crate) fn nvim_buf_del_mark(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1406
    pub(crate) fn nvim_buf_del_user_command(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1030
    pub(crate) fn nvim_buf_del_var(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1150
    pub(crate) fn nvim_buf_delete(
        buf: BufHandle,
        opts: NonOwning<Dictionary>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L921
    pub(crate) fn nvim_buf_get_changedtick(
        buf: BufHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L983
    pub(crate) fn nvim_buf_get_commands(
        buf: BufHandle,
        opts: *const KeyDict_get_commands,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L940
    pub(crate) fn nvim_buf_get_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: NonOwning<String>,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L269
    pub(crate) fn nvim_buf_get_lines(
        channel_id: u64,
        buf: BufHandle,
        start: Integer,
        end: Integer,
        strict_indexing: bool,
        #[cfg(feature = "neovim-nightly")]
        lstate: *mut luajit_bindings::ffi::lua_State,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1299
    pub(crate) fn nvim_buf_get_mark(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1086
    pub(crate) fn nvim_buf_get_name(buf: BufHandle, err: *mut Error)
        -> String;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L876
    pub(crate) fn nvim_buf_get_offset(
        buf: BufHandle,
        index: Integer,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1049
    pub(crate) fn nvim_buf_get_option(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L778
    pub(crate) fn nvim_buf_get_text(
        channel_id: u64,
        buf: BufHandle,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")]
        lstate: *mut luajit_bindings::ffi::lua_State,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1049
    pub(crate) fn nvim_buf_get_var(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1135
    pub(crate) fn nvim_buf_is_loaded(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1198
    pub(crate) fn nvim_buf_is_valid(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1049
    pub(crate) fn nvim_buf_line_count(
        buf: BufHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L957
    pub(crate) fn nvim_buf_set_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: NonOwning<String>,
        lhs: NonOwning<String>,
        rhs: NonOwning<String>,
        opts: *const KeyDict_keymap,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L365
    pub(crate) fn nvim_buf_set_lines(
        channel_id: u64,
        buf: BufHandle,
        start: Integer,
        end: Integer,
        strict_indexing: bool,
        replacement: NonOwning<Array>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1265
    pub(crate) fn nvim_buf_set_mark(
        buf: BufHandle,
        name: NonOwning<String>,
        line: Integer,
        col: Integer,
        opts: NonOwning<Dictionary>,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1104
    pub(crate) fn nvim_buf_set_name(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1069
    pub(crate) fn nvim_buf_set_option(
        channel_id: u64,
        buf: BufHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L538
    pub(crate) fn nvim_buf_set_text(
        channel_id: u64,
        buf: BufHandle,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        replacement: NonOwning<Array>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1013
    pub(crate) fn nvim_buf_set_var(
        buf: BufHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );
}
