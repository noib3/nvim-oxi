use nvim_types::{Error as NvimError, Integer, String as NvimString};

use super::ffi::extmark::*;
use super::opts::*;
use super::types::*;
use crate::Result;

impl super::Buffer {
    /// Binding to `nvim_buf_add_highlight`.
    ///
    /// Adds a highlight to the buffer. `line`, `col_start` and `col_end` are
    /// all 0-indexed. You can also pass `-1` to `col_end` to highlight to end
    /// of line.
    pub fn add_highlight<I, L, S, E>(
        &mut self,
        ns_id: I,
        hl_group: &str,
        line: L,
        col_start: S,
        col_end: E,
    ) -> Result<i64>
    where
        I: Into<Integer>,
        L: Into<Integer>,
        S: Into<Integer>,
        E: Into<Integer>,
    {
        let hl_group = NvimString::from(hl_group);
        let mut err = NvimError::new();
        let ns_id = unsafe {
            nvim_buf_add_highlight(
                self.0,
                ns_id.into(),
                hl_group.non_owning(),
                line.into(),
                col_start.into(),
                col_end.into(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ns_id)
    }

    /// Binding to `nvim_buf_clear_namespace`.
    ///
    /// Clears namespaced objects like highlights, extmarks, or virtual text
    /// from a region.
    ///
    /// Lines are 0-indexed. It's possible to clear the namespace in the entire
    /// buffer by specifying `line_start = 0` and `line_end = -1`.
    pub fn clear_namespace<I, S, E>(
        &mut self,
        ns_id: I,
        line_start: S,
        line_end: E,
    ) -> Result<()>
    where
        I: Into<Integer>,
        S: Into<Integer>,
        E: Into<Integer>,
    {
        let mut err = NvimError::new();
        unsafe {
            nvim_buf_clear_namespace(
                self.0,
                ns_id.into(),
                line_start.into(),
                line_end.into(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_extmark`.
    ///
    /// Creates or updates an extmark. Both `line` and `col` are 0-indexed.
    /// Returnes the id if the creates/updates extmark.
    pub fn set_extmark<I, L, C>(
        &mut self,
        ns_id: I,
        line: L,
        col: C,
        opts: &SetExtmarkOpts,
    ) -> Result<u32>
    where
        I: Into<Integer>,
        L: Into<Integer>,
        C: Into<Integer>,
    {
        let mut err = NvimError::new();
        let id = unsafe {
            nvim_buf_set_extmark(
                self.0,
                ns_id.into(),
                line.into(),
                col.into(),
                &opts.into(),
                &mut err,
            )
        };
        err.into_err_or_else(|| id.try_into().expect("always positive"))
    }
}

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
