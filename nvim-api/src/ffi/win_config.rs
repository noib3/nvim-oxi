use nvim_types::{Boolean, BufHandle, Dictionary, Error, WinHandle};

use crate::types::KeyDict_float_config;

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/win_config.c#L144
    pub(crate) fn nvim_open_win(
        buffer: BufHandle,
        enter: Boolean,
        config: *const KeyDict_float_config,
        err: *mut Error,
    ) -> WinHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/win_config.c#L225
    pub(crate) fn nvim_win_get_config(
        window: WinHandle,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/win_config.c#L187
    pub(crate) fn nvim_win_set_config(
        window: WinHandle,
        config: *const KeyDict_float_config,
        err: *mut Error,
    );
}
