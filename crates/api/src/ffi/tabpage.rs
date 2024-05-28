use types::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
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
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
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
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/tabpage.c#L69
    pub(crate) fn nvim_tabpage_set_var(
        tabpage: TabHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/tabpage.c#L130
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    pub(crate) fn nvim_tabpage_set_win(
        tabpage: TabHandle,
        win: WinHandle,
        err: *mut Error,
    );
}
