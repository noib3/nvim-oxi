use types::{
    Array,
    Error,
    Integer,
    NonOwning,
    Object,
    String,
    TabHandle,
    WinHandle,
};

#[link(name = "nvim", kind = "raw-dylib")]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/tabpage.c#L86
    pub(crate) fn nvim_tabpage_del_var(
        tabpage: TabHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/tabpage.c#L129
    pub(crate) fn nvim_tabpage_get_number(
        tabpage: TabHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/tabpage.c#L51
    pub(crate) fn nvim_tabpage_get_var(
        tabpage: TabHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/tabpage.c#L103
    pub(crate) fn nvim_tabpage_get_win(
        tabpage: TabHandle,
        err: *mut Error,
    ) -> WinHandle;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/tabpage.c#L145
    pub(crate) fn nvim_tabpage_is_valid(tabpage: TabHandle) -> bool;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/tabpage.c#L21
    pub(crate) fn nvim_tabpage_list_wins(
        tabpage: TabHandle,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/tabpage.c#L69
    pub(crate) fn nvim_tabpage_set_var(
        tabpage: TabHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );
}
