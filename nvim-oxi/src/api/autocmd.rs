use nvim_types::{
    Array,
    Error as NvimError,
    Integer,
    Object,
    String as NvimString,
};

use super::ffi::autocmd::*;
use crate::Result;

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
