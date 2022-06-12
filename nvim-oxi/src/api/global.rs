use nvim_types::{
    array::Array,
    dictionary::Dictionary,
    error::Error as NvimError,
    object::Object,
    string::String as NvimString,
    Integer,
};

use super::ffi::global::*;
use super::opts::CreateCommandOpts;
use super::types::Mode;
use crate::{
    api::Buffer, Result,
    lua::LUA_INTERNAL_CALL,
    object::{FromObject, ToObject},
};

/// Binding to `nvim_chan_send`
pub fn chan_send<Int>(chan: Int, data: &str) -> Result<()>
where
    Int: Into<Integer>,
{
    let mut err = NvimError::new();
    unsafe { nvim_chan_send(chan.into(), data.into(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_create_buf`.
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = NvimError::new();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    err.into_err_or_else(|| handle.into())
}

/// Binding to `nvim_create_user_command`
pub fn create_user_command<Value>(
    name: &str,
    command: Value,
    opts: &CreateCommandOpts,
) -> Result<()>
where
    Value: ToObject,
{
    let mut err = NvimError::new();
    unsafe {
        nvim_create_user_command(
            name.into(),
            command.to_obj()?,
            &(opts.into()),
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

/// Binding to `nvim_del_keymap`
pub fn del_keymap(mode: Mode, lhs: &str) -> Result<()> {
    let mut err = NvimError::new();
    unsafe {
        nvim_del_keymap(
            LUA_INTERNAL_CALL,
            mode.into(),
            lhs.into(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_mark`
pub fn del_mark(name: &str) -> Result<bool> {
    let mut err = NvimError::new();
    let res = unsafe { nvim_del_mark(name.into(), &mut err) };
    err.into_err_or_else(|| res)
}

/// Binding to `nvim_del_user_command`
pub fn del_user_command(name: &str) -> Result<()> {
    let mut err = NvimError::new();
    unsafe { nvim_del_user_command(name.into(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_var`
pub fn del_var(name: &str) -> Result<()> {
    let mut err = NvimError::new();
    unsafe { nvim_del_var(name.into(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_echo`.
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
    unsafe { nvim_echo(chunks, history, Dictionary::new(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_err_write`
pub fn err_write(string: &str) {
    unsafe { nvim_err_write(string.into()) }
}

/// Binding to `nvim_err_writeln`
pub fn err_writeln(string: &str) {
    unsafe { nvim_err_writeln(string.into()) }
}


// eval_statusline

/// Binding to `nvim_feedkeys`
pub fn feedkeys(keys: &str, mode: &str, escape_ks: bool) {
    unsafe { nvim_feedkeys(keys.into(), mode.into(), escape_ks) }
}

// get_all_options_info

// get_api_info

// get_chan_info

// get_color_by_name

// get_color_map

// get_commands

// get_context

/// Binding to `nvim_get_current_buf`.
pub fn get_current_buf() -> Buffer {
    unsafe { nvim_get_current_buf() }.into()
}

// get_current_line

// get_current_tabpage

// get_current_win

// get_hl_by_id

// get_hl_by_name

// get_hl_id_by_name

// get_keymap

// get_mark

/// Binding to `nvim_get_mode`.
pub fn get_mode() -> Dictionary {
    unsafe { nvim_get_mode() }
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
pub fn get_var<Value>(name: &str) -> Result<Value>
where
    Value: FromObject,
{
    let mut err = NvimError::new();
    let obj = unsafe { nvim_get_var(name.into(), &mut err) };
    err.into_err_or_flatten(|| Value::from_obj(obj))
}

/// Binding to `nvim_get_vvar`
pub fn get_vvar<Value>(name: &str) -> Result<Value>
where
    Value: FromObject,
{
    let mut err = NvimError::new();
    let obj = unsafe { nvim_get_vvar(name.into(), &mut err) };
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
pub fn replace_termcodes<Str: Into<NvimString>>(
    str: Str,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> NvimString {
    unsafe { nvim_replace_termcodes(str.into(), from_part, do_lt, special) }
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

/// Binding to `nvim_set_var`
pub fn set_var<Value>(name: &str, value: Value) -> Result<()>
where
    Value: ToObject,
{
    let mut err = NvimError::new();
    unsafe { nvim_set_var(name.into(), value.to_obj()?, &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_set_vvar`
pub fn set_vvar<Value>(name: &str, value: Value) -> Result<()>
where
    Value: ToObject,
{
    let mut err = NvimError::new();
    unsafe { nvim_set_vvar(name.into(), value.to_obj()?, &mut err) };
    err.into_err_or_else(|| ())
}


// strwidth
