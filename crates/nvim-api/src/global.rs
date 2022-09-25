use std::path::{Path, PathBuf};

use nvim_types::{
    self as nvim,
    Array,
    Dictionary,
    FromObject,
    Integer,
    Object,
    ToObject,
};

use super::ffi::global::*;
use super::opts::*;
use super::types::*;
use super::LUA_INTERNAL_CALL;
use crate::iterator::SuperIterator;
use crate::trait_utils::StringOrFunction;
use crate::{Buffer, TabPage, Window};
use crate::{Error, Result};

/// Binding to [`nvim_chan_send`](https://neovim.io/doc/user/api.html#nvim_chan_send()).
///
/// Sends data to a channel.
pub fn chan_send(channel_id: u32, data: &str) -> Result<()> {
    let mut err = nvim::Error::new();
    let data = nvim::String::from(data);
    unsafe { nvim_chan_send(channel_id.into(), data.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_create_buf`](https://neovim.io/doc/user/api.html#nvim_create_buf()).
///
/// Creates a new, empty, unnamed buffer.
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = nvim::Error::new();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    err.into_err_or_else(|| handle.into())
}

/// Binding to [`nvim_create_user_command`](https://neovim.io/doc/user/api.html#nvim_create_user_command()).
///
/// Creates a new [user command](https://neovim.io/doc/user/map.html#user-commands).
pub fn create_user_command<Cmd>(
    name: &str,
    command: Cmd,
    opts: Option<&CreateCommandOpts>,
) -> Result<()>
where
    Cmd: StringOrFunction<CommandArgs, ()>,
{
    let name = nvim::String::from(name);
    let command = command.to_obj();
    let opts = opts.map(KeyDict_user_command::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    unsafe {
        nvim_create_user_command(
            name.non_owning(),
            command.non_owning(),
            &opts.into(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_del_current_line`](https://neovim.io/doc/user/api.html#nvim_del_current_line()).
///
/// Deletes the current line.
pub fn del_current_line() -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_del_current_line(&mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_del_keymap`](https://neovim.io/doc/user/api.html#nvim_del_keymap()).
///
/// Unmaps a global mapping for the given mode. To unmap a buffer-local mapping
/// use [`Buffer::del_keymap`] instead.
pub fn del_keymap(mode: Mode, lhs: &str) -> Result<()> {
    let mode = nvim::String::from(mode);
    let lhs = nvim::String::from(lhs);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_del_keymap(
            LUA_INTERNAL_CALL,
            mode.non_owning(),
            lhs.non_owning(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_del_mark`](https://neovim.io/doc/user/api.html#nvim_del_mark()).
///
/// Deletes an uppercase/file named mark. Returns an error if a lowercase or
/// buffer-local named mark is used. Use [`Buffer::del_mark`] to delete a
/// buffer-local mark.
pub fn del_mark(name: char) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let was_deleted = unsafe { nvim_del_mark(name.non_owning(), &mut err) };
    err.into_err_or_flatten(|| match was_deleted {
        true => Ok(()),
        _ => Err(Error::custom("Couldn't delete mark")),
    })
}

/// Binding to [`nvim_del_user_command`](https://neovim.io/doc/user/api.html#nvim_del_user_command()).
///
/// Deletes a global user-defined command.  Use [`Buffer::del_user_command`] to
/// delete a buffer-local command.
pub fn del_user_command(name: &str) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe { nvim_del_user_command(name.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_del_var`](https://neovim.io/doc/user/api.html#nvim_del_var()).
///
/// Removes a global (`g:`) variable.
pub fn del_var(name: &str) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe { nvim_del_var(name.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_echo`](https://neovim.io/doc/user/api.html#nvim_echo()).
///
/// Echoes a message to the Neovim message area.
pub fn echo<'hl, Text, Chunks>(chunks: Chunks, history: bool) -> Result<()>
where
    Chunks: IntoIterator<Item = (Text, Option<&'hl str>)>,
    Text: Into<nvim::String>,
{
    let chunks = chunks
        .into_iter()
        .map(|(text, hlgroup)| {
            Array::from_iter([
                Object::from(text.into()),
                Object::from(hlgroup.map(|hl| hl.to_owned())),
            ])
        })
        .collect::<Array>();

    let mut err = nvim::Error::new();
    let opts = Dictionary::new();
    unsafe {
        nvim_echo(chunks.non_owning(), history, opts.non_owning(), &mut err)
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_err_write`](https://neovim.io/doc/user/api.html#nvim_err_write()).
///
/// Writes a message to the Neovim error buffer. Does not append a newline
/// (`"\n"`); the message gets buffered and won't be displayed until a linefeed
/// is written.
pub fn err_write(str: &str) {
    unsafe { nvim_err_write(nvim::String::from(str).non_owning()) }
}

/// Binding to [`nvim_err_writeln`](https://neovim.io/doc/user/api.html#nvim_err_writeln()).
///
/// Writes a message to the Neovim error buffer. Appends a newline (`"\n"`), so
/// the buffer is flushed and displayed.
pub fn err_writeln(str: &str) {
    unsafe { nvim_err_writeln(nvim::String::from(str).non_owning()) }
}

/// Binding to [`nvim_eval_statusline`](https://neovim.io/doc/user/api.html#nvim_eval_statusline()).
///
/// Evaluates a string to be displayed in the statusline.
pub fn eval_statusline(
    str: &str,
    opts: Option<&EvalStatuslineOpts>,
) -> Result<StatuslineInfos> {
    let str = nvim::String::from(str);
    let opts = opts.map(KeyDict_eval_statusline::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    let dict = unsafe {
        nvim_eval_statusline(str.non_owning(), &opts.into(), &mut err)
    };
    err.into_err_or_flatten(|| Ok(StatuslineInfos::from_obj(dict.into())?))
}

/// Binding to [`nvim_feedkeys`](https://neovim.io/doc/user/api.html#nvim_feedkeys()).
pub fn feedkeys(keys: &str, mode: Mode, escape_ks: bool) {
    let keys = nvim::String::from(keys);
    let mode = nvim::String::from(mode);
    unsafe { nvim_feedkeys(keys.non_owning(), mode.non_owning(), escape_ks) }
}

/// Binding to [`nvim_get_all_options_info`](https://neovim.io/doc/user/api.html#nvim_get_all_options_info()).
///
/// Gets the option information for all options.
pub fn get_all_options_info() -> Result<impl SuperIterator<OptionInfos>> {
    let mut err = nvim::Error::new();
    let infos = unsafe { nvim_get_all_options_info(&mut err) };
    err.into_err_or_else(|| {
        infos
            .into_iter()
            .map(|(_, optinf)| OptionInfos::from_obj(optinf).unwrap())
    })
}

/// Binding to [`nvim_get_chan_info`](https://neovim.io/doc/user/api.html#nvim_get_chan_info()).
///
/// Gets information about a channel.
pub fn get_chan_info(channel_id: u32) -> Result<ChannelInfos> {
    let mut err = nvim::Error::new();
    let infos = unsafe { nvim_get_chan_info(channel_id.into(), &mut err) };
    err.into_err_or_flatten(|| Ok(ChannelInfos::from_obj(infos.into())?))
}

/// Binding to [`nvim_get_color_by_name`](https://neovim.io/doc/user/api.html#nvim_get_color_by_name()).
///
/// Returns the 24-bit RGB value of a `crate::api::get_color_map` color name or
/// "#rrggbb" hexadecimal string.
pub fn get_color_by_name(name: &str) -> Result<u32> {
    let name = nvim::String::from(name);
    let color = unsafe { nvim_get_color_by_name(name.non_owning()) };
    (color != -1).then(|| color.try_into().unwrap()).ok_or_else(|| {
        Error::custom(format!("{name} is not a valid color name"))
    })
}

/// Binding to [`nvim_get_color_map`](https://neovim.io/doc/user/api.html#nvim_get_color_map()).
///
/// Returns an iterator over tuples representing color names and 24-bit RGB
/// values (e.g. 65535).
pub fn get_color_map() -> impl SuperIterator<(String, u32)> {
    unsafe { nvim_get_color_map() }.into_iter().map(|(k, v)| {
        (String::try_from(k).unwrap(), u32::from_obj(v).unwrap())
    })
}

/// Binding to [`nvim_get_commands`](https://neovim.io/doc/user/api.html#nvim_get_commands()).
///
/// Returns an iterator over the infos of the global ex commands. Only
/// user-defined commands are returned, not builtin ones.
pub fn get_commands(
    opts: Option<&GetCommandsOpts>,
) -> Result<impl SuperIterator<CommandInfos>> {
    let opts = opts.map(KeyDict_get_commands::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    let cmds = unsafe { nvim_get_commands(&opts.into(), &mut err) };
    err.into_err_or_else(|| {
        cmds.into_iter().map(|(_, cmd)| CommandInfos::from_obj(cmd).unwrap())
    })
}

/// Binding to [`nvim_get_context`](https://neovim.io/doc/user/api.html#nvim_get_context()).
///
/// Returns a snapshot of the current editor state.
pub fn get_context(opts: Option<&GetContextOpts>) -> Result<EditorContext> {
    let opts = opts.map(KeyDict_context::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    let ctx = unsafe { nvim_get_context(&opts.into(), &mut err) };
    err.into_err_or_flatten(|| Ok(EditorContext::from_obj(ctx.into())?))
}

/// Binding to [`nvim_get_current_buf`](https://neovim.io/doc/user/api.html#nvim_get_current_buf()).
///
/// Gets the current buffer.
pub fn get_current_buf() -> Buffer {
    unsafe { nvim_get_current_buf() }.into()
}

/// Binding to [`nvim_get_current_line`](https://neovim.io/doc/user/api.html#nvim_get_current_line()).
///
/// Gets the current line in the current bufferr.
pub fn get_current_line() -> Result<String> {
    let mut err = nvim::Error::new();
    let str = unsafe { nvim_get_current_line(&mut err) };
    err.into_err_or_flatten(|| str.try_into().map_err(From::from))
}

/// Binding to [`nvim_get_current_tabpage`](https://neovim.io/doc/user/api.html#nvim_get_current_tabpage()).
///
/// Gets the current tabpage.
pub fn get_current_tabpage() -> TabPage {
    unsafe { nvim_get_current_tabpage() }.into()
}

/// Binding to [`nvim_get_current_win`](https://neovim.io/doc/user/api.html#nvim_get_current_win()).
///
/// Gets the current window.
pub fn get_current_win() -> Window {
    unsafe { nvim_get_current_win() }.into()
}

/// Binding to [`nvim_get_hl_by_id`](https://neovim.io/doc/user/api.html#nvim_get_hl_by_id()).
///
/// Gets a highlight definition by id.
pub fn get_hl_by_id(hl_id: u32, rgb: bool) -> Result<HighlightInfos> {
    let mut err = nvim::Error::new();
    let hl = unsafe { nvim_get_hl_by_id(hl_id.into(), rgb, &mut err) };
    err.into_err_or_flatten(|| Ok(HighlightInfos::from_obj(hl.into())?))
}

/// Binding to [`nvim_get_hl_by_name`](https://neovim.io/doc/user/api.html#nvim_get_hl_by_name()).
///
/// Gets a highlight definition by name.
pub fn get_hl_by_name(name: &str, rgb: bool) -> Result<HighlightInfos> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let hl = unsafe { nvim_get_hl_by_name(name.non_owning(), rgb, &mut err) };
    err.into_err_or_flatten(|| Ok(HighlightInfos::from_obj(hl.into())?))
}

/// Binding to [`nvim_get_hl_id_by_name`](https://neovim.io/doc/user/api.html#nvim_get_hl_id_by_name()).
///
/// Gets a highlight definition by name.
pub fn get_hl_id_by_name(name: &str) -> Result<u32> {
    let name = nvim::String::from(name);
    let id = unsafe { nvim_get_hl_id_by_name(name.non_owning()) };
    id.try_into().map_err(Into::into)
}

/// Binding to [`nvim_get_keymap`](https://neovim.io/doc/user/api.html#nvim_get_keymap()).
///
/// Returns an iterator over the global mapping definitions.
pub fn get_keymap(mode: Mode) -> impl SuperIterator<KeymapInfos> {
    let mode = nvim::String::from(mode);
    unsafe { nvim_get_keymap(LUA_INTERNAL_CALL, mode.non_owning()) }
        .into_iter()
        .map(|obj| KeymapInfos::from_obj(obj).unwrap())
}

/// Binding to [`nvim_get_mark`](https://neovim.io/doc/user/api.html#nvim_get_mark()).
///
/// Returns a tuple `(row, col, buffer, buffername)` representing the position
/// of the named mark. Marks are (1,0)-indexed.
pub fn get_mark(
    name: char,
    opts: Option<&GetMarkOpts>,
) -> Result<(usize, usize, Buffer, String)> {
    let name = nvim::String::from(name);
    let opts = opts.map(Dictionary::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    let mark = unsafe {
        nvim_get_mark(name.non_owning(), opts.non_owning(), &mut err)
    };
    err.into_err_or_flatten(|| {
        let mut iter = mark.into_iter();
        let row = usize::from_obj(iter.next().expect("row is present"))?;
        let col = usize::from_obj(iter.next().expect("col is present"))?;
        let buffer =
            Buffer::from_obj(iter.next().expect("buffer is present"))?;
        let buffername =
            String::from_obj(iter.next().expect("buffername is present"))?;
        Ok((row, col, buffer, buffername))
    })
}

/// Binding to [`nvim_get_mode`](https://neovim.io/doc/user/api.html#nvim_get_mode()).
///
/// Gets the current mode. The [`blocking`](GotMode::blocking) field of
/// [`GotMode`] is `true` if Neovim is waiting for input.
pub fn get_mode() -> Result<GotMode> {
    Ok(GotMode::from_obj(unsafe { nvim_get_mode() }.into())?)
}

/// Binding to [`nvim_get_option`](https://neovim.io/doc/user/api.html#nvim_get_option()).
///
/// Gets the value of a global option.
pub fn get_option<Opt>(name: &str) -> Result<Opt>
where
    Opt: FromObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let obj = unsafe { nvim_get_option(name.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Ok(Opt::from_obj(obj)?))
}

/// Binding to [`nvim_get_option_info`](https://neovim.io/doc/user/api.html#nvim_get_option_info()).
///
/// Gets all the informations related to an option.
pub fn get_option_info(name: &str) -> Result<OptionInfos> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let obj = unsafe { nvim_get_option_info(name.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Ok(OptionInfos::from_obj(obj.into())?))
}

/// Binding to [`nvim_get_option_value`](https://neovim.io/doc/user/api.html#nvim_get_option_value()).
///
/// Gets the local value of an option if it exists, or the global value
/// otherwise. Local values always correspond to the current buffer or window.
///
/// To get a buffer-local orr window-local option for a specific buffer of
/// window consider using [`Buffer::get_option`] or [`Window::get_option`] instead.
pub fn get_option_value<Opt>(
    name: &str,
    opts: Option<&OptionValueOpts>,
) -> Result<Opt>
where
    Opt: FromObject,
{
    let name = nvim::String::from(name);
    let opts = opts.map(KeyDict_option::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    let obj = unsafe {
        nvim_get_option_value(name.non_owning(), &opts.into(), &mut err)
    };
    err.into_err_or_flatten(|| Ok(Opt::from_obj(obj)?))
}

/// Binding to [`nvim_get_proc`](https://neovim.io/doc/user/api.html#nvim_get_proc()).
///
/// Gets informations about a process with a given `pid`.
pub fn get_proc(pid: u32) -> Result<ProcInfos> {
    let mut err = nvim::Error::new();
    let obj = unsafe { nvim_get_proc(pid.into(), &mut err) };
    err.into_err_or_flatten(|| Ok(ProcInfos::from_obj(obj)?))
}

/// Binding to [`nvim_get_proc_children`](https://neovim.io/doc/user/api.html#nvim_get_proc_children()).
///
/// Gets the immediate children of process `pid`.
pub fn get_proc_children(pid: u32) -> Result<impl SuperIterator<u32>> {
    let mut err = nvim::Error::new();
    let procs = unsafe { nvim_get_proc_children(pid.into(), &mut err) };
    err.into_err_or_else(|| {
        procs.into_iter().map(|obj| u32::from_obj(obj).unwrap())
    })
}

/// Binding to [`nvim_get_runtime_file`](https://neovim.io/doc/user/api.html#nvim_get_runtime_file()).
///
/// Returns an iterator over all the files matching `name` in the runtime path.
pub fn get_runtime_file(
    name: impl AsRef<Path>,
    get_all: bool,
) -> Result<impl SuperIterator<PathBuf>> {
    let name = nvim::String::from(name.as_ref().to_owned());
    let mut err = nvim::Error::new();
    let files =
        unsafe { nvim_get_runtime_file(name.non_owning(), get_all, &mut err) };
    err.into_err_or_else(|| {
        files
            .into_iter()
            .map(|obj| PathBuf::from(nvim::String::from_obj(obj).unwrap()))
    })
}

/// Binding to [`nvim_get_var`](https://neovim.io/doc/user/api.html#nvim_get_var()).
///
/// Gets a global (`g:`) variable.
pub fn get_var<Var>(name: &str) -> Result<Var>
where
    Var: FromObject,
{
    let mut err = nvim::Error::new();
    let name = nvim::String::from(name);
    let obj = unsafe { nvim_get_var(name.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Ok(Var::from_obj(obj)?))
}

/// Binding to [`nvim_get_vvar`](https://neovim.io/doc/user/api.html#nvim_get_vvar()).
///
/// Gets a `v:` variable.
pub fn get_vvar<Var>(name: &str) -> Result<Var>
where
    Var: FromObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let obj = unsafe { nvim_get_vvar(name.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Ok(Var::from_obj(obj)?))
}

/// Binding to [`nvim_input`](https://neovim.io/doc/user/api.html#nvim_input()).
///
/// Queues raw user-input. Unlike [`api::feedkeys`](feedkeys) this uses a
/// low-level input buffer and the call is non-blocking.
///
/// Returns the number of bytes written to the buffer.
pub fn input<Input>(keys: Input) -> Result<usize>
where
    Input: Into<nvim::String>,
{
    unsafe { nvim_input(keys.into().non_owning()) }
        .try_into()
        .map_err(From::from)
}

/// Binding to [`nvim_input_mouse`](https://neovim.io/doc/user/api.html#nvim_input_mouse()).
///
/// Send mouse event from GUI. The call is non-blocking.
pub fn input_mouse(
    button: MouseButton,
    action: MouseAction,
    modifier: &str,
    grid: u32,
    row: usize,
    col: usize,
) -> Result<()> {
    let button = nvim::String::from(button);
    let action = nvim::String::from(action);
    let modifier = nvim::String::from(modifier);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_input_mouse(
            button.non_owning(),
            action.non_owning(),
            modifier.non_owning(),
            grid.into(),
            row.try_into()?,
            col.try_into()?,
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_list_bufs`](https://neovim.io/doc/user/api.html#nvim_list_bufs()).
///
/// Gets the current list of [`Buffer`]s, including unlisted (unloaded/deleted)
/// buffers (like `:ls!`). Use [`Buffer::is_loaded`] to check if a
/// buffer is loaded.
pub fn list_bufs() -> impl SuperIterator<Buffer> {
    unsafe { nvim_list_bufs() }
        .into_iter()
        .map(|obj| Buffer::from_obj(obj).unwrap())
}

/// Binding to [`nvim_list_chans`](https://neovim.io/doc/user/api.html#nvim_list_chans()).
///
/// Returns an iterator over the informations about all the open channels.
pub fn list_chans() -> impl SuperIterator<ChannelInfos> {
    unsafe { nvim_list_chans() }
        .into_iter()
        .map(|obj| ChannelInfos::from_obj(obj).unwrap())
}

/// Binding to [`nvim_list_runtime_paths`](https://neovim.io/doc/user/api.html#nvim_list_runtime_paths()).
///
/// Gets the paths contained in https://neovim's runtimepath.
pub fn list_runtime_paths() -> Result<impl SuperIterator<PathBuf>> {
    let mut err = nvim::Error::new();
    let paths = unsafe { nvim_list_runtime_paths(&mut err) };
    err.into_err_or_else(|| {
        paths
            .into_iter()
            .map(|obj| PathBuf::from(nvim::String::from_obj(obj).unwrap()))
    })
}

/// Binding to [`nvim_list_bufs`](https://neovim.io/doc/user/api.html#nvim_list_bufs()).
///
/// Gets the current list of `Tabpage`s.
pub fn list_tabpages() -> impl SuperIterator<TabPage> {
    unsafe { nvim_list_tabpages() }
        .into_iter()
        .map(|obj| TabPage::from_obj(obj).unwrap())
}

/// Binding to [`nvim_list_uis`](https://neovim.io/doc/user/api.html#nvim_list_uis()).
///
/// Returns an iterator over the informations about all the attached UIs.
pub fn list_uis() -> impl SuperIterator<UiInfos> {
    unsafe { nvim_list_uis() }
        .into_iter()
        .map(|obj| UiInfos::from_obj(obj).unwrap())
}

/// Binding to [`nvim_list_wins`](https://neovim.io/doc/user/api.html#nvim_list_wins()).
///
/// Gets the current list of `Window`s.
pub fn list_wins() -> impl SuperIterator<Window> {
    unsafe { nvim_list_wins() }
        .into_iter()
        .map(|obj| Window::from_obj(obj).unwrap())
}

/// Binding to [`nvim_load_context`](https://neovim.io/doc/user/api.html#nvim_load_context()).
///
/// Sets the current editor state from the given [`EditorContext`].
pub fn load_context(ctx: EditorContext) {
    let ctx = Dictionary::from(ctx);
    let _ = unsafe { nvim_load_context(ctx.non_owning()) };
}

/// Binding to [`nvim_notify`](https://neovim.io/doc/user/api.html#nvim_notify()).
pub fn notify(
    msg: &str,
    log_level: LogLevel,
    opts: Option<&NotifyOpts>,
) -> Result<()> {
    let msg = nvim::String::from(msg);
    let opts = opts.map(Dictionary::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    let _ = unsafe {
        nvim_notify(
            msg.non_owning(),
            log_level as Integer,
            opts.non_owning(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_open_term`](https://neovim.io/doc/user/api.html#nvim_open_term()).
///
/// Opens a terminal instance in a buffer. Returns the id of a channel that can
/// be used to send data to the instance via
/// [`nvim_oxi::api::chan_send`](chan_send).
pub fn open_term(buffer: &Buffer, opts: Option<&OpenTermOpts>) -> Result<u32> {
    let opts = opts.map(Dictionary::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    let channel_id =
        unsafe { nvim_open_term(buffer.0, opts.non_owning(), &mut err) };
    err.into_err_or_flatten(|| match channel_id {
        0 => Err(Error::custom("Couldn't create terminal instance")),
        other => Ok(other.try_into().expect("always positive")),
    })
}

/// Binding to [`nvim_out_write`](https://neovim.io/doc/user/api.html#nvim_out_write()).
///
/// Writes a message to the Vim output buffer, without appending a "\n". The
/// message is buffered and won't be displayed until a linefeed is written.
pub fn out_write<Msg>(str: Msg)
where
    Msg: Into<nvim::String>,
{
    unsafe { nvim_out_write(str.into().non_owning()) }
}

/// Binding to [`nvim_paste`](https://neovim.io/doc/user/api.html#nvim_paste()).
///
/// Returns `true` if the client may continue the paste, `false` if it must
/// cancel it.
pub fn paste<Data>(data: Data, crlf: bool, phase: PastePhase) -> Result<bool>
where
    Data: Into<nvim::String>,
{
    let mut err = nvim::Error::new();
    let go_on = unsafe {
        nvim_paste(data.into().non_owning(), crlf, phase as Integer, &mut err)
    };
    err.into_err_or_else(|| go_on)
}

/// Binding to [`nvim_put`](https://neovim.io/doc/user/api.html#nvim_put()).
///
/// Puts text at cursor, in any mode.
pub fn put<Line, Lines>(
    lines: Lines,
    reg_type: RegisterType,
    after: bool,
    follow: bool,
) -> Result<()>
where
    Lines: Iterator<Item = Line>,
    Line: Into<nvim::String>,
{
    let lines = lines.into_iter().map(Into::into).collect::<Array>();
    let reg_type = nvim::String::from(reg_type);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_put(
            lines.non_owning(),
            reg_type.non_owning(),
            after,
            follow,
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_replace_termcodes`](https://neovim.io/doc/user/api.html#nvim_replace_termcodes()).
///
/// Replaces terminal codes and keycodes (`<CR>`, `<Esc>`, ...) in a string
/// with the internal representation.
pub fn replace_termcodes<Input>(
    str: Input,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> nvim::String
where
    Input: Into<nvim::String>,
{
    let str = str.into();
    unsafe {
        nvim_replace_termcodes(str.non_owning(), from_part, do_lt, special)
    }
}

/// Binding to [`nvim_select_popupmenu_item`](https://neovim.io/doc/user/api.html#nvim_select_popupmenu_item()).
///
/// Selects an item in the completion popupmenu.
pub fn select_popupmenu_item(
    item: usize,
    insert: bool,
    finish: bool,
    opts: Option<&SelectPopupMenuItemOpts>,
) -> Result<()> {
    let opts = opts.map(Dictionary::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    unsafe {
        nvim_select_popupmenu_item(
            item.try_into()?,
            insert,
            finish,
            opts.non_owning(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_current_buf`](https://neovim.io/doc/user/api.html#nvim_set_current_buf()).
///
/// Sets the current buffer.
pub fn set_current_buf(buf: &Buffer) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_buf(buf.0, &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_current_dir`](https://neovim.io/doc/user/api.html#nvim_set_current_dir()).
///
/// Changes the global working directory.
pub fn set_current_dir<Dir>(dir: Dir) -> Result<()>
where
    Dir: AsRef<Path>,
{
    let dir = nvim::String::from(dir.as_ref().to_owned());
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_dir(dir.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_current_line`](https://neovim.io/doc/user/api.html#nvim_set_current_line()).
///
/// Sets the current line.
pub fn set_current_line<Line>(line: Line) -> Result<()>
where
    Line: Into<nvim::String>,
{
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_line(line.into().non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_current_tabpage`](https://neovim.io/doc/user/api.html#nvim_set_current_tabpage()).
///
/// Sets the current tabpage.
pub fn set_current_tabpage(tabpage: &TabPage) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_tabpage(tabpage.0, &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_current_win`](https://neovim.io/doc/user/api.html#nvim_set_current_win()).
///
/// Sets the current window.
pub fn set_current_win(win: &Window) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_win(win.0, &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_hl`](https://neovim.io/doc/user/api.html#nvim_set_hl()).
///
/// Sets a highlight group.
pub fn set_hl(
    ns_id: u32,
    name: &str,
    opts: Option<&SetHighlightOpts>,
) -> Result<()> {
    let name = nvim::String::from(name);
    let opts = opts.map(KeyDict_highlight::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_hl(
            ns_id as Integer,
            name.non_owning(),
            &opts.into(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_keymap`](https://neovim.io/doc/user/api.html#nvim_set_keymap()).
///
/// Sets a global mapping for the given mode. To set a buffer-local mapping use
/// [`Buffer::set_keymap`] instead.
pub fn set_keymap(
    mode: Mode,
    lhs: &str,
    rhs: &str,
    opts: Option<&SetKeymapOpts>,
) -> Result<()> {
    let mode = nvim::String::from(mode);
    let lhs = nvim::String::from(lhs);
    let rhs = nvim::String::from(rhs);
    let opts = opts.map(KeyDict_keymap::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_keymap(
            LUA_INTERNAL_CALL,
            mode.non_owning(),
            lhs.non_owning(),
            rhs.non_owning(),
            &opts,
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_option`](https://neovim.io/doc/user/api.html#nvim_set_option()).
///
/// Sets the global value of an option.
pub fn set_option<Opt>(name: &str, value: Opt) -> Result<()>
where
    Opt: ToObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_option(
            LUA_INTERNAL_CALL,
            name.non_owning(),
            value.to_obj()?.non_owning(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_option_value`](https://neovim.io/doc/user/api.html#nvim_set_option_value()).
///
/// Sets the value of an option. The behaviour of this function matches that of
/// `:set`: for global-local options, both the global and local value are set
/// unless specified otherwise in the [`scope`](OptionValueOptsBuilder::scope)
/// field of `opts`.
pub fn set_option_value<Opt>(
    name: &str,
    value: Opt,
    opts: Option<&OptionValueOpts>,
) -> Result<()>
where
    Opt: ToObject,
{
    let name = nvim::String::from(name);
    let opts = opts.map(KeyDict_option::from).unwrap_or_default();
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_option_value(
            name.non_owning(),
            value.to_obj()?.non_owning(),
            &opts.into(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_var`](https://neovim.io/doc/user/api.html#nvim_set_var()).
///
/// Sets a global (`g:`) variable.
pub fn set_var<Var>(name: &str, value: Var) -> Result<()>
where
    Var: ToObject,
{
    let name = nvim::String::from(name);
    let value = value.to_obj()?;
    let mut err = nvim::Error::new();
    unsafe { nvim_set_var(name.non_owning(), value.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_set_vvar`](https://neovim.io/doc/user/api.html#nvim_set_vvar()).
///
/// Sets a `v:` variable, if it's not readonly.
pub fn set_vvar<Var>(name: &str, value: Var) -> Result<()>
where
    Var: ToObject,
{
    let name = nvim::String::from(name);
    let value = value.to_obj()?;
    let mut err = nvim::Error::new();
    unsafe { nvim_set_vvar(name.non_owning(), value.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_strwidth`](https://neovim.io/doc/user/api.html#nvim_strwidth()).
///
/// Calculates the number of display cells occupied by `text`. Control
/// characters like `<Tab>` count as one cell.
pub fn strwidth(text: &str) -> Result<usize> {
    let text = nvim::String::from(text);
    let mut err = nvim::Error::new();
    let width = unsafe { nvim_strwidth(text.non_owning(), &mut err) };
    err.into_err_or_else(|| width.try_into().expect("always positive"))
}
