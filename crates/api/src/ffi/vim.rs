use types::*;

use crate::opts::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1037
    pub(crate) fn nvim_chan_send(
        chan: Integer,
        data: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L888
    pub(crate) fn nvim_create_buf(
        listed: bool,
        scratch: bool,
        err: *mut Error,
    ) -> BufHandle;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L623
    pub(crate) fn nvim_del_current_line(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1446
    pub(crate) fn nvim_del_keymap(
        channel_id: u64,
        mode: NonOwning<String>,
        lhs: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1958
    pub(crate) fn nvim_del_mark(
        name: NonOwning<String>,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L667
    pub(crate) fn nvim_del_var(name: NonOwning<String>, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L705
    pub(crate) fn nvim_echo(
        chunks: NonOwning<Array>,
        history: bool,
        #[cfg(not(any(feature = "neovim-0-9", feature = "neovim-nightly")))]
        opts: NonOwning<Dictionary>,
        #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
        opts: *const EchoOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L749
    pub(crate) fn nvim_err_write(str: NonOwning<String>);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L760
    pub(crate) fn nvim_err_writeln(str: NonOwning<String>);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L2070
    pub(crate) fn nvim_eval_statusline(
        str: NonOwning<String>,
        opts: *const EvalStatuslineOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L226
    pub(crate) fn nvim_feedkeys(
        keys: NonOwning<String>,
        mode: NonOwning<String>,
        escape_ks: bool,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1567
    pub(crate) fn nvim_get_chan_info(
        chan: Integer,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1275
    pub(crate) fn nvim_get_color_by_name(name: NonOwning<String>) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1288
    pub(crate) fn nvim_get_color_map(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1308
    pub(crate) fn nvim_get_context(
        opts: *const GetContextOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L794
    pub(crate) fn nvim_get_current_buf() -> BufHandle;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L603
    pub(crate) fn nvim_get_current_line(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1075
    pub(crate) fn nvim_get_current_tabpage() -> TabHandle;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L849
    pub(crate) fn nvim_get_current_win() -> WinHandle;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L82
    pub(crate) fn nvim_get_hl_id_by_name(name: NonOwning<String>) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.8.3/src/nvim/api/vim.c#L1397
    pub(crate) fn nvim_get_keymap(
        mode: NonOwning<String>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1987
    pub(crate) fn nvim_get_mark(
        name: NonOwning<String>,
        #[cfg(not(feature = "neovim-nightly"))] opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const GetMarkOpts,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1378
    pub(crate) fn nvim_get_mode(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1842
    pub(crate) fn nvim_get_proc(
        pid: Integer,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1802
    pub(crate) fn nvim_get_proc_children(
        pid: Integer,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L516
    pub(crate) fn nvim_get_runtime_file(
        name: NonOwning<String>,
        all: bool,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L635
    pub(crate) fn nvim_get_var(
        name: NonOwning<String>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L678
    pub(crate) fn nvim_get_vvar(
        name: NonOwning<String>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L305
    pub(crate) fn nvim_input(keys: NonOwning<String>) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L333
    pub(crate) fn nvim_input_mouse(
        button: NonOwning<String>,
        action: NonOwning<String>,
        modifier: NonOwning<String>,
        grid: Integer,
        row: Integer,
        col: Integer,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L772
    pub(crate) fn nvim_list_bufs(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1580
    pub(crate) fn nvim_list_chans() -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L493
    pub(crate) fn nvim_list_runtime_paths(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1053
    pub(crate) fn nvim_list_tabpages() -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1793
    pub(crate) fn nvim_list_uis(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L827
    pub(crate) fn nvim_list_wins(
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1355
    pub(crate) fn nvim_load_context(dict: NonOwning<Dictionary>) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L463
    pub(crate) fn nvim_notify(
        msg: NonOwning<String>,
        log_level: Integer,
        opts: NonOwning<Dictionary>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L952
    pub(crate) fn nvim_open_term(
        buf: BufHandle,
        #[cfg(not(feature = "neovim-nightly"))] opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")] opts: *const OpenTermOpts,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L739
    pub(crate) fn nvim_out_write(str: NonOwning<String>);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1127
    pub(crate) fn nvim_paste(
        data: NonOwning<String>,
        crlf: bool,
        phase: Integer,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1198
    pub(crate) fn nvim_put(
        lines: NonOwning<Array>,
        r#type: NonOwning<String>,
        after: bool,
        follow: bool,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L411
    pub(crate) fn nvim_replace_termcodes(
        str: NonOwning<String>,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1890
    pub(crate) fn nvim_select_popupmenu_item(
        item: Integer,
        insert: bool,
        finish: bool,
        #[cfg(not(feature = "neovim-nightly"))] opts: NonOwning<Dictionary>,
        #[cfg(feature = "neovim-nightly")]
        opts: *const SelectPopupMenuItemOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L804
    pub(crate) fn nvim_set_current_buf(buffer: BufHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L576
    pub(crate) fn nvim_set_current_dir(
        dir: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L613
    pub(crate) fn nvim_set_current_line(
        line: NonOwning<String>,
        #[cfg(feature = "neovim-nightly")] arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1085
    pub(crate) fn nvim_set_current_tabpage(
        tabpage: TabHandle,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L859
    pub(crate) fn nvim_set_current_win(window: WinHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L156
    pub(crate) fn nvim_set_hl(
        #[cfg(feature = "neovim-nightly")] channel_id: u64,
        ns_id: Integer,
        name: NonOwning<String>,
        val: *const SetHighlightOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L222
    #[cfg(feature = "neovim-nightly")]
    pub(crate) fn nvim_set_hl_ns(ns_id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L222
    #[cfg(feature = "neovim-nightly")]
    pub(crate) fn nvim_set_hl_ns_fast(ns_id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L1434
    pub(crate) fn nvim_set_keymap(
        channel_id: u64,
        mode: NonOwning<String>,
        lhs: NonOwning<String>,
        rhs: NonOwning<String>,
        opts: *const SetKeymapOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L657
    pub(crate) fn nvim_set_var(
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L689
    pub(crate) fn nvim_set_vvar(
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vim.c#L480
    pub(crate) fn nvim_strwidth(
        text: NonOwning<String>,
        err: *mut Error,
    ) -> Integer;
}
