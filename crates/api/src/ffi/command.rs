use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/command.c#L938
    pub(crate) fn nvim_buf_create_user_command(
        #[cfg(not(feature = "neovim-0-8"))] channel_id: u64,
        buf: BufHandle,
        name: NonOwning<String>,
        command: NonOwning<Object>,
        opts: *const CreateCommandOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/command.c#L1243
    pub(crate) fn nvim_buf_get_commands(
        buf: BufHandle,
        opts: *const GetCommandsOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/command.c#L1230
    pub(crate) fn nvim_get_commands(
        opts: *const GetCommandsOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/command.c#L98
    pub(crate) fn nvim_parse_cmd(
        src: NonOwning<String>,
        #[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
        opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const ParseCmdOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        error: *mut Error,
    ) -> Dictionary;
}
