use types::{self as nvim, conversion::FromObject, Array, Integer, Object};

use super::ffi::autocmd::*;
use super::opts::*;
use super::types::*;
use super::LUA_INTERNAL_CALL;
use crate::choose;
use crate::Result;
use crate::SuperIterator;

/// Binding to [`nvim_clear_autocmds()`][1].
///
/// Clears all the autocommands matched by at least one of `opts`'s fields.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_clear_autocmds()
pub fn clear_autocmds(opts: &ClearAutocmdsOpts) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe {
        nvim_clear_autocmds(
            opts,
            #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
            types::arena(),
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_create_augroup()`][1].
///
/// Creates a new autocommand group or gets an existing one. To get the id of
/// an existing augroup set the
/// [`clear`](super::opts::CreateAugroupOptsBuilder::clear) field of `opts` to
/// `false`.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_create_augroup()
pub fn create_augroup(name: &str, opts: &CreateAugroupOpts) -> Result<u32> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let id = unsafe {
        nvim_create_augroup(
            LUA_INTERNAL_CALL,
            name.as_nvim_str(),
            opts,
            &mut err,
        )
    };
    choose!(err, Ok(id.try_into().expect("always positive")))
}

/// Binding to [`nvim_create_autocmd()`][1].
///
/// Creates a new autocommand.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_create_autocmd()
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
            opts,
            #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
            types::arena(),
            &mut err,
        )
    };
    choose!(err, Ok(id.try_into().expect("always positive")))
}

/// Binding to [`nvim_del_augroup_by_id()`][1].
///
/// Deletes an autocommand group by id.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_augroup_by_id()
pub fn del_augroup_by_id(id: u32) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_del_augroup_by_id(id as Integer, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_del_augroup_by_name()`][1].
///
/// Deletes an autocommand group by name.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_augroup_by_name()
pub fn del_augroup_by_name(name: &str) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe { nvim_del_augroup_by_name(name.as_nvim_str(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_del_autocmd()`][1].
///
/// Deletes an autocommand by id.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_autocmd()
pub fn del_autocmd(id: u32) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_del_autocmd(id as Integer, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_exec_autocmds()`][1].
///
/// Executes all the autocommands registered on the given `events` that also
/// match `opts`.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_exec_autocmds()
pub fn exec_autocmds<'a, I>(events: I, opts: &ExecAutocmdsOpts) -> Result<()>
where
    I: IntoIterator<Item = &'a str>,
{
    let events = Object::from(Array::from_iter(events));
    let mut err = nvim::Error::new();
    unsafe {
        nvim_exec_autocmds(
            events.non_owning(),
            opts,
            #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
            types::arena(),
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_get_autocmds()`][1].
///
/// Gets all the autocommands that match `opts`. When multiple patterns or
/// events are provided, it will find all the autocommands that match any
/// combination of them.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_autocmds()
pub fn get_autocmds(
    opts: &GetAutocmdsOpts,
) -> Result<impl SuperIterator<AutocmdInfos>> {
    let mut err = nvim::Error::new();
    let infos = unsafe {
        nvim_get_autocmds(
            opts,
            #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
            types::arena(),
            &mut err,
        )
    };
    choose!(
        err,
        Ok({
            infos
                .into_iter()
                .map(|obj| AutocmdInfos::from_object(obj).unwrap())
        })
    )
}
