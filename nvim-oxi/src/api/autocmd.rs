use nvim_types::{
    Array,
    Error as NvimError,
    Integer,
    Object,
    String as NvimString,
};

use super::ffi::autocmd::*;
use super::opts::*;
use super::types::*;
use crate::{
    lua::LUA_INTERNAL_CALL,
    object::{FromObject, ToObject},
    Result,
};

/// Binding to `nvim_clear_autocmds`.
///
/// Clears all the autocommands matched by at least one of the fields of
pub fn clear_autocmds(opts: ClearAutocmdsOpts) -> Result<()> {
    let mut err = NvimError::new();
    unsafe { nvim_clear_autocmds(&opts.into(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_create_augroup`.
///
/// Creates a new autocommand group or gets an existing one. To get the id of
/// an existing augroup set the `clear` field of `opts` to `false`.
pub fn create_augroup(name: &str, opts: CreateAugroupOpts) -> Result<u32> {
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    let id = unsafe {
        nvim_create_augroup(
            LUA_INTERNAL_CALL,
            name.non_owning(),
            &opts.into(),
            &mut err,
        )
    };
    err.into_err_or_else(|| id.try_into().expect("always positive"))
}

/// Binding to `nvim_del_augroup_by_id`.
///
/// Deletes an autocommand group by id.
pub fn del_augroup_by_id(id: u32) -> Result<()> {
    let mut err = NvimError::new();
    unsafe { nvim_del_augroup_by_id(id as Integer, &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_augroup_by_name`.
///
/// Deletes an autocommand group by name.
pub fn del_augroup_by_name(name: &str) -> Result<()> {
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    unsafe { nvim_del_augroup_by_name(name.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_autocmd`.
///
/// Deletes an autocommand by id.
pub fn del_autocmd(id: u32) -> Result<()> {
    let mut err = NvimError::new();
    unsafe { nvim_del_autocmd(id as Integer, &mut err) };
    err.into_err_or_else(|| ())
}
