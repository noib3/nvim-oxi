use types::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
unsafe extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L559
    pub(crate) fn nvim_buf_get_option(
        buf: BufHandle,
        name: NvimStr,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L581
    pub(crate) fn nvim_buf_set_option(
        channel_id: u64,
        buf: BufHandle,
        name: NvimStr,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L37
    pub(crate) fn nvim_exec(
        channel_id: u64,
        src: NvimStr,
        output: Boolean,
        error: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L186
    pub(crate) fn nvim_get_hl_by_id(
        hl_id: Integer,
        rgb: bool,
        arena: *mut core::ffi::c_void,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L207
    pub(crate) fn nvim_get_hl_by_name(
        name: NvimStr,
        rgb: bool,
        arena: *mut core::ffi::c_void,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L545
    pub(crate) fn nvim_get_option(name: NvimStr, err: *mut Error) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L518
    pub(crate) fn nvim_get_option_info(
        name: NvimStr,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L527
    pub(crate) fn nvim_notify(
        msg: NvimStr,
        log_level: Integer,
        opts: NonOwning<Dictionary>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L532
    pub(crate) fn nvim_set_option(
        channel_id: u64,
        name: NvimStr,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L601
    pub(crate) fn nvim_win_get_option(
        win: WinHandle,
        name: NvimStr,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/deprecated.c#L623
    pub(crate) fn nvim_win_set_option(
        channel_id: u64,
        win: WinHandle,
        name: NvimStr,
        value: NonOwning<Object>,
        err: *mut Error,
    );
}
