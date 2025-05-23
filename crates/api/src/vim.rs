use std::path::{Path, PathBuf};

use types::NvimStr;
use types::{
    self as nvim,
    Array,
    Dictionary,
    Integer,
    Object,
    conversion::{FromObject, ToObject},
};

use crate::LUA_INTERNAL_CALL;
use crate::SuperIterator;
use crate::choose;
use crate::ffi::vim::*;
use crate::opts::*;
use crate::types::*;
use crate::{Buffer, TabPage, Window};
use crate::{Error, Result};

/// Binding to [`nvim_chan_send()`][1].
///
/// Sends data to a channel.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_chan_send()
pub fn chan_send(channel_id: u32, data: &str) -> Result<()> {
    let mut err = nvim::Error::new();
    let data = nvim::String::from(data);
    unsafe { nvim_chan_send(channel_id.into(), data.as_nvim_str(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_create_buf()`][1].
///
/// Creates a new, empty, unnamed buffer.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_create_buf()
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = nvim::Error::new();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    choose!(err, Ok(handle.into()))
}

/// Binding to [`nvim_del_current_line()`][1].
///
/// Deletes the current line.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_current_line()
pub fn del_current_line() -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_del_current_line(types::arena(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_del_keymap()`][1].
///
/// Unmaps a global mapping for the given mode. To unmap a buffer-local mapping
/// use [`Buffer::del_keymap`] instead.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_keymap()
pub fn del_keymap(mode: Mode, lhs: &str) -> Result<()> {
    let mode = nvim::String::from(mode);
    let lhs = nvim::String::from(lhs);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_del_keymap(
            LUA_INTERNAL_CALL,
            mode.as_nvim_str(),
            lhs.as_nvim_str(),
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_del_mark()`][1].
///
/// Deletes an uppercase/file named mark. Returns an error if a lowercase or
/// buffer-local named mark is used. Use [`Buffer::del_mark`] to delete a
/// buffer-local mark.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_mark()
pub fn del_mark(name: char) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let was_deleted = unsafe { nvim_del_mark(name.as_nvim_str(), &mut err) };
    choose!(
        err,
        match was_deleted {
            true => Ok(()),
            _ => Err(Error::custom("Couldn't delete mark")),
        }
    )
}

/// Binding to [`nvim_del_var()`][1].
///
/// Removes a global (`g:`) variable.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_var()
pub fn del_var(name: &str) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe { nvim_del_var(name.as_nvim_str(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_echo()`][1].
///
/// Echoes a message to the Neovim message area.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_echo()
pub fn echo<HlGroup, Text, Chunks>(
    chunks: Chunks,
    history: bool,
    opts: &EchoOpts,
) -> Result<()>
where
    Chunks: IntoIterator<Item = (Text, Option<HlGroup>)>,
    Text: Into<nvim::String>,
    HlGroup: Into<nvim::String>,
{
    let chunks = chunks
        .into_iter()
        .map(|(text, hlgroup)| {
            Array::from_iter([
                Object::from(text.into()),
                Object::from(hlgroup.map(Into::into)),
            ])
        })
        .collect::<Array>();

    let mut err = nvim::Error::new();
    unsafe { nvim_echo(chunks.non_owning(), history, opts, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_err_write()`][1].
///
/// Writes a message to the Neovim error buffer. Does not append a newline
/// (`"\n"`); the message gets buffered and won't be displayed until a linefeed
/// is written.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_err_write()
pub fn err_write(str: &str) {
    unsafe { nvim_err_write(nvim::String::from(str).as_nvim_str()) }
}

/// Binding to [`nvim_err_writeln()`][1].
///
/// Writes a message to the Neovim error buffer. Appends a newline (`"\n"`), so
/// the buffer is flushed and displayed.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_err_writeln()
pub fn err_writeln(str: &str) {
    unsafe { nvim_err_writeln(nvim::String::from(str).as_nvim_str()) }
}

/// Binding to [`nvim_eval_statusline()`][1].
///
/// Evaluates a string to be displayed in the statusline.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_eval_statusline()
pub fn eval_statusline(
    str: &str,
    opts: &EvalStatuslineOpts,
) -> Result<StatuslineInfos> {
    let str = nvim::String::from(str);
    let mut err = nvim::Error::new();
    let dict = unsafe {
        nvim_eval_statusline(str.as_nvim_str(), opts, types::arena(), &mut err)
    };
    choose!(err, Ok(StatuslineInfos::from_object(dict.into())?))
}

/// Binding to [`nvim_feedkeys()`][1].
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_feedkeys()
pub fn feedkeys<'a>(
    keys: impl Into<NvimStr<'a>>,
    mode: impl Into<NvimStr<'a>>,
    escape_ks: bool,
) {
    unsafe { nvim_feedkeys(keys.into(), mode.into(), escape_ks) }
}

/// Binding to [`nvim_get_chan_info()`][1].
///
/// Gets information about a channel.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_chan_info()
pub fn get_chan_info(channel_id: u32) -> Result<ChannelInfos> {
    let mut err = nvim::Error::new();
    let infos = unsafe { nvim_get_chan_info(channel_id.into(), &mut err) };
    choose!(err, Ok(ChannelInfos::from_object(infos.into())?))
}

/// Binding to [`nvim_get_color_by_name()`][1].
///
/// Returns the 24-bit RGB value of a `crate::api::get_color_map` color name or
/// "#rrggbb" hexadecimal string.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_color_by_name()
pub fn get_color_by_name(name: &str) -> Result<u32> {
    let name = nvim::String::from(name);
    let color = unsafe { nvim_get_color_by_name(name.as_nvim_str()) };
    (color != -1).then(|| color.try_into().unwrap()).ok_or_else(|| {
        Error::custom(format!("{name:?} is not a valid color name"))
    })
}

/// Binding to [`nvim_get_color_map()`][1].
///
/// Returns an iterator over tuples representing color names and 24-bit RGB
/// values (e.g. 65535).
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_color_map()
pub fn get_color_map() -> impl SuperIterator<(String, u32)> {
    unsafe { nvim_get_color_map(types::arena()) }.into_iter().map(|(k, v)| {
        (k.to_string_lossy().into(), u32::from_object(v).unwrap())
    })
}

/// Binding to [`nvim_get_context()`][1].
///
/// Returns a snapshot of the current editor state.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_context()
pub fn get_context(opts: &GetContextOpts) -> Result<EditorContext> {
    let mut err = nvim::Error::new();
    let ctx = unsafe { nvim_get_context(opts, types::arena(), &mut err) };
    choose!(err, Ok(EditorContext::from_object(ctx.into())?))
}

/// Binding to [`nvim_get_current_buf()`][1].
///
/// Gets the current buffer.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_buf()
pub fn get_current_buf() -> Buffer {
    unsafe { nvim_get_current_buf() }.into()
}

/// Binding to [`nvim_get_current_line()`][1].
///
/// Gets the current line in the current bufferr.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_line()
pub fn get_current_line() -> Result<String> {
    let mut err = nvim::Error::new();
    let s = unsafe { nvim_get_current_line(types::arena(), &mut err) };
    choose!(err, Ok(s.to_string_lossy().into()))
}

/// Binding to [`nvim_get_current_tabpage()`][1].
///
/// Gets the current tabpage.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_tabpage()
pub fn get_current_tabpage() -> TabPage {
    unsafe { nvim_get_current_tabpage() }.into()
}

/// Binding to [`nvim_get_current_win()`][1].
///
/// Gets the current window.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_win()
pub fn get_current_win() -> Window {
    unsafe { nvim_get_current_win() }.into()
}

/// Binding to [`nvim_get_hl()`][1].
///
/// Gets all or specific highlight groups in a namespace.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_hl()
pub fn get_hl(
    ns_id: u32,
    opts: &GetHighlightOpts,
) -> Result<
    GetHlInfos<impl SuperIterator<(types::String, HighlightInfos)> + use<>>,
> {
    let mut err = nvim::Error::new();
    let dict = unsafe {
        nvim_get_hl(ns_id as Integer, opts, types::arena(), &mut err)
    };
    if err.is_err() {
        return Err(err.into());
    }

    let is_map = dict
        .iter()
        .next()
        .map(|(_, hl_infos)| {
            let d = types::serde::Deserializer::new(hl_infos.clone());
            <HighlightInfos as serde::Deserialize>::deserialize(d).is_ok()
        })
        .unwrap_or(false);

    if is_map {
        let iter = dict.into_iter().map(|(hl_name, hl_infos)| {
            let infos = HighlightInfos::from_object(hl_infos).unwrap();
            (hl_name, infos)
        });
        Ok(GetHlInfos::Map(iter))
    } else {
        HighlightInfos::from_object(dict.into())
            .map(GetHlInfos::Single)
            .map_err(Into::into)
    }
}

/// Binding to [`nvim_get_hl_id_by_name()`][1].
///
/// Gets a highlight definition by name.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_hl_id_by_name()
pub fn get_hl_id_by_name(name: &str) -> Result<u32> {
    let name = nvim::String::from(name);
    let id = unsafe { nvim_get_hl_id_by_name(name.as_nvim_str()) };
    id.try_into().map_err(Into::into)
}

/// Binding to [`nvim_get_hl_ns()`][1].
///
/// Gets the active highlight namespace.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_hl_ns()
pub fn get_hl_ns(opts: &GetNamespaceOpts) -> Result<i64> {
    let mut err = nvim::Error::new();
    let res = unsafe { nvim_get_hl_ns(opts, &mut err) };
    choose!(err, Ok(res))
}

/// Binding to [`nvim_get_keymap()`][1].
///
/// Returns an iterator over the global mapping definitions.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_keymap()
pub fn get_keymap(mode: Mode) -> impl SuperIterator<KeymapInfos> {
    let mode = nvim::String::from(mode);
    let keymaps =
        unsafe { nvim_get_keymap(mode.as_nvim_str(), types::arena()) };
    keymaps.into_iter().map(|obj| KeymapInfos::from_object(obj).unwrap())
}

/// Binding to [`nvim_get_mark()`][1].
///
/// Returns a tuple `(row, col, buffer, buffername)` representing the position
/// of the named mark. Marks are (1,0)-indexed.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_mark()
pub fn get_mark(
    name: char,
    opts: &GetMarkOpts,
) -> Result<(usize, usize, Buffer, String)> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let mark = unsafe {
        nvim_get_mark(name.as_nvim_str(), opts, types::arena(), &mut err)
    };
    choose!(err, {
        let mut iter = mark.into_iter();
        let row = usize::from_object(iter.next().expect("row is present"))?;
        let col = usize::from_object(iter.next().expect("col is present"))?;
        let buffer =
            Buffer::from_object(iter.next().expect("buffer is present"))?;
        let buffername =
            String::from_object(iter.next().expect("buffername is present"))?;
        Ok((row, col, buffer, buffername))
    })
}

/// Binding to [`nvim_get_mode()`][1].
///
/// Gets the current mode. The [`blocking`](GotMode::blocking) field of
/// [`GotMode`] is `true` if Neovim is waiting for input.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_mode()
pub fn get_mode() -> Result<GotMode> {
    unsafe { nvim_get_mode(types::arena()) }.try_into().map_err(Into::into)
}

/// Binding to [`nvim_get_proc()`][1].
///
/// Gets informations about a process with a given `pid`.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_proc()
pub fn get_proc(pid: u32) -> Result<ProcInfos> {
    let mut err = nvim::Error::new();
    let obj = unsafe { nvim_get_proc(pid.into(), types::arena(), &mut err) };
    choose!(err, Ok(ProcInfos::from_object(obj)?))
}

/// Binding to [`nvim_get_proc_children()`][1].
///
/// Gets the immediate children of process `pid`.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_proc_children()
pub fn get_proc_children(pid: u32) -> Result<impl SuperIterator<u32>> {
    let mut err = nvim::Error::new();
    let procs = unsafe {
        nvim_get_proc_children(pid.into(), types::arena(), &mut err)
    };
    choose!(
        err,
        Ok(procs.into_iter().map(|obj| u32::from_object(obj).unwrap()))
    )
}

/// Binding to [`nvim_get_runtime_file()`][1].
///
/// Returns an iterator over all the files matching `name` in the runtime path.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_runtime_file()
pub fn get_runtime_file(
    name: impl AsRef<Path>,
    get_all: bool,
) -> Result<impl SuperIterator<PathBuf>> {
    let name = nvim::String::from(name.as_ref());
    let mut err = nvim::Error::new();
    let files = unsafe {
        nvim_get_runtime_file(
            name.as_nvim_str(),
            get_all,
            types::arena(),
            &mut err,
        )
    };
    choose!(
        err,
        Ok({
            files.into_iter().map(|obj| {
                PathBuf::from(nvim::String::from_object(obj).unwrap())
            })
        })
    )
}

/// Binding to [`nvim_get_var()`][1].
///
/// Gets a global (`g:`) variable.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_var()
pub fn get_var<Var>(name: &str) -> Result<Var>
where
    Var: FromObject,
{
    let mut err = nvim::Error::new();
    let name = nvim::String::from(name);
    let obj =
        unsafe { nvim_get_var(name.as_nvim_str(), types::arena(), &mut err) };
    choose!(err, Ok(Var::from_object(obj)?))
}

/// Binding to [`nvim_get_vvar()`][1].
///
/// Gets a `v:` variable.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_vvar()
pub fn get_vvar<Var>(name: &str) -> Result<Var>
where
    Var: FromObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let obj =
        unsafe { nvim_get_vvar(name.as_nvim_str(), types::arena(), &mut err) };
    choose!(err, Ok(Var::from_object(obj)?))
}

/// Binding to [`nvim_input()`][1].
///
/// Queues raw user-input. Unlike [`api::feedkeys`](feedkeys) this uses a
/// low-level input buffer and the call is non-blocking.
///
/// Returns the number of bytes written to the buffer.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_input()
pub fn input<Input>(keys: Input) -> Result<usize>
where
    Input: Into<nvim::String>,
{
    unsafe {
        nvim_input(
            #[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
            LUA_INTERNAL_CALL,
            keys.into().as_nvim_str(),
        )
    }
    .try_into()
    .map_err(From::from)
}

/// Binding to [`nvim_input_mouse()`][1].
///
/// Send mouse event from GUI. The call is non-blocking.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_input_mouse()
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
            button.as_nvim_str(),
            action.as_nvim_str(),
            modifier.as_nvim_str(),
            grid.into(),
            row.try_into()?,
            col.try_into()?,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_list_bufs()`][1].
///
/// Gets the current list of [`Buffer`]s, including [unlisted][2]
/// buffers (like `:ls!`). Use [`Buffer::is_loaded`] to check if a
/// buffer is loaded.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_bufs()
/// [2]: unloaded/deleted
pub fn list_bufs() -> impl SuperIterator<Buffer> {
    let bufs = unsafe { nvim_list_bufs(types::arena()) };
    bufs.into_iter().map(|obj| Buffer::from_object(obj).unwrap())
}

/// Binding to [`nvim_list_chans()`][1].
///
/// Returns an iterator over the informations about all the open channels.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_chans()
pub fn list_chans() -> impl SuperIterator<ChannelInfos> {
    unsafe { nvim_list_chans() }
        .into_iter()
        .map(|obj| ChannelInfos::from_object(obj).unwrap())
}

/// Binding to [`nvim_list_runtime_paths()`][1].
///
/// Gets the paths contained in https://neovim's runtimepath.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_runtime_paths()
pub fn list_runtime_paths() -> Result<impl SuperIterator<PathBuf>> {
    let mut err = nvim::Error::new();
    let paths = unsafe { nvim_list_runtime_paths(types::arena(), &mut err) };
    choose!(
        err,
        Ok({
            paths.into_iter().map(|obj| {
                PathBuf::from(nvim::String::from_object(obj).unwrap())
            })
        })
    )
}

/// Binding to [`nvim_list_bufs()`][1].
///
/// Gets the current list of `Tabpage`s.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_bufs()
pub fn list_tabpages() -> impl SuperIterator<TabPage> {
    unsafe { nvim_list_tabpages() }
        .into_iter()
        .map(|obj| TabPage::from_object(obj).unwrap())
}

/// Binding to [`nvim_list_uis()`][1].
///
/// Returns an iterator over the informations about all the attached UIs.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_uis()
pub fn list_uis() -> impl SuperIterator<UiInfos> {
    unsafe { nvim_list_uis(types::arena()) }
        .into_iter()
        .map(|obj| UiInfos::from_object(obj).unwrap())
}

/// Binding to [`nvim_list_wins()`][1].
///
/// Gets the current list of `Window`s.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_wins()
pub fn list_wins() -> impl SuperIterator<Window> {
    unsafe { nvim_list_wins(types::arena()) }
        .into_iter()
        .map(|obj| Window::from_object(obj).unwrap())
}

/// Binding to [`nvim_load_context()`][1].
///
/// Sets the current editor state from the given [`EditorContext`].
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_load_context()
pub fn load_context(ctx: EditorContext) {
    let ctx = Dictionary::from(ctx);
    let _ = unsafe { nvim_load_context(ctx.non_owning()) };
}

/// Binding to [`nvim_notify()`][1].
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_notify()
pub fn notify(
    msg: &str,
    log_level: LogLevel,
    opts: &Dictionary,
) -> Result<Object> {
    let msg = nvim::String::from(msg);
    let mut err = nvim::Error::new();
    let obj = unsafe {
        nvim_notify(
            msg.as_nvim_str(),
            log_level as Integer,
            opts.non_owning(),
            types::arena(),
            &mut err,
        )
    };
    choose!(err, Ok(obj))
}

/// Binding to [`nvim_open_term()`][1].
///
/// Opens a terminal instance in a buffer. Returns the id of a channel that can
/// be used to send data to the instance via
/// [`nvim_oxi::api::chan_send`](chan_send).
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_open_term()
pub fn open_term(buffer: &Buffer, opts: &OpenTermOpts) -> Result<u32> {
    let mut err = nvim::Error::new();
    let channel_id = unsafe { nvim_open_term(buffer.0, opts, &mut err) };
    choose!(
        err,
        match channel_id {
            0 => Err(Error::custom("Couldn't create terminal instance")),
            other => Ok(other.try_into().expect("always positive")),
        }
    )
}

/// Binding to [`nvim_out_write()`][1].
///
/// Writes a message to the Vim output buffer, without appending a "\n". The
/// message is buffered and won't be displayed until a linefeed is written.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_out_write()
pub fn out_write<Msg>(str: Msg)
where
    Msg: Into<nvim::String>,
{
    unsafe { nvim_out_write(str.into().as_nvim_str()) }
}

/// Binding to [`nvim_paste()`][1].
///
/// Returns `true` if the client may continue the paste, `false` if it must
/// cancel it.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_paste()
pub fn paste<Data>(data: Data, crlf: bool, phase: PastePhase) -> Result<bool>
where
    Data: Into<nvim::String>,
{
    let mut err = nvim::Error::new();
    let go_on = unsafe {
        nvim_paste(
            data.into().as_nvim_str(),
            crlf,
            phase as Integer,
            types::arena(),
            &mut err,
        )
    };
    choose!(err, Ok(go_on))
}

/// Binding to [`nvim_put()`][1].
///
/// Puts text at cursor, in any mode.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_put()
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
            reg_type.as_nvim_str(),
            after,
            follow,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_replace_termcodes()`][1].
///
/// Replaces terminal codes and keycodes (`<CR>`, `<Esc>`, ...) in a string
/// with the internal representation.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_replace_termcodes()
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
        nvim_replace_termcodes(str.as_nvim_str(), from_part, do_lt, special)
    }
}

/// Binding to [`nvim_select_popupmenu_item()`][1].
///
/// Selects an item in the completion popupmenu.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_select_popupmenu_item()
pub fn select_popupmenu_item(
    item: usize,
    insert: bool,
    finish: bool,
    opts: &SelectPopupMenuItemOpts,
) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe {
        nvim_select_popupmenu_item(
            item.try_into()?,
            insert,
            finish,
            opts,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_buf()`][1].
///
/// Sets the current buffer.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_buf()
pub fn set_current_buf(buf: &Buffer) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_buf(buf.0, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_dir()`][1].
///
/// Changes the global working directory.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_dir()
pub fn set_current_dir<Dir>(dir: Dir) -> Result<()>
where
    Dir: AsRef<Path>,
{
    let dir = nvim::String::from(dir.as_ref());
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_dir(dir.as_nvim_str(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_line()`][1].
///
/// Sets the current line.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_line()
pub fn set_current_line<Line>(line: Line) -> Result<()>
where
    Line: Into<nvim::String>,
{
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_current_line(
            line.into().as_nvim_str(),
            types::arena(),
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_tabpage()`][1].
///
/// Sets the current tabpage.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_tabpage()
pub fn set_current_tabpage(tabpage: &TabPage) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_tabpage(tabpage.0, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_win()`][1].
///
/// Sets the current window.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_win()
pub fn set_current_win(win: &Window) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_win(win.0, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_hl()`][1].
///
/// Sets a highlight group.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_hl()
pub fn set_hl(ns_id: u32, name: &str, opts: &SetHighlightOpts) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_hl(
            LUA_INTERNAL_CALL,
            ns_id as Integer,
            name.as_nvim_str(),
            opts,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_set_hl_ns()`][1].
///
/// Set the active namespace for the highlights defined with [`set_hl`]. This
/// can be set for a single window, see [`Window::set_hl`].
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_hl_ns()
pub fn set_hl_ns(ns_id: u32) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_hl_ns(ns_id as Integer, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_hl_ns_fast()`][1].
///
/// Set the active namespace for the highlights defined with [`set_hl`] while
/// redrawing.
///
/// This function is meant to be called while redrawing, primarily from
/// [`set_decoration_provider`](crate::set_decoration_provider)'s `on_win` and
/// `on_lines` callbacks, which are allowed to change the namespace during a
/// redraw cycle.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_hl_ns_fast()
pub fn set_hl_ns_fast(ns_id: u32) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_hl_ns_fast(ns_id as Integer, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_keymap()`][1].
///
/// Sets a global mapping for the given mode. To set a buffer-local mapping use
/// [`Buffer::set_keymap`] instead.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_keymap()
pub fn set_keymap(
    mode: Mode,
    lhs: &str,
    rhs: &str,
    opts: &SetKeymapOpts,
) -> Result<()> {
    let mode = nvim::String::from(mode);
    let lhs = nvim::String::from(lhs);
    let rhs = nvim::String::from(rhs);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_keymap(
            LUA_INTERNAL_CALL,
            mode.as_nvim_str(),
            lhs.as_nvim_str(),
            rhs.as_nvim_str(),
            opts,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_set_var()`][1].
///
/// Sets a global (`g:`) variable.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_var()
pub fn set_var<Var>(name: &str, value: Var) -> Result<()>
where
    Var: ToObject,
{
    let name = nvim::String::from(name);
    let value = value.to_object()?;
    let mut err = nvim::Error::new();
    unsafe { nvim_set_var(name.as_nvim_str(), value.non_owning(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_vvar()`][1].
///
/// Sets a `v:` variable, if it's not readonly.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_vvar()
pub fn set_vvar<Var>(name: &str, value: Var) -> Result<()>
where
    Var: ToObject,
{
    let name = nvim::String::from(name);
    let value = value.to_object()?;
    let mut err = nvim::Error::new();
    unsafe { nvim_set_vvar(name.as_nvim_str(), value.non_owning(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_strwidth()`][1].
///
/// Calculates the number of display cells occupied by `text`. Control
/// characters like `<Tab>` count as one cell.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_strwidth()
pub fn strwidth(text: &str) -> Result<usize> {
    let text = nvim::String::from(text);
    let mut err = nvim::Error::new();
    let width = unsafe { nvim_strwidth(text.as_nvim_str(), &mut err) };
    choose!(err, Ok(width.try_into().expect("always positive")))
}
