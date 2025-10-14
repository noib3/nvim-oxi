use types::*;

use crate::opts::*;

// On 0.10 and 0.11.
#[cfg(all(feature = "neovim-0-10", not(feature = "neovim-nightly")))]
type NvimEchoOutput = ();

// Only on Nightly.
#[cfg(feature = "neovim-nightly")]
type NvimEchoOutput = Object;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
unsafe extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1146
    pub(crate) fn nvim_chan_send(
        chan: Integer,
        data: NvimStr,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L968
    pub(crate) fn nvim_create_buf(
        listed: bool,
        scratch: bool,
        err: *mut Error,
    ) -> BufHandle;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L702
    pub(crate) fn nvim_del_current_line(arena: *mut Arena, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1549
    pub(crate) fn nvim_del_keymap(
        channel_id: u64,
        mode: NvimStr,
        lhs: NvimStr,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L2000
    pub(crate) fn nvim_del_mark(name: NvimStr, err: *mut Error) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L746
    pub(crate) fn nvim_del_var(name: NvimStr, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L784
    pub(crate) fn nvim_echo(
        chunks: NonOwning<Array>,
        history: bool,
        opts: *const EchoOpts,
        err: *mut Error,
    ) -> NvimEchoOutput;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L826
    pub(crate) fn nvim_err_write(str: NvimStr);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L837
    pub(crate) fn nvim_err_writeln(str: NvimStr);

    // https://github.com/neovim/neovim/blob/v0.11.3/src/nvim/api/vim.c#L1987
    pub(crate) fn nvim_eval_statusline(
        str: NvimStr,
        opts: *const EvalStatuslineOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L274
    pub(crate) fn nvim_feedkeys(keys: NvimStr, mode: NvimStr, escape_ks: bool);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1679
    pub(crate) fn nvim_get_chan_info(
        chan: Integer,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1275
    pub(crate) fn nvim_get_color_by_name(name: NvimStr) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1392
    pub(crate) fn nvim_get_color_map(arena: *mut Arena) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1411
    pub(crate) fn nvim_get_context(
        opts: *const GetContextOpts,
        arena: *mut Arena,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L870
    pub(crate) fn nvim_get_current_buf() -> BufHandle;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L682
    pub(crate) fn nvim_get_current_line(
        arena: *mut Arena,
        err: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1183
    pub(crate) fn nvim_get_current_tabpage() -> TabHandle;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L929
    pub(crate) fn nvim_get_current_win() -> WinHandle;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L118
    pub(crate) fn nvim_get_hl(
        ns_id: Integer,
        opts: *const GetHighlightOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L96
    pub(crate) fn nvim_get_hl_id_by_name(name: NvimStr) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L204
    pub(crate) fn nvim_get_hl_ns(
        opts: *const GetNamespaceOpts,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.8.3/src/nvim/api/vim.c#L1497
    pub(crate) fn nvim_get_keymap(mode: NvimStr, arena: *mut Arena) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1999
    pub(crate) fn nvim_get_mark(
        name: NvimStr,
        opts: *const GetMarkOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1478
    pub(crate) fn nvim_get_mode(arena: *mut Arena) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1878
    pub(crate) fn nvim_get_proc(
        pid: Integer,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1838
    pub(crate) fn nvim_get_proc_children(
        pid: Integer,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L586
    pub(crate) fn nvim_get_runtime_file(
        name: NvimStr,
        all: bool,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L714
    pub(crate) fn nvim_get_var(
        name: NvimStr,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L757
    pub(crate) fn nvim_get_vvar(
        name: NvimStr,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L360
    pub(crate) fn nvim_input(
        #[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
        channel_id: u64,
        keys: NvimStr,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L390
    pub(crate) fn nvim_input_mouse(
        button: NvimStr,
        action: NvimStr,
        modifier: NvimStr,
        grid: Integer,
        row: Integer,
        col: Integer,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L849
    pub(crate) fn nvim_list_bufs(arena: *mut Arena) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1697
    pub(crate) fn nvim_list_chans() -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L557
    pub(crate) fn nvim_list_runtime_paths(
        arena: *mut Arena,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1162
    pub(crate) fn nvim_list_tabpages() -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1829
    pub(crate) fn nvim_list_uis(arena: *mut Arena) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L908
    pub(crate) fn nvim_list_wins(arena: *mut Arena) -> Array;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1455
    pub(crate) fn nvim_load_context(dict: NonOwning<Dictionary>) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L527
    pub(crate) fn nvim_notify(
        msg: NvimStr,
        log_level: Integer,
        opts: NonOwning<Dictionary>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1060
    pub(crate) fn nvim_open_term(
        buf: BufHandle,
        opts: *const OpenTermOpts,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L816
    pub(crate) fn nvim_out_write(str: NvimStr);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1235
    pub(crate) fn nvim_paste(
        data: NvimStr,
        crlf: bool,
        phase: Integer,
        arena: *mut Arena,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1302
    pub(crate) fn nvim_put(
        lines: NonOwning<Array>,
        r#type: NvimStr,
        after: bool,
        follow: bool,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L474
    pub(crate) fn nvim_replace_termcodes(
        str: NvimStr,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1924
    pub(crate) fn nvim_select_popupmenu_item(
        item: Integer,
        insert: bool,
        finish: bool,
        opts: *const SelectPopupMenuItemOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L880
    pub(crate) fn nvim_set_current_buf(buffer: BufHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L655
    pub(crate) fn nvim_set_current_dir(dir: NvimStr, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L692
    pub(crate) fn nvim_set_current_line(
        line: NvimStr,
        arena: *mut Arena,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1193
    pub(crate) fn nvim_set_current_tabpage(
        tabpage: TabHandle,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L939
    pub(crate) fn nvim_set_current_win(window: WinHandle, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L172
    pub(crate) fn nvim_set_hl(
        channel_id: u64,
        ns_id: Integer,
        name: NvimStr,
        val: *const SetHighlightOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L223
    pub(crate) fn nvim_set_hl_ns(ns_id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L243
    pub(crate) fn nvim_set_hl_ns_fast(ns_id: Integer, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L1537
    pub(crate) fn nvim_set_keymap(
        channel_id: u64,
        mode: NvimStr,
        lhs: NvimStr,
        rhs: NvimStr,
        opts: *const SetKeymapOpts,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L736
    pub(crate) fn nvim_set_var(
        name: NvimStr,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L768
    pub(crate) fn nvim_set_vvar(
        name: NvimStr,
        value: NonOwning<Object>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vim.c#L544
    pub(crate) fn nvim_strwidth(text: NvimStr, err: *mut Error) -> Integer;
}
