use std::path::PathBuf;

use nvim_types::{
    Array,
    Dictionary,
    Error as NvimError,
    Integer,
    Object,
    String as NvimString,
    TabHandle,
    WinHandle,
};

use super::ffi::global::*;
use super::opts::*;
use super::types::*;
use crate::{
    api::Buffer,
    lua::LUA_INTERNAL_CALL,
    object::{FromObject, ToObject},
    Result,
};

/// Binding to `nvim_chan_send`.
pub fn chan_send(chan: impl Into<Integer>, data: &str) -> Result<()> {
    let mut err = NvimError::new();
    let data = NvimString::from(data);
    unsafe { nvim_chan_send(chan.into(), data.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_create_buf`.
///
/// Creates a new, empty, unnamed buffer.
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = NvimError::new();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    err.into_err_or_else(|| handle.into())
}

/// Binding to `nvim_create_user_command`.
pub fn create_user_command<Value>(
    name: &str,
    command: Value,
    opts: &CreateCommandOpts,
) -> Result<()>
where
    Value: ToObject,
{
    let name = NvimString::from(name);
    let command = command.to_obj()?;
    let mut err = NvimError::new();
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

/// Binding to `nvim_del_current_line`
pub fn del_current_line() -> Result<()> {
    let mut err = NvimError::new();
    unsafe { nvim_del_current_line(&mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_keymap`.
pub fn del_keymap(mode: Mode, lhs: &str) -> Result<()> {
    let mode = NvimString::from(mode);
    let lhs = NvimString::from(lhs);
    let mut err = NvimError::new();
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

/// Binding to `nvim_del_mark`.
pub fn del_mark(name: char) -> Result<bool> {
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    let res = unsafe { nvim_del_mark(name.non_owning(), &mut err) };
    err.into_err_or_else(|| res)
}

/// Binding to `nvim_del_user_command`.
pub fn del_user_command(name: &str) -> Result<()> {
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    unsafe { nvim_del_user_command(name.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_var`.
///
/// Removes a global (`g:`) variable.
pub fn del_var(name: &str) -> Result<()> {
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    unsafe { nvim_del_var(name.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_echo`.
///
/// Echoes a message to the Neovim message area.
pub fn echo<Text, HlGroup, Chunks>(chunks: Chunks, history: bool) -> Result<()>
where
    Text: std::fmt::Display,
    HlGroup: AsRef<str>,
    Chunks: IntoIterator<Item = (Text, Option<HlGroup>)>,
{
    let chunks = chunks
        .into_iter()
        .map(|(text, hlgroup)| {
            Array::from_iter([
                Object::from(text.to_string()),
                Object::from(hlgroup.map(|hl| hl.as_ref().to_owned())),
            ])
        })
        .collect::<Array>();

    let mut err = NvimError::new();
    let opts = Dictionary::new();
    unsafe {
        nvim_echo(chunks.non_owning(), history, opts.non_owning(), &mut err)
    };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_err_write`.
///
/// Writes a message to the Neovim error buffer. Does not append a newline
/// (`"\n"`); the message gets buffered and won't be displayed until a linefeed
/// is written.
pub fn err_write(str: &str) {
    unsafe { nvim_err_write(NvimString::from(str).non_owning()) }
}

/// Binding to `nvim_err_writeln`.
///
/// Writes a message to the Neovim error buffer. Appends a newline (`"\n"`), so
/// the buffer is flushed and displayed.
pub fn err_writeln(str: &str) {
    unsafe { nvim_err_writeln(NvimString::from(str).non_owning()) }
}

/// Binding to `nvim_eval_statusline`.
///
/// Evaluates a string to be displayed in the statusline.
pub fn eval_statusline(
    str: &str,
    opts: EvalStatuslineOpts,
) -> Result<StatuslineInfos> {
    let str = NvimString::from(str);
    let mut err = NvimError::new();
    let dict = unsafe {
        nvim_eval_statusline(str.non_owning(), &opts.into(), &mut err)
    };
    err.into_err_or_flatten(|| StatuslineInfos::from_obj(dict.into()))
}

/// Binding to `nvim_feedkeys`.
pub fn feedkeys(keys: &str, mode: Mode, escape_ks: bool) {
    let keys = NvimString::from(keys);
    let mode = NvimString::from(mode);
    unsafe { nvim_feedkeys(keys.non_owning(), mode.non_owning(), escape_ks) }
}

/// Binding to `nvim_get_all_options_info`.
///
/// Gets the option information for all options.
pub fn get_all_options_info() -> Result<impl Iterator<Item = OptionInfos>> {
    let mut err = NvimError::new();
    let infos = unsafe { nvim_get_all_options_info(&mut err) };
    err.into_err_or_else(|| {
        infos.into_iter().flat_map(|(_, optinf)| OptionInfos::from_obj(optinf))
    })
}

/// Binding to `nvim_get_chan_info`.
///
/// Gets information about a channel.
pub fn get_chan_info(chan: impl Into<Integer>) -> Result<ChannelInfos> {
    let mut err = NvimError::new();
    let infos = unsafe { nvim_get_chan_info(chan.into(), &mut err) };
    err.into_err_or_flatten(|| ChannelInfos::from_obj(infos.into()))
}

/// Binding to `nvim_get_color_by_name`.
///
/// Returns the 24-bit RGB value of a `crate::api::get_color_map` color name or
/// "#rrggbb" hexadecimal string.
pub fn get_color_by_name(name: &str) -> u32 {
    let name = NvimString::from(name);
    let color = unsafe { nvim_get_color_by_name(name.non_owning()) };
    // TODO: don't panic
    color.try_into().expect("invalid argument")
}

/// Binding to `nvim_get_color_map`.
///
/// Returns an iterator over tuples representing color names and 24-bit RGB
/// values (e.g. 65535).
pub fn get_color_map() -> impl Iterator<Item = (String, u32)> {
    unsafe { nvim_get_color_map() }.into_iter().map(|(k, v)| {
        (String::try_from(k).unwrap(), u32::try_from(v).unwrap())
    })
}

/// Binding to `nvim_get_commands`.
///
/// Returns an iterator over the infos of the global ex commands. Only
/// user-defined commands are returned, not builtin ones.
pub fn get_commands(
    opts: GetCommandsOpts,
) -> Result<impl Iterator<Item = CommandInfos>> {
    let mut err = NvimError::new();
    let cmds = unsafe { nvim_get_commands(&opts.into(), &mut err) };
    err.into_err_or_else(|| {
        cmds.into_iter().flat_map(|(_, cmd)| CommandInfos::from_obj(cmd))
    })
}

/// Binding to `nvim_get_context`.
///
/// Returns a snapshot of the current editor state.
pub fn get_context(opts: GetContextOpts) -> Result<EditorContext> {
    let mut err = NvimError::new();
    let ctx = unsafe { nvim_get_context(&opts.into(), &mut err) };
    err.into_err_or_flatten(|| EditorContext::from_obj(ctx.into()))
}

/// Binding to `nvim_get_current_buf`.
///
/// Gets the current buffer.
pub fn get_current_buf() -> Buffer {
    unsafe { nvim_get_current_buf() }.into()
}

/// Binding to `nvim_get_current_line`.
///
/// Gets the current line in the current bufferr.
pub fn get_current_line() -> Result<String> {
    let mut err = NvimError::new();
    let str = unsafe { nvim_get_current_line(&mut err) };
    err.into_err_or_flatten(|| str.try_into().map_err(crate::Error::from))
}

/// Binding to `nvim_get_current_tabpage`.
///
/// Gets the current tabpage.
pub fn get_current_tabpage() -> TabHandle {
    // TODO: return `Tab` once that's implemented.
    unsafe { nvim_get_current_tabpage() }
}

/// Binding to `nvim_get_current_win`.
///
/// Gets the current window.
pub fn get_current_win() -> WinHandle {
    // TODO: return `Window` once that's implemented.
    unsafe { nvim_get_current_win() }
}

/// Binding to `nvim_get_hl_by_id`.
///
/// Gets a highlight definition by id.
pub fn get_hl_by_id(hl_id: impl Into<Integer>, rgb: bool) -> Result<HlAttrs> {
    let mut err = NvimError::new();
    let hl = unsafe { nvim_get_hl_by_id(hl_id.into(), rgb, &mut err) };
    err.into_err_or_flatten(|| HlAttrs::from_obj(hl.into()))
}

/// Binding to `nvim_get_hl_by_name`.
///
/// Gets a highlight definition by name.
pub fn get_hl_by_name(name: &str, rgb: bool) -> Result<HlAttrs> {
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    let hl = unsafe { nvim_get_hl_by_name(name.non_owning(), rgb, &mut err) };
    err.into_err_or_flatten(|| HlAttrs::from_obj(hl.into()))
}

/// Binding to `nvim_get_hl_id_by_name`.
///
/// Gets a highlight definition by name.
pub fn get_hl_id_by_name(name: &str) -> Result<u32> {
    let name = NvimString::from(name);
    let id = unsafe { nvim_get_hl_id_by_name(name.non_owning()) };
    id.try_into().map_err(crate::Error::from)
}

/// Binding to `nvim_get_keymap`.
///
/// Returns an iterator over the global mapping definitions.
pub fn get_keymap(mode: Mode) -> impl Iterator<Item = KeymapInfos> {
    let mode = NvimString::from(mode);
    unsafe { nvim_get_keymap(LUA_INTERNAL_CALL, mode.non_owning()) }
        .into_iter()
        .flat_map(KeymapInfos::from_obj)
}

/// Binding to `nvim_get_mark`.
///
/// Returns a tuple `(row, col, buffer, buffername)` representing the position
/// of the named mark. Marks are (1,0)-indexed.
pub fn get_mark(
    name: char,
    opts: GetMarkOpts,
) -> Result<(usize, usize, Buffer, String)> {
    let name = NvimString::from(name);
    let opts = Dictionary::from(opts);
    let mut err = NvimError::new();
    let mark = unsafe {
        nvim_get_mark(name.non_owning(), opts.non_owning(), &mut err)
    };
    err.into_err_or_flatten(|| {
        let mut iter = mark.into_iter();
        let row = iter.next().expect("row is present").try_into()?;
        let col = iter.next().expect("col is present").try_into()?;
        let buffer: i32 =
            iter.next().expect("buffer is present").try_into()?;
        let buffername =
            iter.next().expect("buffername is present").try_into()?;
        Ok((row, col, buffer.into(), buffername))
    })
}

/// Binding to `nvim_get_mode`.
///
/// Gets the current mode. The `blocking` field of `GotMode` is `true` if
/// Neovim is waiting for input.
pub fn get_mode() -> Result<GotMode> {
    GotMode::from_obj(unsafe { nvim_get_mode() }.into())
}

/// Binding to `nvim_get_option`.
///
/// Gets the value of a global option.
pub fn get_option<V: FromObject>(name: impl Into<NvimString>) -> Result<V> {
    let mut err = NvimError::new();
    let obj = unsafe { nvim_get_option(name.into().non_owning(), &mut err) };
    err.into_err_or_flatten(|| V::from_obj(obj))
}

/// Binding to `nvim_get_option_info`.
///
/// Gets all the informations related to an option.
pub fn get_option_info(name: impl Into<NvimString>) -> Result<OptionInfos> {
    let mut err = NvimError::new();
    let obj = unsafe { nvim_get_option(name.into().non_owning(), &mut err) };
    err.into_err_or_flatten(|| OptionInfos::from_obj(obj))
}

/// Binding to `nvim_get_option_value`.
///
/// Gets the local value of an option if it exists, or the global value
/// otherwise. Local values always correspond to the current buffer or window.
///
/// To get a buffer-local orr window-local option for a specific buffer of
/// window consider using `Buffer::get_option` or `Window::get_option` instead.
pub fn get_option_value<N, V>(name: N, opts: GetOptionValueOpts) -> Result<V>
where
    V: FromObject,
    N: Into<NvimString>,
{
    let mut err = NvimError::new();
    let obj = unsafe {
        nvim_get_option_value(name.into().non_owning(), &opts.into(), &mut err)
    };
    err.into_err_or_flatten(|| V::from_obj(obj))
}

/// Binding to `nvim_get_proc`.
///
/// Gets informations about a process with a given `pid`.
pub fn get_proc(pid: impl Into<Integer>) -> Result<ProcInfos> {
    let mut err = NvimError::new();
    let obj = unsafe { nvim_get_proc(pid.into(), &mut err) };
    err.into_err_or_flatten(|| ProcInfos::from_obj(obj))
}

/// Binding to `nvim_get_proc_children`.
///
/// Gets the immediate children of process `pid`.
pub fn get_proc_children(
    pid: impl Into<Integer>,
) -> Result<impl Iterator<Item = u32>> {
    let mut err = NvimError::new();
    let procs = unsafe { nvim_get_proc_children(pid.into(), &mut err) };
    err.into_err_or_else(|| procs.into_iter().flat_map(u32::try_from))
}

/// Binding to `nvim_get_runtime_file`.
///
/// Returns an iterator over all the files matching `name` in the runtime path.
pub fn get_runtime_file(
    name: impl Into<NvimString>,
    get_all: bool,
) -> Result<impl Iterator<Item = PathBuf>> {
    let mut err = NvimError::new();
    let files = unsafe {
        nvim_get_runtime_file(name.into().non_owning(), get_all, &mut err)
    };
    err.into_err_or_else(|| {
        files
            .into_iter()
            .flat_map(NvimString::try_from)
            .flat_map(PathBuf::try_from)
    })
}

/// Binding to `nvim_get_var`.
///
/// Gets a global (`g:`) variable.
pub fn get_var<Value>(name: &str) -> Result<Value>
where
    Value: FromObject,
{
    let mut err = NvimError::new();
    let name = NvimString::from(name);
    let obj = unsafe { nvim_get_var(name.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Value::from_obj(obj))
}

/// Binding to `nvim_get_vvar`.
///
/// Gets a `v:` variable.
pub fn get_vvar<Value>(name: &str) -> Result<Value>
where
    Value: FromObject,
{
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    let obj = unsafe { nvim_get_vvar(name.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Value::from_obj(obj))
}

/// Binding to `nvim_input`.
///
/// Queues raw user-input. Unlike `crate::api::nvim_feedkeys` this uses a
/// low-level input buffer and the call is non-blocking.
pub fn input(keys: impl Into<NvimString>) -> Result<usize> {
    unsafe { nvim_input(keys.into().non_owning()) }
        .try_into()
        .map_err(crate::Error::from)
}

/// Binding to `nvim_input_mouse`.
///
/// Send mouse event from GUI. The call is non-blocking.
pub fn input_mouse(
    button: impl Into<NvimString>,
    action: impl Into<NvimString>,
    modifier: impl Into<NvimString>,
    grid: u32,
    row: usize,
    col: usize,
) -> Result<()> {
    let mut err = NvimError::new();
    unsafe {
        nvim_input_mouse(
            button.into().non_owning(),
            action.into().non_owning(),
            modifier.into().non_owning(),
            grid.into(),
            row.try_into()?,
            col.try_into()?,
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

// list_bufs

// list_chans

// list_runtime_paths

// list_tabpages

// list_uis

// list_wins

// load_context

// notify

/// Binding to `nvim_open_term`.
///
/// Opens a terminal instance in a buffer.
pub fn open_term(buffer: Buffer, opts: OpenTermOpts) -> Result<u32> {
    let opts = Dictionary::from(opts);
    let mut err = NvimError::new();
    let chan_id =
        unsafe { nvim_open_term(buffer.0, opts.non_owning(), &mut err) };
    err.into_err_or_else(|| chan_id.try_into().expect("always positive"))
}

/// Binding to `nvim_out_write`.
///
/// Writes a message to the Vim output buffer, without appending a "\n". The
/// message is buffered and won't be displayed until a linefeed is written.
pub fn out_write(str: impl Into<NvimString>) {
    unsafe { nvim_out_write(str.into().non_owning()) }
}

/// Binding to `nvim_paste`.
pub fn paste(
    data: impl Into<NvimString>,
    crlf: bool,
    phase: PastePhase,
) -> Result<bool> {
    let mut err = NvimError::new();
    let go_on = unsafe {
        nvim_paste(data.into().non_owning(), crlf, phase as Integer, &mut err)
    };
    err.into_err_or_else(|| go_on)
}

/// Binding to `nvim_put`.
///
/// Puts text at cursor, in any mode.
pub fn put<Line, Lines>(
    lines: Lines,
    reg_type: RegisterType,
    after: bool,
    follow: bool,
) -> Result<()>
where
    Line: Into<NvimString>,
    Lines: Iterator<Item = Line>,
{
    let lines = lines.into_iter().map(Into::into).collect::<Array>();
    let reg_type = NvimString::from(reg_type);
    let mut err = NvimError::new();
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

/// Binding to `nvim_replace_termcodes`.
///
/// Replaces terminal codes and keycodes (`<CR>`, `<Esc>`, ...) in a string
/// with the internal representation.
pub fn replace_termcodes<Codes: Into<NvimString>>(
    str: Codes,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> NvimString {
    let str = str.into();
    unsafe {
        nvim_replace_termcodes(str.non_owning(), from_part, do_lt, special)
    }
}

// select_popupmenu_item

// set_current_buf

// set_current_dir

// set_current_line

// set_current_tapage

// set_current_win

// set_hl

// set_keymap

// set_option

// set_option_value

/// Binding to `nvim_set_var`.
///
/// Sets a global (`g:`) variable.
pub fn set_var<Value>(name: &str, value: Value) -> Result<()>
where
    Value: ToObject,
{
    let name = NvimString::from(name);
    let value = value.to_obj()?;
    let mut err = NvimError::new();
    unsafe { nvim_set_var(name.non_owning(), value.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_set_vvar`.
///
/// Sets a `v:` variable, if it's not readonly.
pub fn set_vvar<Value>(name: &str, value: Value) -> Result<()>
where
    Value: ToObject,
{
    let name = NvimString::from(name);
    let value = value.to_obj()?;
    let mut err = NvimError::new();
    unsafe { nvim_set_vvar(name.non_owning(), value.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_strwidth`.
///
/// Calculates the number of display cells occupied by `text`. Control
/// characters like `<Tab>` count as one cell.
pub fn strwidth(text: &str) -> Result<usize> {
    let text = NvimString::from(text);
    let mut err = NvimError::new();
    let width = unsafe { nvim_strwidth(text.non_owning(), &mut err) };
    err.into_err_or_else(|| width.try_into().expect("always positive"))
}
