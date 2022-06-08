use nvim_types::{
    array::Array, dictionary::Dictionary, error::Error as NvimError,
    object::Object, string::String as NvimString, Integer,
};

use super::ffi::autocmd::*;
use super::opts::CreateCommandOpts;
use super::types::Mode;
use crate::{
    api::Buffer,
    lua::LUA_INTERNAL_CALL,
    object::{FromObject, ToObject},
    Result,
};

pub fn clear_autocmds(opts: &CreateCommandOpts) -> Result<()> {
    let mut error = NvimError::new();

    unsafe { nvim_clear_autocmds(&(opts.into()), &mut error) };
    error.into_err_or_else(|| ())
}

pub fn create_augroup(
    channel_id: u64,
    name: &str,
    opts: &CreateCommandOpts,
) -> Result<Integer> {
    let mut error = NvimError::new();

    let result = unsafe {
        nvim_create_augroup(
            channel_id,
            name.into(),
            &(opts.into()),
            &mut error,
        )
    };

    error.into_err_or_else(|| result)
}
