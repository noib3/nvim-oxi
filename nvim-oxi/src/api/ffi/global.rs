#![allow(dead_code)]

use nvim_types::{
    array::Array,
    dictionary::Dictionary,
    error::Error,
    object::Object,
    string::String,
    BufHandle,
    Integer,
    TabHandle,
    WinHandle,
};

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1231
    pub(crate) fn nvim_chan_send(chan: Integer, data: String, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1057
    pub(crate) fn nvim_create_buf(
        listed: bool,
        scratch: bool,
        err: *mut Error,
    ) -> BufHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L2487
    pub(crate) fn nvim_create_user_command(
        name: String,
        command: Object,
        opts: *const Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L626
    pub(crate) fn nvim_del_current_line(err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1645
    pub(crate) fn nvim_del_keymap(
        channel_id: u64,
        mode: String,
        lhs: String,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L2172
    pub(crate) fn nvim_del_mark(name: String, err: *mut Error) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L2497
    pub(crate) fn nvim_del_user_command(name: String, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L671
    pub(crate) fn nvim_del_var(name: String, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L889
    pub(crate) fn nvim_echo(
        chunks: Array,
        history: bool,
        opts: Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L938
    pub(crate) fn nvim_err_write(str: String);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L949
    pub(crate) fn nvim_err_writeln(str: String);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L2290
    pub(crate) fn nvim_eval_statusline(
        str: String,
        opts: *const Dictionary,
        errr: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L235
    pub(crate) fn nvim_feedkeys(keys: String, mode: String, escape_ks: bool);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L835
    pub(crate) fn nvim_get_all_options_info(err: *mut Error) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1781
    pub(crate) fn nvim_get_chan_info(
        chan: Integer,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1477
    pub(crate) fn nvim_get_color_by_name(name: String) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1489
    pub(crate) fn nvim_get_color_map() -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1589
    pub(crate) fn nvim_get_commands(
        opts: *const Dictionary,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1440
    pub(crate) fn nvim_get_context(
        opts: *const Dictionary,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L963
    pub(crate) fn nvim_get_current_buf() -> BufHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L594
    pub(crate) fn nvim_get_current_line(err: *mut Error) -> String;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1219
    pub(crate) fn nvim_get_current_tabpage() -> TabHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L999
    pub(crate) fn nvim_get_current_win() -> WinHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L95
    pub(crate) fn nvim_get_hl_by_id(
        hl_id: Integer,
        rgb: bool,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L75
    pub(crate) fn nvim_get_hl_by_name(
        name: String,
        rgb: bool,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L109
    pub(crate) fn nvim_get_hl_id_by_name(name: String) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1525
    pub(crate) fn nvim_get_keymap(channel_id: u64, mode: String) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L2119
    pub(crate) fn nvim_get_mark(
        name: String,
        opts: Dictionary,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1578
    pub(crate) fn nvim_get_mode() -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L682
    pub(crate) fn nvim_get_option(name: String, err: *mut Error) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L839
    pub(crate) fn nvim_get_option_info(
        name: String,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L700
    pub(crate) fn nvim_get_option_value(
        name: String,
        opts: *const Dictionary,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1975
    pub(crate) fn nvim_get_proc(pid: Integer, err: *mut Error) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1931
    pub(crate) fn nvim_get_proc_children(
        pid: Integer,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L519
    pub(crate) fn nvim_get_runtime_file(
        name: String,
        all: bool,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L621
    pub(crate) fn nvim_get_var(name: String, err: *mut Error) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L662
    pub(crate) fn nvim_get_vvar(name: String, err: *mut Error) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L311
    pub(crate) fn nvim_input(keys: String) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L338
    pub(crate) fn nvim_input_mouse(
        button: String,
        action: String,
        modifier: String,
        grid: Integer,
        row: Integer,
        col: Integer,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L930
    pub(crate) fn nvim_list_bufs() -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1724
    pub(crate) fn nvim_list_chans() -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L497
    pub(crate) fn nvim_list_runtime_paths(err: *mut Error) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1198
    pub(crate) fn nvim_list_tabpages() -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1923
    pub(crate) fn nvim_list_uis() -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L978
    pub(crate) fn nvim_list_wins() -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1485
    pub(crate) fn nvim_load_context(dict: Dictionary) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L468
    pub(crate) fn nvim_notify(
        msg: String,
        log_level: Integer,
        opts: Dictionary,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1095
    pub(crate) fn nvim_open_term(
        buf: BufHandle,
        opts: Dictionary,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L899
    pub(crate) fn nvim_out_write(str: String);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1265
    pub(crate) fn nvim_paste(
        data: String,
        crlf: bool,
        phase: Integer,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1335
    pub(crate) fn nvim_put(
        lines: Array,
        r#type: String,
        after: bool,
        follow: bool,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L398
    pub(crate) fn nvim_replace_termcodes(
        str: String,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> String;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L2023
    pub(crate) fn nvim_select_popupmenu_item(
        item: Integer,
        insert: bool,
        finish: bool,
        opts: Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L960
    pub(crate) fn nvim_set_current_buf(buffer: BufHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L567
    pub(crate) fn nvim_set_current_dir(dir: String, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L603
    pub(crate) fn nvim_set_current_line(line: String, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1228
    pub(crate) fn nvim_set_current_tabpage(
        tabpage: TabHandle,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1008
    pub(crate) fn nvim_set_current_win(window: WinHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L164
    pub(crate) fn nvim_set_hl(
        ns_id: Integer,
        name: String,
        val: *const Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1560
    pub(crate) fn nvim_set_keymap(
        channel_id: u64,
        mode: String,
        lhs: String,
        rhs: String,
        opts: *const Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L850
    pub(crate) fn nvim_set_option(
        channel_id: u64,
        name: String,
        value: Object,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L760
    pub(crate) fn nvim_set_option_value(
        name: String,
        value: Object,
        opts: *const Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L643
    pub(crate) fn nvim_set_var(name: String, value: Object, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L672
    pub(crate) fn nvim_set_vvar(name: String, value: Object, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L672
    pub(crate) fn nvim_strwidth(text: String, err: *mut Error) -> Integer;
}
