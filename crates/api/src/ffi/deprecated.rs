use types::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L373
    pub(crate) fn nvim_buf_get_option(
        buf: BufHandle,
        name: NonOwning<String>,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L393
    pub(crate) fn nvim_buf_set_option(
        channel_id: u64,
        buf: BufHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/deprecated.c#L33
    pub(crate) fn nvim_exec(
        channel_id: u64,
        src: NonOwning<String>,
        output: Boolean,
        error: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/deprecated.c#L188
    pub(crate) fn nvim_get_hl_by_id(
        hl_id: Integer,
        rgb: bool,
        arena: *mut core::ffi::c_void,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/deprecated.c#L209
    pub(crate) fn nvim_get_hl_by_name(
        name: NonOwning<String>,
        rgb: bool,
        arena: *mut core::ffi::c_void,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L361
    pub(crate) fn nvim_get_option(
        name: NonOwning<String>,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/deprecated.c#L520
    pub(crate) fn nvim_get_option_info(
        name: NonOwning<String>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L350
    pub(crate) fn nvim_set_option(
        channel_id: u64,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L411
    pub(crate) fn nvim_win_get_option(
        win: WinHandle,
        name: NonOwning<String>,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L431
    pub(crate) fn nvim_win_set_option(
        channel_id: u64,
        win: WinHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );
}
