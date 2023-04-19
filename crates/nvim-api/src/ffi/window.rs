use nvim_types::{
    Array,
    Boolean,
    BufHandle,
    Error,
    Integer,
    LuaRef,
    NonOwning,
    Object,
    String,
    TabHandle,
    WinHandle,
};

extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L428
    pub(crate) fn nvim_win_call(
        win: WinHandle,
        fun: LuaRef,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L392
    pub(crate) fn nvim_win_close(
        win: WinHandle,
        force: Boolean,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L268
    pub(crate) fn nvim_win_del_var(
        win: WinHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L32
    pub(crate) fn nvim_win_get_buf(
        win: WinHandle,
        err: *mut Error,
    ) -> BufHandle;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L63
    pub(crate) fn nvim_win_get_cursor(
        win: WinHandle,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L139
    pub(crate) fn nvim_win_get_height(
        win: WinHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L322
    pub(crate) fn nvim_win_get_number(
        win: WinHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L411
    pub(crate) fn nvim_win_get_option(
        win: WinHandle,
        name: NonOwning<String>,
        #[cfg(not(feature = "neovim-0-8"))] arena: *mut core::ffi::c_void,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L285
    pub(crate) fn nvim_win_get_position(
        win: WinHandle,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L304
    pub(crate) fn nvim_win_get_tabpage(
        win: WinHandle,
        err: *mut Error,
    ) -> TabHandle;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L233
    pub(crate) fn nvim_win_get_var(
        win: WinHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L185
    pub(crate) fn nvim_win_get_width(
        win: WinHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L360
    pub(crate) fn nvim_win_hide(win: WinHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L342
    pub(crate) fn nvim_win_is_valid(win: WinHandle) -> Boolean;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L49
    pub(crate) fn nvim_win_set_buf(
        win: WinHandle,
        buf: BufHandle,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L83
    pub(crate) fn nvim_win_set_cursor(
        win: WinHandle,
        pos: NonOwning<Array>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L156
    pub(crate) fn nvim_win_set_height(
        win: WinHandle,
        height: Integer,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L431
    pub(crate) fn nvim_win_set_option(
        channel_id: u64,
        win: WinHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L251
    pub(crate) fn nvim_win_set_var(
        win: WinHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/window.c#L203
    pub(crate) fn nvim_win_set_width(
        win: WinHandle,
        width: Integer,
        err: *mut Error,
    );
}
