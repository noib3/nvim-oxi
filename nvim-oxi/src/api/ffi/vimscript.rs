use nvim_types::{Array, Dictionary, Error, NonOwning, Object, String};

extern "C" {
    // https://github.com/neovim/neovim/blob/502f03fc064d1eb427d214521d5cb9f5425a15b4/src/nvim/api/vimscript.c#L48
    pub(crate) fn nvim_exec(
        channel_id: u64,
        src: NonOwning<String>,
        output: bool,
        error: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/502f03fc064d1eb427d214521d5cb9f5425a15b4/src/nvim/api/vimscript.c#L110
    pub(crate) fn nvim_command(command: NonOwning<String>, err: *mut Error);

    // https://github.com/neovim/neovim/blob/502f03fc064d1eb427d214521d5cb9f5425a15b4/src/nvim/api/vimscript.c#L126
    pub(crate) fn nvim_eval(
        expr: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/502f03fc064d1eb427d214521d5cb9f5425a15b4/src/nvim/api/vimscript.c#L235
    pub(crate) fn nvim_call_function(
        r#fn: NonOwning<String>,
        args: NonOwning<Array>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/502f03fc064d1eb427d214521d5cb9f5425a15b4/src/nvim/api/vimscript.c#L250
    pub(crate) fn nvim_call_dict_function(
        dict: Object,
        r#fn: NonOwning<String>,
        args: NonOwning<Array>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/502f03fc064d1eb427d214521d5cb9f5425a15b4/src/nvim/api/vimscript.c#L405
    pub fn nvim_parse_expression(
        expr: NonOwning<String>,
        flags: NonOwning<String>,
        highlight: bool,
        err: *mut Error,
    ) -> Dictionary;
}
