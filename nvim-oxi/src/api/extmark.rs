use nvim_types::{Error as NvimError, String as NvimString};

use super::ffi::extmark::*;
use super::opts::*;
use super::types::*;
use crate::Result;

/// Binding to `nvim_create_namespace`.
///
/// Creates a new namespace or gets the id of an existing one. If `name`
/// matches an existing namespace the associated id is returned.
pub fn create_namespace(name: &str) -> u32 {
    let name = NvimString::from(name);
    unsafe { nvim_create_namespace(name.non_owning()) }
        .try_into()
        .expect("always positive")
}
