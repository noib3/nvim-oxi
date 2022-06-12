use nvim_types::{Array, Error};
use nvim_types::{Object, ObjectData, ObjectType, String as NvimString};

use super::ffi::vimscript::*;
use crate::lua::LUA_INTERNAL_CALL;
use crate::object::FromObject;
use crate::Result;

pub fn exec(src: &str, output: bool) -> Result<String> {
    let src = NvimString::from(src);
    let mut err = Error::new();
    let output = unsafe {
        nvim_exec(LUA_INTERNAL_CALL, src.non_owning(), output.into(), &mut err)
    };
    err.into_err_or_else(move || output)
        .and_then(|output| output.into_string().map_err(From::from))
}

pub fn command(command: &str) -> Result<()> {
    let command = NvimString::from(command);
    let mut err = Error::new();
    unsafe { nvim_command(command.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

pub fn eval<Value>(expr: &str) -> Result<Value>
where
    Value: FromObject,
{
    let expr = NvimString::from(expr);
    let mut err = Error::new();
    let output = unsafe { nvim_eval(expr.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Value::from_obj(output))
}

// TODO: accept tuple for args
pub fn call_function<Ret>(func: &str, args: Vec<Object>) -> Result<Ret>
where
    Ret: FromObject,
{
    let func = NvimString::from(func);
    let args = Array::from(args);
    let mut err = Error::new();
    let output = unsafe {
        nvim_call_function(func.non_owning(), args.non_owning(), &mut err)
    };
    err.into_err_or_flatten(|| Ret::from_obj(output))
}

// nvim_call_dict_function
// nvim_parse_expression
