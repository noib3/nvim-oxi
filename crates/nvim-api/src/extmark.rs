use nvim_types::{
    self as nvim,
    conversion::FromObject,
    Array,
    Dictionary,
    Integer,
};

use super::ffi::extmark::*;
use super::opts::*;
use super::types::*;
use super::Buffer;
use crate::iterator::SuperIterator;
use crate::{Error, Result};

impl Buffer {
    /// Binding to [`nvim_buf_add_highlight`](https://neovim.io/doc/user/api.html#nvim_buf_add_highlight()).
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
        let hl_group = nvim::String::from(hl_group);
        let mut err = nvim::Error::new();
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

    /// Binding to [`nvim_buf_clear_namespace`](https://neovim.io/doc/user/api.html#nvim_buf_clear_namespace()).
    ///
    /// Clears namespaced objects like highlights, extmarks, or virtual text
    /// from a region.
    ///
    /// Lines are 0-indexed. It's possible to clear the namespace in the entire
    /// buffer by specifying `line_start = 0` and `line_end = -1`.
    pub fn clear_namespace(
        &mut self,
        ns_id: u32,
        line_start: usize,
        line_end: usize,
    ) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe {
            nvim_buf_clear_namespace(
                self.0,
                ns_id as Integer,
                line_start as Integer,
                line_end as Integer,
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to [`nvim_buf_del_extmark`](https://neovim.io/doc/user/api.html#nvim_buf_del_extmark()).
    ///
    /// Removes an extmark from the buffer.
    pub fn del_extmark(&mut self, ns_id: u32, extmark_id: u32) -> Result<()> {
        let mut err = nvim::Error::new();
        let was_found = unsafe {
            nvim_buf_del_extmark(
                self.0,
                ns_id as Integer,
                extmark_id as Integer,
                &mut err,
            )
        };
        err.into_err_or_flatten(|| match was_found {
            true => Ok(()),
            _ => Err(Error::custom(format!(
                "No extmark with id {extmark_id} was found"
            ))),
        })
    }

    /// Binding to [`nvim_buf_get_extmark_by_id`](https://neovim.io/doc/user/api.html#nvim_buf_get_extmark_by_id()).
    ///
    /// The first two elements of the returned tuple represent the 0-indexed
    /// `row, col` position of the extmark. The last element is only present if
    /// the [`details`](crate::api::opts::GetExtmarkByIdOptsBuilder::details)
    /// option field was set to `true`.
    pub fn get_extmark_by_id(
        &self,
        ns_id: u32,
        extmark_id: u32,
        opts: &GetExtmarkByIdOpts,
    ) -> Result<(usize, usize, Option<ExtmarkInfos>)> {
        let opts = Dictionary::from(opts);
        let mut err = nvim::Error::new();
        let tuple = unsafe {
            nvim_buf_get_extmark_by_id(
                self.0,
                ns_id as Integer,
                extmark_id as Integer,
                opts.non_owning(),
                &mut err,
            )
        };
        err.into_err_or_flatten(move || {
            if tuple.is_empty() {
                return Err(Error::custom(format!(
                    "No extmark with id {extmark_id} was found"
                )));
            }

            let mut iter = tuple.into_iter();
            let row =
                usize::from_object(iter.next().expect("row is present"))?;
            let col =
                usize::from_object(iter.next().expect("col is present"))?;
            let infos =
                iter.next().map(ExtmarkInfos::from_object).transpose()?;
            Ok((row, col, infos))
        })
    }

    /// Bindings to `nvim_buf_get_extmarks`.
    ///
    /// Gets all the extmarks in a buffer region specified by start and end
    /// positions. Returns an iterator over `(extmark_id, row, col, infos)`
    /// tuples in "traversal order". Like for [`Buffer::get_extmark_by_id`],
    /// the `infos` are present only if the
    /// [`details`](crate::api::opts::GetExtmarksOptsBuilder::details) option
    /// field was set to `true`.
    pub fn get_extmarks(
        &self,
        ns_id: u32,
        start: ExtmarkPosition,
        end: ExtmarkPosition,
        opts: &GetExtmarksOpts,
    ) -> Result<impl SuperIterator<(u32, usize, usize, Option<ExtmarkInfos>)>>
    {
        let opts = Dictionary::from(opts);
        let mut err = nvim::Error::new();
        let extmarks = unsafe {
            nvim_buf_get_extmarks(
                self.0,
                ns_id as Integer,
                start.into(),
                end.into(),
                opts.non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(move || {
            extmarks.into_iter().map(|tuple| {
                let mut iter = Array::from_object(tuple).unwrap().into_iter();
                let id = u32::from_object(iter.next().expect("id is present"))
                    .unwrap();
                let row =
                    usize::from_object(iter.next().expect("row is present"))
                        .unwrap();
                let col =
                    usize::from_object(iter.next().expect("col is present"))
                        .unwrap();
                let infos = iter
                    .next()
                    .map(ExtmarkInfos::from_object)
                    .transpose()
                    .unwrap();
                (id, row, col, infos)
            })
        })
    }

    /// Binding to [`nvim_buf_set_extmark`](https://neovim.io/doc/user/api.html#nvim_buf_set_extmark()).
    ///
    /// Creates or updates an extmark. Both `line` and `col` are 0-indexed.
    /// Returns the id of the created/updated extmark.
    pub fn set_extmark(
        &mut self,
        ns_id: u32,
        line: usize,
        col: usize,
        opts: &SetExtmarkOpts,
    ) -> Result<u32> {
        let mut err = nvim::Error::new();
        let id = unsafe {
            nvim_buf_set_extmark(
                self.0,
                ns_id as Integer,
                line as Integer,
                col as Integer,
                &opts.0,
                &mut err,
            )
        };
        err.into_err_or_else(|| id.try_into().expect("always positive"))
    }
}

/// Binding to [`nvim_create_namespace`](https://neovim.io/doc/user/api.html#nvim_create_namespace()).
///
/// Creates a new namespace or gets the id of an existing one. If `name`
/// matches an existing namespace the associated id is returned.
pub fn create_namespace(name: &str) -> u32 {
    let name = nvim::String::from(name);
    unsafe { nvim_create_namespace(name.non_owning()) }
        .try_into()
        .expect("always positive")
}

/// Binding to [`nvim_get_namespaces`](https://neovim.io/doc/user/api.html#nvim_get_namespaces()).
///
/// Returns an iterator over all the existing, non-anonymous namespace names
/// and ids tuples `(name, id)`.
pub fn get_namespaces() -> impl SuperIterator<(String, u32)> {
    unsafe { nvim_get_namespaces() }.into_iter().map(|(k, v)| {
        let k = k.try_into().expect("namespace name is valid UTF-8");
        let v = u32::from_object(v).expect("namespace id is positive");
        (k, v)
    })
}

/// Binding to [`nvim_set_decoration_provider`](https://neovim.io/doc/user/api.html#nvim_set_decoration_provider()).
///
/// Sets or changes a decoration provider for a namespace.
pub fn set_decoration_provider(
    ns_id: u32,
    opts: &DecorationProviderOpts,
) -> Result<()> {
    let opts = Dictionary::from(opts);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_decoration_provider(
            ns_id as Integer,
            opts.non_owning(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}
