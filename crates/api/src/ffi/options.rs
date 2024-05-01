use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L289
    pub(crate) fn nvim_get_all_options_info(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L146
    pub(crate) fn nvim_get_option_value(
        name: NonOwning<String>,
        opts: *const OptionValueOpts,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/options.c#L232
    pub(crate) fn nvim_set_option_value(
        #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
        channel_id: u64,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        opts: *const OptionValueOpts,
        err: *mut Error,
    );
}
