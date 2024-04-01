use std::ops::RangeBounds;

use types::{self as nvim, conversion::FromObject, Array, Integer};

use crate::choose;
use crate::ffi::extmark::*;
use crate::opts::*;
use crate::types::*;
use crate::utils;
use crate::Buffer;
use crate::SuperIterator;
use crate::{Error, Result};

impl Buffer {
    /// Binding to [`nvim_buf_add_highlight()`][1].
    ///
    /// Adds a highlight to the buffer. Both `line` and `byte_range` are
    /// 0-indexed.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_add_highlight()
    pub fn add_highlight<R>(
        &mut self,
        ns_id: u32,
        hl_group: &str,
        line: usize,
        byte_range: R,
    ) -> Result<i64>
    where
        R: RangeBounds<usize>,
    {
        let hl_group = nvim::String::from(hl_group);
        let mut err = nvim::Error::new();
        let (start, end) = utils::range_to_limits::<true, _>(byte_range);
        let ns_id = unsafe {
            nvim_buf_add_highlight(
                self.0,
                ns_id.into(),
                hl_group.non_owning(),
                line as Integer,
                start,
                end,
                &mut err,
            )
        };
        choose!(err, Ok(ns_id))
    }

    /// Binding to [`nvim_buf_clear_namespace()`][1].
    ///
    /// Clears namespaced objects like highlights, extmarks, or virtual text
    /// from a region.
    ///
    /// The line range is 0-indexed.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_clear_namespace()
    pub fn clear_namespace<R>(
        &mut self,
        ns_id: u32,
        line_range: R,
    ) -> Result<()>
    where
        R: RangeBounds<usize>,
    {
        let mut err = nvim::Error::new();
        let (start, end) = utils::range_to_limits::<true, _>(line_range);
        unsafe {
            nvim_buf_clear_namespace(
                self.0,
                ns_id as Integer,
                start,
                end,
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_del_extmark()`][1].
    ///
    /// Removes an extmark from the buffer.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_del_extmark()
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
        choose!(
            err,
            match was_found {
                true => Ok(()),
                _ => Err(Error::custom(format!(
                    "No extmark with id {extmark_id} was found"
                ))),
            }
        )
    }

    /// Binding to [`nvim_buf_get_extmark_by_id()`][1].
    ///
    /// The first two elements of the returned tuple represent the 0-indexed
    /// `row, col` position of the extmark. The last element is only present if
    /// the [`details`](crate::opts::GetExtmarkByIdOptsBuilder::details) option
    /// field was set to `true`.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_extmark_by_id()
    pub fn get_extmark_by_id(
        &self,
        ns_id: u32,
        extmark_id: u32,
        opts: &GetExtmarkByIdOpts,
    ) -> Result<(usize, usize, Option<ExtmarkInfos>)> {
        #[cfg(not(feature = "neovim-nightly"))]
        let opts = types::Dictionary::from(opts);
        let mut err = nvim::Error::new();
        let tuple = unsafe {
            nvim_buf_get_extmark_by_id(
                self.0,
                ns_id as Integer,
                extmark_id as Integer,
                #[cfg(not(feature = "neovim-nightly"))]
                opts.non_owning(),
                #[cfg(feature = "neovim-nightly")]
                opts,
                &mut err,
            )
        };
        choose!(err, {
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

    /// Bindings to [`nvim_buf_get_extmarks`][1].
    ///
    /// Gets all the extmarks in a buffer region specified by start and end
    /// positions. Returns an iterator over `(extmark_id, row, col, infos)`
    /// tuples in "traversal order". Like for [`Buffer::get_extmark_by_id`],
    /// the `infos` are present only if the
    /// [`details`](crate::opts::GetExtmarksOptsBuilder::details) option field
    /// was set to `true`.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_extmarks()
    pub fn get_extmarks(
        &self,
        ns_id: u32,
        start: ExtmarkPosition,
        end: ExtmarkPosition,
        opts: &GetExtmarksOpts,
    ) -> Result<impl SuperIterator<(u32, usize, usize, Option<ExtmarkInfos>)>>
    {
        #[cfg(not(feature = "neovim-nightly"))]
        let opts = types::Dictionary::from(opts);
        let mut err = nvim::Error::new();
        let extmarks = unsafe {
            nvim_buf_get_extmarks(
                self.0,
                ns_id as Integer,
                start.into(),
                end.into(),
                #[cfg(not(feature = "neovim-nightly"))]
                opts.non_owning(),
                #[cfg(feature = "neovim-nightly")]
                opts,
                &mut err,
            )
        };
        choose!(
            err,
            Ok({
                extmarks.into_iter().map(|tuple| {
                    let mut iter =
                        Array::from_object(tuple).unwrap().into_iter();
                    let id =
                        u32::from_object(iter.next().expect("id is present"))
                            .unwrap();
                    let row = usize::from_object(
                        iter.next().expect("row is present"),
                    )
                    .unwrap();
                    let col = usize::from_object(
                        iter.next().expect("col is present"),
                    )
                    .unwrap();
                    let infos = iter
                        .next()
                        .map(ExtmarkInfos::from_object)
                        .transpose()
                        .unwrap();
                    (id, row, col, infos)
                })
            })
        )
    }

    /// Binding to [`nvim_buf_set_extmark()`][1].
    ///
    /// Creates or updates an extmark. Both `line` and `col` are 0-indexed.
    /// Returns the id of the created/updated extmark.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_set_extmark()
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
                opts,
                &mut err,
            )
        };
        choose!(err, Ok(id.try_into().expect("always positive")))
    }
}

/// Binding to [`nvim_create_namespace()`][1].
///
/// Creates a new namespace or gets the id of an existing one. If `name`
/// matches an existing namespace the associated id is returned.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_create_namespace()
pub fn create_namespace(name: &str) -> u32 {
    let name = nvim::String::from(name);
    unsafe { nvim_create_namespace(name.non_owning()) }
        .try_into()
        .expect("always positive")
}

/// Binding to [`nvim_get_namespaces()`][1].
///
/// Returns an iterator over all the existing, non-anonymous namespace names
/// and ids tuples `(name, id)`.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_namespaces()
pub fn get_namespaces() -> impl SuperIterator<(String, u32)> {
    unsafe { nvim_get_namespaces() }.into_iter().map(|(k, v)| {
        let k = k.to_string_lossy().into();
        let v = u32::from_object(v).expect("namespace id is positive");
        (k, v)
    })
}

/// Binding to [`nvim_set_decoration_provider()`][1].
///
/// Sets or changes a decoration provider for a namespace.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_decoration_provider()
pub fn set_decoration_provider(
    ns_id: u32,
    opts: &DecorationProviderOpts,
) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_decoration_provider(ns_id as Integer, opts, &mut err) };
    choose!(err, ())
}
