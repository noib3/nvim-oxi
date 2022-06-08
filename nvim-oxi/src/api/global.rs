use nvim_types::{
    array::Array,
    dictionary::Dictionary,
    error::Error as NvimError,
    object::Object,
    string::String as NvimString,
};

use super::ffi::global::*;
use crate::{api::Buffer, Result, object::FromObject};

// chan_send

/// Binding to `nvim_create_buf`.
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = NvimError::new();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    err.into_err_or_else(|| handle.into())
}

// create_user_command

// del_current_line

// del_keymap

// del_mark

// del_user_command

// del_var

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

// err_write

// err_writeln

// eval_statusline

// feedkeys

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

// get_vvar

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

// set_var

// set_vvar

// strwidth
