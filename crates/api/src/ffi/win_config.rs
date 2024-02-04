use types::{Boolean, BufHandle, Dictionary, Error, WinHandle};

use crate::types::KeyDict_float_config;

extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/win_config.c#L159
    pub(crate) fn nvim_open_win(
        buffer: BufHandle,
        enter: Boolean,
        config: *const KeyDict_float_config,
        err: *mut Error,
    ) -> WinHandle;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/win_config.c#L240
    pub(crate) fn nvim_win_get_config(
        window: WinHandle,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/win_config.c#L202
    pub(crate) fn nvim_win_set_config(
        window: WinHandle,
        config: *const KeyDict_float_config,
        err: *mut Error,
    );
}
