use nvim_types::{self as nvim, Array, FromObject, Object};

use super::ffi::vimscript::*;
use super::types::*;
use super::LUA_INTERNAL_CALL;
use crate::Result;

/// Binding to [`nvim_call_dict_function`](https://neovim.io/doc/user/api.html#nvim_call_dict_function()).
///
/// Calls a VimL dictionary function with the given arguments, returning the
/// result of the funtion call.
pub fn call_dict_function<Args, Ret>(
    dict: &str,
    func: &str,
    args: Args,
) -> Result<Ret>
where
    Args: Into<Array>,
    Ret: FromObject,
{
    let dict = Object::from(nvim::String::from(dict));
    let func = nvim::String::from(func);
    let args = args.into();
    let mut err = nvim::Error::new();
    let res = unsafe {
        nvim_call_dict_function(
            dict.non_owning(),
            func.non_owning(),
            args.non_owning(),
            &mut err,
        )
    };
    err.into_err_or_flatten(|| Ok(Ret::from_obj(res)?))
}

/// Binding to [`nvim_call_function`](https://neovim.io/doc/user/api.html#nvim_call_function()).
///
/// Calls a VimL function with the given arguments, returning the result of the
/// funtion call.
pub fn call_function<Args, Ret>(func: &str, args: Args) -> Result<Ret>
where
    Args: Into<Array>,
    Ret: FromObject,
{
    let func = nvim::String::from(func);
    let args = args.into();
    let mut err = nvim::Error::new();
    let res = unsafe {
        nvim_call_function(func.non_owning(), args.non_owning(), &mut err)
    };
    err.into_err_or_flatten(|| Ok(Ret::from_obj(res)?))
}

/// Binding to [`nvim_cmd`](https://neovim.io/doc/user/api.html#nvim_cmd()).
///
/// Executes an Ex command. Unlike `crare::api::command` it takes a structured
/// `CmdInfos` object instead of a string.
#[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "neovim-0-8", feature = "neovim-nightly")))
)]
pub fn cmd(
    infos: &CmdInfos,
    opts: &super::opts::CmdOpts,
) -> Result<Option<String>> {
    let opts = super::opts::KeyDict_cmd_opts::from(opts);
    let mut err = nvim::Error::new();
    let output = unsafe {
        nvim_cmd(LUA_INTERNAL_CALL, &infos.into(), &opts.into(), &mut err)
    };
    err.into_err_or_flatten(|| {
        output
            .into_string()
            .map_err(From::from)
            .map(|output| (!output.is_empty()).then_some(output))
    })
}

/// Binding to [`nvim_command`](https://neovim.io/doc/user/api.html#nvim_command()).
///
/// Executes an Ex command.
pub fn command(command: &str) -> Result<()> {
    let command = nvim::String::from(command);
    let mut err = nvim::Error::new();
    unsafe { nvim_command(command.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_eval`](https://neovim.io/doc/user/api.html#nvim_eval()).
///
/// Evaluates a VimL expression.
pub fn eval<V>(expr: &str) -> Result<V>
where
    V: FromObject,
{
    let expr = nvim::String::from(expr);
    let mut err = nvim::Error::new();
    let output = unsafe { nvim_eval(expr.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Ok(V::from_obj(output)?))
}

/// Binding to [`nvim_exec`](https://neovim.io/doc/user/api.html#nvim_exec()).
///
/// Executes a multiline block of Ex commands. If `output` is true the
/// output is captured and returned.
pub fn exec(src: &str, output: bool) -> Result<Option<String>> {
    let src = nvim::String::from(src);
    let mut err = nvim::Error::new();
    let output = unsafe {
        nvim_exec(LUA_INTERNAL_CALL, src.non_owning(), output, &mut err)
    };
    err.into_err_or_flatten(|| {
        output
            .into_string()
            .map_err(From::from)
            .map(|output| (!output.is_empty()).then_some(output))
    })
}

/// Binding to [`nvim_parse_cmd`](https://neovim.io/doc/user/api.html#nvim_parse_cmd()).
///
/// Parses the command line.
#[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "neovim-0-8", feature = "neovim-nightly")))
)]
pub fn parse_cmd(
    src: &str,
    opts: &super::opts::ParseCmdOpts,
) -> Result<CmdInfos> {
    let src = nvim::String::from(src);
    let opts = nvim::Dictionary::from(opts);
    let mut err = nvim::Error::new();
    let dict = unsafe {
        nvim_parse_cmd(src.non_owning(), opts.non_owning(), &mut err)
    };
    err.into_err_or_flatten(|| Ok(CmdInfos::from_obj(dict.into())?))
}

/// Binding to [`nvim_parse_expression`](https://neovim.io/doc/user/api.html#nvim_parse_expression()).
///
/// Parses a VimL expression.
pub fn parse_expression(
    expr: &str,
    flags: &str,
    include_highlight: bool,
) -> Result<ParsedVimLExpression> {
    let expr = nvim::String::from(expr);
    let flags = nvim::String::from(flags);
    let mut err = nvim::Error::new();
    let dict = unsafe {
        nvim_parse_expression(
            expr.non_owning(),
            flags.non_owning(),
            include_highlight,
            &mut err,
        )
    };
    err.into_err_or_flatten(|| {
        Ok(ParsedVimLExpression::from_obj(dict.into())?)
    })
}
