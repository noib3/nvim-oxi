#[cfg(feature = "neovim-nightly")]
use types::Arena;
use types::{Array, Dictionary, Error, NonOwning, Object, String};

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L283
    pub(crate) fn nvim_call_dict_function(
        dict: NonOwning<Object>,
        r#fn: NonOwning<String>,
        args: NonOwning<Array>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L268
    pub(crate) fn nvim_call_function(
        r#fn: NonOwning<String>,
        args: NonOwning<Array>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L138
    pub(crate) fn nvim_command(command: NonOwning<String>, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L154
    pub(crate) fn nvim_eval(
        expr: NonOwning<String>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L438
    pub fn nvim_parse_expression(
        expr: NonOwning<String>,
        flags: NonOwning<String>,
        highlight: bool,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;
}
