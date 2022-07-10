use nvim_types::{self as nvim, Array, Integer, Object};

use super::ffi::autocmd::*;
use super::opts::*;
use super::types::*;
use crate::object::FromObject;
use crate::{lua::LUA_INTERNAL_CALL, Result};

/// Binding to [`nvim_clear_autocmds`](https://neovim.io/doc/user/api.html#nvim_clear_autocmds()).
///
/// Clears all the autocommands matched by at least one of `opts`'s fields.
pub fn clear_autocmds(opts: &ClearAutocmdsOpts) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_clear_autocmds(&opts.into(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_create_augroup`](https://neovim.io/doc/user/api.html#nvim_create_augroup()).
///
/// Creates a new autocommand group or gets an existing one. To get the id of
/// an existing augroup set the
/// [`clear`](super::opts::CreateAugroupOptsBuilder::clear) field of `opts` to
/// `false`.
pub fn create_augroup(name: &str, opts: &CreateAugroupOpts) -> Result<u32> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
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

/// Binding to [`nvim_create_autocmd`](https://neovim.io/doc/user/api.html#nvim_create_autocmd()).
///
/// Creates a new autocommand.
pub fn create_autocmd<'a, I>(
    events: I,
    opts: &CreateAutocmdOpts,
) -> Result<u32>
where
    I: IntoIterator<Item = &'a str>,
{
    let events = Object::from(Array::from_iter(events));
    let mut err = nvim::Error::new();
    let id = unsafe {
        nvim_create_autocmd(
            LUA_INTERNAL_CALL,
            events.non_owning(),
            &opts.into(),
            &mut err,
        )
    };
    err.into_err_or_else(|| id.try_into().expect("always positive"))
}

/// Binding to [`nvim_del_augroup_by_id`](https://neovim.io/doc/user/api.html#nvim_del_augroup_by_id()).
///
/// Deletes an autocommand group by id.
pub fn del_augroup_by_id(id: u32) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_del_augroup_by_id(id as Integer, &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_del_augroup_by_name`](https://neovim.io/doc/user/api.html#nvim_del_augroup_by_name()).
///
/// Deletes an autocommand group by name.
pub fn del_augroup_by_name(name: &str) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe { nvim_del_augroup_by_name(name.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_del_autocmd`](https://neovim.io/doc/user/api.html#nvim_del_autocmd()).
///
/// Deletes an autocommand by id.
pub fn del_autocmd(id: u32) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_del_autocmd(id as Integer, &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_exec_autocmds`](https://neovim.io/doc/user/api.html#nvim_exec_autocmds()).
///
/// Executes all the autocommands registered on the given `events` that also
/// match `opts`.
pub fn exec_autocmds<'a, I>(events: I, opts: &ExecAutocmdsOpts) -> Result<()>
where
    I: IntoIterator<Item = &'a str>,
{
    let events = Object::from(Array::from_iter(events));
    let mut err = nvim::Error::new();
    unsafe { nvim_exec_autocmds(events.non_owning(), &opts.into(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to [`nvim_get_autocmds`](https://neovim.io/doc/user/api.html#nvim_get_autocmds()).
///
/// Gets all the autocommands that match `opts`. When multiple patterns or
/// events are provided, it will find all the autocommands that match any
/// combination of them.
pub fn get_autocmds(
    opts: &GetAutocmdsOpts,
) -> Result<impl ExactSizeIterator<Item = AutocmdInfos>> {
    let mut err = nvim::Error::new();
    let infos = unsafe { nvim_get_autocmds(&opts.into(), &mut err) };
    err.into_err_or_else(|| {
        infos.into_iter().map(|obj| AutocmdInfos::from_obj(obj).unwrap())
    })
}
