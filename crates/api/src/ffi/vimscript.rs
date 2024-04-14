#[cfg(feature = "neovim-nightly")]
use types::Arena;
use types::{Array, Boolean, Dictionary, Error, NonOwning, Object, String};

use crate::opts::*;

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

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/command.c#L320
    pub(crate) fn nvim_cmd(
        channel_id: u64,
        cmd: *const crate::types::KeyDict_cmd,
        opts: *const CmdOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L138
    pub(crate) fn nvim_command(command: NonOwning<String>, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L154
    pub(crate) fn nvim_eval(
        expr: NonOwning<String>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/deprecated.c#L33
    pub(crate) fn nvim_exec(
        channel_id: u64,
        src: NonOwning<String>,
        output: Boolean,
        error: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/command.c#L98
    pub(crate) fn nvim_parse_cmd(
        src: NonOwning<String>,
        #[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
        opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const ParseCmdOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L438
    pub fn nvim_parse_expression(
        expr: NonOwning<String>,
        flags: NonOwning<String>,
        highlight: bool,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;
}
