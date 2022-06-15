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

/// Binding to `nvim_feedkeys`
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

// get_hl_by_name

// get_hl_id_by_name

// get_keymap

// get_mark

/// Binding to `nvim_get_mode`.
pub fn get_mode() -> Dictionary {
    unsafe { nvim_get_mode() }
    // TODO
    // (
    //     dict.get("mode").expect("`mode` key is present"),
    //     dict.get("blocking").expect("`blocking` key is present"),
    // )
}

// get_option

// get_option_info

// get_option_value

// get_proc

// get_proc_children

// get_runtime_file

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

// input

// input_mouse

// list_bufs

// list_chans

// list_runtime_paths

// list_tabpages

// list_uis

// list_wins

// load_context

// notify

// open_term

// out_write

// paste

// put

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
