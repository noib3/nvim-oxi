use types::*;

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vimscript.c#L278
    pub(crate) fn nvim_call_dict_function(
        dict: NonOwning<Object>,
        r#fn: NvimStr,
        args: NonOwning<Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vimscript.c#L263
    pub(crate) fn nvim_call_function(
        r#fn: NvimStr,
        args: NonOwning<Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vimscript.c#L135
    pub(crate) fn nvim_command(command: NvimStr, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vimscript.c#L151
    pub(crate) fn nvim_eval(
        expr: NvimStr,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vimscript.c#L53
    pub(crate) fn nvim_exec2(
        channel_id: u64,
        src: NvimStr,
        opts: *const crate::opts::ExecOpts,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/vimscript.c#L430
    pub fn nvim_parse_expression(
        expr: NvimStr,
        flags: NvimStr,
        highlight: bool,
        arena: *mut Arena,
        err: *mut Error,
    ) -> Dictionary;
}
