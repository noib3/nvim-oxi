use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/options.c#L266
    pub(crate) fn nvim_get_all_options_info(
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/options.c#L305
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    pub(crate) fn nvim_get_option_info2(
        name: NvimStr,
        opts: *const OptionOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/options.c#L152
    pub(crate) fn nvim_get_option_value(
        name: NvimStr,
        opts: *const OptionOpts,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/options.c#L217
    pub(crate) fn nvim_set_option_value(
        channel_id: u64,
        name: NvimStr,
        value: NonOwning<Object>,
        opts: *const OptionOpts,
        err: *mut Error,
    );
}
