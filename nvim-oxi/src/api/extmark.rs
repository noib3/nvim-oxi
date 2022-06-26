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

/// Binding to `nvim_get_namespaces`.
///
/// Returns an iterator over all the existing, non-anonymous namespace names
/// and ids tuples `(name, id)`.
pub fn get_namespaces() -> impl Iterator<Item = (String, u32)> {
    unsafe { nvim_get_namespaces() }.into_iter().map(|(k, v)| {
        let k = k.try_into().expect("namespace name is valid UTF-8");
        let v = v.try_into().expect("namespace id is positive");
        (k, v)
    })
}
