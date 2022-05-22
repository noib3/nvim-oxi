use std::ffi::CString;

use nvim_types::{
    Array,
    BufHandle,
    Dictionary,
    Error as NvimError,
    NvimString,
};

use super::buffer::Buffer;
use crate::Result;

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1057
    fn nvim_create_buf(
        listed: bool,
        scratch: bool,
        err: *mut NvimError,
    ) -> BufHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L889
    fn nvim_echo(
        chunks: Array,
        history: bool,
        opts: Dictionary,
        err: *mut NvimError,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L963
    fn nvim_get_current_buf() -> BufHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1578
    fn nvim_get_mode() -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L398
    fn nvim_replace_termcodes(
        str: NvimString,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> NvimString;
}

// chan_send

/// Binding to `vim.api.nvim_create_buf`.
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = NvimError::default();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    err.into_err_or_else(|| Buffer::from(handle))
}

// create_user_command

// del_current_line

// del_keymap

// del_mark

// del_user_command

// del_var

/// Binding to `vim.api.nvim_echo`.
pub fn echo<Text, HlGroup, Chunks>(chunks: Chunks, history: bool) -> Result<()>
where
    Text: std::fmt::Display,
    HlGroup: AsRef<str>,
    Chunks: IntoIterator<Item = (Text, Option<HlGroup>)>,
{
    let chunks = chunks
        .into_iter()
        .map(|(text, maybe_hlgroup)| {
            let text = text.to_string();
            match maybe_hlgroup {
                Some(group) => vec![text, group.as_ref().to_owned()],
                None => vec![text],
            }
        })
        .collect();

    let mut err = NvimError::default();
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

/// Binding to `vim.api.nvim_get_current_buf`.
pub fn get_current_buf() -> Buffer {
    Buffer::from(unsafe { nvim_get_current_buf() })
}

// get_current_line

// get_current_tabpage

// get_current_win

// get_hl_by_id

// get_hl_by_name

// get_hl_id_by_name

// get_keymap

// get_mark

/// Binding to `vim.api.nvim_get_mode`.
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

// get_var

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

/// Binding to `vim.api.nvim_replace_termcodes`.
pub fn replace_termcodes(
    str: &str,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> CString {
    let str = NvimString::from(str);

    unsafe { nvim_replace_termcodes(str, from_part, do_lt, special) }
        .as_c_str()
        .to_owned()
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
