use nvim_types::{
    Array,
    Boolean,
    Dictionary,
    Error,
    NonOwning,
    Object,
    String,
};

extern "C" {
    // https://github.com/neovim/neovim/blob/mastert/src/nvim/api/vimscript.c#L248
    pub(crate) fn nvim_call_dict_function(
        dict: Object,
        r#fn: NonOwning<String>,
        args: NonOwning<Array>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vimscript.c#L233
    pub(crate) fn nvim_call_function(
        r#fn: NonOwning<String>,
        args: NonOwning<Array>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/command.c#L296
    pub(crate) fn nvim_cmd(
        channel_id: u64,
        // cmd: *const KeyDict_cmd,
        // opts: *const KeyDict_cmd_opts,
        err: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vimscript.c#L108
    pub(crate) fn nvim_command(command: NonOwning<String>, err: *mut Error);

    // https://github.com/neovim/neovim/blob/mastet/src/nvim/api/vimscript.c#L124
    pub(crate) fn nvim_eval(
        expr: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vimscript.c#L46
    pub(crate) fn nvim_exec(
        channel_id: u64,
        src: NonOwning<String>,
        output: Boolean,
        error: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/command.c#L77
    pub(crate) fn nvim_parse_cmd(
        src: NonOwning<String>,
        opts: NonOwning<Dictionary>,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/502f03fc064d1eb427d214521d5cb9f5425a15b4/src/nvim/api/vimscript.c#L405
    pub fn nvim_parse_expression(
        expr: NonOwning<String>,
        flags: NonOwning<String>,
        highlight: bool,
        err: *mut Error,
    ) -> Dictionary;
}
