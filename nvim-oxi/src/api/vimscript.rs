use nvim_types::{Array, Error, String as NvimString};

use super::ffi::vimscript::*;
use crate::lua::LUA_INTERNAL_CALL;
use crate::object::FromObject;
use crate::Result;

// nvim_call_dict_function

/// Binding to `nvim_call_function`.
///
/// Calls a VimL function with the given arguments. Returns the result of the
/// function call.
pub fn call_function<A, R>(func: &str, args: A) -> Result<R>
where
    A: Into<Array>,
    R: FromObject,
{
    let func = NvimString::from(func);
    let args = args.into();
    let mut err = Error::new();
    let res = unsafe {
        nvim_call_function(func.non_owning(), args.non_owning(), &mut err)
    };
    err.into_err_or_flatten(|| R::from_obj(res))
}

// nvim_cmd

/// Binding to `nvim_command`.
///
/// Executes an Ex command.
pub fn command(command: &str) -> Result<()> {
    let command = NvimString::from(command);
    let mut err = Error::new();
    unsafe { nvim_command(command.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_eval`.
///
/// Evaluates a VimL expression.
pub fn eval<V>(expr: &str) -> Result<V>
where
    V: FromObject,
{
    let expr = NvimString::from(expr);
    let mut err = Error::new();
    let output = unsafe { nvim_eval(expr.non_owning(), &mut err) };
    err.into_err_or_flatten(|| V::from_obj(output))
}

/// Binding to `nvim_exec`.
///
/// Executes a multiline block of Ex commands. If `output` is true the
/// output is captured and returned.
pub fn exec(src: &str, output: bool) -> Result<Option<String>> {
    let src = NvimString::from(src);
    let mut err = Error::new();
    let output = unsafe {
        nvim_exec(LUA_INTERNAL_CALL, src.non_owning(), output.into(), &mut err)
    };
    err.into_err_or_flatten(|| {
        output
            .into_string()
            .map_err(From::from)
            .map(|output| (!output.is_empty()).then(|| output))
    })
}

// nvim_parse_cmd

// nvim_parse_expression
