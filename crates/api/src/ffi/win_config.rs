use types::*;

use crate::types::WindowOpts;

pub(crate) type WinGetConfigOutput = WindowOpts;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
unsafe extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/win_config.c#L159
    pub(crate) fn nvim_open_win(
        buffer: BufHandle,
        enter: Boolean,
        config: *const WindowOpts,
        err: *mut Error,
    ) -> WinHandle;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/win_config.c#L240
    pub(crate) fn nvim_win_get_config(
        window: WinHandle,
        arena: *mut Arena,
        err: *mut Error,
    ) -> WinGetConfigOutput;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/win_config.c#L202
    pub(crate) fn nvim_win_set_config(
        window: WinHandle,
        config: *const WindowOpts,
        err: *mut Error,
    );
}
