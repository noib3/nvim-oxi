use types::{self as nvim, conversion::FromObject};

use super::opts::*;
use crate::Buffer;
use crate::LUA_INTERNAL_CALL;
use crate::Result;
use crate::choose;
use crate::ffi::command::*;
use crate::trait_utils::{StringOrFunction, SuperIterator};
use crate::types::*;

/// Binding to [`nvim_cmd()`][1].
///
/// Executes an Ex command. Unlike `crare::api::command` it takes a structured
/// `CmdInfos` object instead of a string.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_cmd()
pub fn cmd(infos: &CmdInfos, opts: &CmdOpts) -> Result<Option<String>> {
    let mut err = nvim::Error::new();
    let output = unsafe {
        nvim_cmd(
            LUA_INTERNAL_CALL,
            &infos.into(),
            opts,
            types::arena(),
            &mut err,
        )
    };
    choose!(err, {
        Ok((!output.is_empty()).then(|| output.to_string_lossy().into()))
    })
}

/// Binding to [`nvim_create_user_command()`][1].
///
/// Creates a new [user command](https://neovim.io/doc/user/map.html#user-commands).
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_create_user_command()
pub fn create_user_command<Cmd>(
    name: &str,
    command: Cmd,
    opts: &CreateCommandOpts,
) -> Result<()>
where
    Cmd: StringOrFunction<CommandArgs, ()>,
{
    let name = nvim::String::from(name);
    let command = command.to_object();
    let mut err = nvim::Error::new();
    unsafe {
        nvim_create_user_command(
            LUA_INTERNAL_CALL,
            name.as_nvim_str(),
            command.non_owning(),
            opts,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_del_user_command()`][1].
///
/// Deletes a global user-defined command.  Use [`Buffer::del_user_command`] to
/// delete a buffer-local command.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_user_command()
pub fn del_user_command(name: &str) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe { nvim_del_user_command(name.as_nvim_str(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_get_commands()`][1].
///
/// Returns an iterator over the infos of the global ex commands. Only
/// user-defined commands are returned, not builtin ones.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_commands()
pub fn get_commands(
    opts: &GetCommandsOpts,
) -> Result<impl SuperIterator<CommandInfos> + use<>> {
    let mut err = nvim::Error::new();
    let cmds = unsafe { nvim_get_commands(opts, types::arena(), &mut err) };
    choose!(
        err,
        Ok({
            cmds.into_iter()
                .map(|(_, cmd)| CommandInfos::from_object(cmd).unwrap())
        })
    )
}

/// Binding to [`nvim_parse_cmd()`][1].
///
/// Parses the command line.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_parse_cmd()
pub fn parse_cmd(src: &str, opts: &ParseCmdOpts) -> Result<CmdInfos> {
    let src = nvim::String::from(src);
    let mut err = nvim::Error::new();

    let out = unsafe {
        nvim_parse_cmd(src.as_nvim_str(), opts, types::arena(), &mut err)
    }
    .try_into()?;

    choose!(err, Ok(out))
}

impl Buffer {
    /// Binding to [`nvim_buf_create_user_command()`][1].
    ///
    /// Creates a new buffer-local user command.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_create_user_command()
    pub fn create_user_command<Cmd>(
        &mut self,
        name: &str,
        command: Cmd,
        opts: &CreateCommandOpts,
    ) -> Result<()>
    where
        Cmd: StringOrFunction<CommandArgs, ()>,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let command = command.to_object();
        unsafe {
            nvim_buf_create_user_command(
                LUA_INTERNAL_CALL,
                self.0,
                name.as_nvim_str(),
                command.non_owning(),
                opts,
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_del_user_command()`][1].
    ///
    /// Deletes a buffer-local user-command. Use
    /// [`del_user_command`](crate::del_user_command) to delete a global
    /// command.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_del_user_command()
    pub fn del_user_command(&mut self, name: &str) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe {
            nvim_buf_del_user_command(self.0, name.as_nvim_str(), &mut err)
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_get_commands()`][1].
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_commands()
    pub fn get_commands(
        &self,
        opts: &GetCommandsOpts,
    ) -> Result<impl SuperIterator<CommandInfos> + use<>> {
        let mut err = nvim::Error::new();
        let cmds = unsafe {
            nvim_buf_get_commands(self.0, opts, types::arena(), &mut err)
        };
        choose!(
            err,
            Ok({
                cmds.into_iter()
                    .map(|(_, cmd)| CommandInfos::from_object(cmd).unwrap())
            })
        )
    }
}
