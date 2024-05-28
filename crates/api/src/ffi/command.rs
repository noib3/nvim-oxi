use types::*;

use crate::opts::*;

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
pub(crate) type ParseCmdOutput = Dictionary;

#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
pub(crate) type ParseCmdOutput = crate::types::KeyDict_cmd;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/command.c#L902
    pub(crate) fn nvim_buf_create_user_command(
        channel_id: u64,
        buf: BufHandle,
        name: NonOwning<String>,
        command: NonOwning<Object>,
        opts: *const CreateCommandOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/buffer.c#L925
    pub(crate) fn nvim_buf_del_user_command(
        buf: BufHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/command.c#L1182
    pub(crate) fn nvim_buf_get_commands(
        buf: BufHandle,
        opts: *const GetCommandsOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/command.c#L308
    pub(crate) fn nvim_cmd(
        channel_id: u64,
        cmd: *const crate::types::KeyDict_cmd,
        opts: *const CmdOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        err: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/command.c#L880
    pub(crate) fn nvim_create_user_command(
        channel_id: u64,
        name: NonOwning<String>,
        command: NonOwning<Object>,
        opts: *const CreateCommandOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/command.c#L891
    pub(crate) fn nvim_del_user_command(
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/command.c#L1169
    pub(crate) fn nvim_get_commands(
        opts: *const GetCommandsOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/command.c#L99
    pub(crate) fn nvim_parse_cmd(
        src: NonOwning<String>,
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        opts: *const ParseCmdOpts,
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        arena: *mut Arena,
        error: *mut Error,
    ) -> ParseCmdOutput;
}
