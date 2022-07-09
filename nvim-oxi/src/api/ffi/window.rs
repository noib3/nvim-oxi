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
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L410
    pub(crate) fn nvim_win_call(
        win: WinHandle,
        fun: LuaRef,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L374
    pub(crate) fn nvim_win_close(
        win: WinHandle,
        force: Boolean,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L255
    pub(crate) fn nvim_win_del_var(
        win: WinHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L30
    pub(crate) fn nvim_win_get_buf(
        win: WinHandle,
        err: *mut Error,
    ) -> BufHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L59
    pub(crate) fn nvim_win_get_cursor(
        win: WinHandle,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L130
    pub(crate) fn nvim_win_get_height(
        win: WinHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L309
    pub(crate) fn nvim_win_get_number(
        win: WinHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/options.c#L309
    pub(crate) fn nvim_win_get_option(
        win: WinHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L272
    pub(crate) fn nvim_win_get_position(
        win: WinHandle,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L291
    pub(crate) fn nvim_win_get_tabpage(
        win: WinHandle,
        err: *mut Error,
    ) -> TabHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L220
    pub(crate) fn nvim_win_get_var(
        win: WinHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L174
    pub(crate) fn nvim_win_get_width(
        win: WinHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L347
    pub(crate) fn nvim_win_hide(win: WinHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L329
    pub(crate) fn nvim_win_is_valid(win: WinHandle) -> Boolean;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L47
    pub(crate) fn nvim_win_set_buf(
        win: WinHandle,
        buf: BufHandle,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L79
    pub(crate) fn nvim_win_set_cursor(
        win: WinHandle,
        pos: NonOwning<Array>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L147
    pub(crate) fn nvim_win_set_height(
        win: WinHandle,
        height: Integer,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/options.c#L329
    pub(crate) fn nvim_win_set_option(
        channel_id: u64,
        win: WinHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L238
    pub(crate) fn nvim_win_set_var(
        win: WinHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/window.c#L192
    pub(crate) fn nvim_win_set_width(
        win: WinHandle,
        width: Integer,
        err: *mut Error,
    );
}
