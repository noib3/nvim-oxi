use std::fmt;
use std::path::PathBuf;

use nvim_types::error::Error as NvimError;
use nvim_types::{
    array::Array,
    dictionary::Dictionary,
    string::String as NvimString,
    BufHandle,
    Integer,
};

use super::opts::*;
use super::r#extern::*;
use crate::api::global::opts::UserCommandOpts;
use crate::lua::{LuaRef, LUA_INTERNAL_CALL};
use crate::object::{FromObject, ToObject};
use crate::Result;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Buffer(BufHandle);

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer({})", self.0)
    }
}

impl<H: Into<BufHandle>> From<H> for Buffer {
    fn from(handle: H) -> Self {
        Buffer(handle.into())
    }
}

impl Buffer {
    /// Shorthand for `nvim_oxi::api::get_current_buf`.
    #[inline(always)]
    pub fn current() -> Self {
        crate::api::get_current_buf()
    }

    /// Binding to `nvim_buf_attach`.
    pub fn attach(
        &self,
        send_buffer: bool,
        opts: BufAttachOpts,
    ) -> Result<bool> {
        let mut err = NvimError::default();
        let has_attached = unsafe {
            nvim_buf_attach(
                LUA_INTERNAL_CALL,
                self.0,
                send_buffer,
                opts.into(),
                &mut err,
            )
        };
        err.into_err_or_else(|| has_attached)
    }

    /// Binding to `nvim_buf_call`.
    ///
    /// Calls a closure with the buffer as the temporary current buffer.
    pub fn call<F, R>(&self, fun: F) -> Result<R>
    where
        R: ToObject + FromObject,
        F: FnOnce(()) -> Result<R> + 'static,
    {
        let luaref = LuaRef::from_fn_once(fun);
        let mut err = NvimError::default();
        let obj = unsafe { nvim_buf_call(self.0, luaref.0, &mut err) };

        err.into_err_or_flatten(move || {
            luaref.unref();
            R::from_obj(obj).map_err(crate::Error::from)
        })
    }

    /// Binding to `nvim_buf_create_user_command`.
    ///
    /// Creates a new buffer-local user command.
    pub fn create_user_command(
        &self,
        name: &str,
        command: impl ToObject,
        opts: UserCommandOpts,
    ) -> Result<()> {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_create_user_command(
                self.0,
                name.into(),
                command.to_obj(),
                &opts.into(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_del_keymap`.
    ///
    /// Unmaps a buffer-local mapping for the given mode.
    pub fn del_keymap(&mut self, mode: &str, lhs: &str) -> Result<()> {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_del_keymap(
                LUA_INTERNAL_CALL,
                self.0,
                mode.into(),
                lhs.into(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_del_mark`.
    ///
    /// Deletes a named mark in the buffer. If the mark is not set in the
    /// buffer it will return false.
    pub fn del_mark(&mut self, name: &str) -> Result<bool> {
        let mut err = NvimError::default();
        let mark_was_deleted =
            unsafe { nvim_buf_del_mark(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| mark_was_deleted)
    }

    /// Binding to `nvim_buf_del_user_command`.
    pub fn del_user_command(&mut self, name: &str) -> Result<()> {
        let mut err = NvimError::default();
        unsafe { nvim_buf_del_user_command(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_del_var`.
    ///
    /// Removes a buffer-scoped (b:) variable.
    pub fn del_var(&mut self, name: &str) -> Result<()> {
        let mut err = NvimError::default();
        unsafe { nvim_buf_del_var(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    // Binding to `nvim_buf_delete`.
    pub fn delete(self, force: bool, unload: bool) -> Result<()> {
        let opts = Dictionary::from_iter([
            ("force", force.to_obj()),
            ("unload", unload.to_obj()),
        ]);
        let mut err = NvimError::default();
        unsafe { nvim_buf_delete(self.0, opts, &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_get_changedtick`.
    pub fn get_changedtick(&self) -> Result<usize> {
        let mut err = NvimError::default();
        let ct = unsafe { nvim_buf_get_changedtick(self.0, &mut err) };
        err.into_err_or_else(|| ct.try_into().expect("always positive"))
    }

    // get_commands

    // get_keymap

    /// Binding to `nvim_buf_get_lines`.
    ///
    /// Gets a line range from the buffer. Indexing is zero-based,
    /// end-exclusive. Out of bounds indices are clamped to the nearest valid
    /// value, unless `strict_indexing` is set, in which case passing an
    /// invalid index will cause an error.
    pub fn get_lines(
        &self,
        start: usize,
        end: usize,
        strict_indexing: bool,
    ) -> Result<impl Iterator<Item = NvimString>> {
        let mut err = NvimError::default();
        let lines = unsafe {
            nvim_buf_get_lines(
                LUA_INTERNAL_CALL,
                self.0,
                start.try_into()?,
                end.try_into()?,
                strict_indexing,
                &mut err,
            )
        };
        err.into_err_or_else(|| {
            lines.into_iter().flat_map(NvimString::try_from)
        })
    }

    /// Binding to `nvim_buf_get_mark`.
    ///
    /// Returns a tuple `(row, col)` representing the position of the named
    /// mark. Marks are (1,0)-indexed.
    pub fn get_mark(&self, name: &str) -> Result<(usize, usize)> {
        let mut err = NvimError::default();
        let mark = unsafe { nvim_buf_get_mark(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| {
            let mut iter = mark.into_vec().into_iter();
            let row = usize::from_obj(iter.next().unwrap()).unwrap();
            let col = usize::from_obj(iter.next().unwrap()).unwrap();
            (row, col)
        })
    }

    /// Binding to `nvim_buf_get_name`.
    ///
    /// Returns the full filepath of the buffer, replacing all invalid UTF-8
    /// byte sequences in the path with `U+FFFD REPLACEMENT CHARACTER` (�).
    pub fn get_name(&self) -> Result<PathBuf> {
        let mut err = NvimError::default();
        let name = unsafe { nvim_buf_get_name(self.0, &mut err) };
        err.into_err_or_else(|| name.into())
    }

    /// Binding to `nvim_buf_get_offset`.
    ///
    /// Returns the byte offset of a line (0-indexed, so line 1 has index 0).
    pub fn get_offset(&self, index: impl Into<Integer>) -> Result<usize> {
        let mut err = NvimError::default();
        let offset =
            unsafe { nvim_buf_get_offset(self.0, index.into(), &mut err) };
        err.into_err_or_else(|| offset.try_into().expect("offset is positive"))
    }

    /// Binding to `nvim_buf_get_option`.
    ///
    /// Gets a buffer option value. Fails if the returned object couldn't be
    /// converted into the specified type.
    pub fn get_option<Value>(&self, name: &str) -> Result<Value>
    where
        Value: FromObject,
    {
        let mut err = NvimError::default();
        let obj =
            unsafe { nvim_buf_get_option(self.0, name.into(), &mut err) };
        err.into_err_or_flatten(|| {
            Value::from_obj(obj).map_err(crate::Error::from)
        })
    }

    /// Binding to `nvim_buf_get_text`.
    ///
    /// Gets a range from the buffer. This differs from `Buffer::get_lines` in
    /// that it allows retrieving only portions of a line.
    ///
    /// Indexing is zero-based, with both row and column indices being
    /// end-exclusive.
    pub fn get_text(
        &self,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> Result<impl Iterator<Item = NvimString>> {
        let mut err = NvimError::default();
        let lines = unsafe {
            nvim_buf_get_text(
                LUA_INTERNAL_CALL,
                self.0,
                start_row.try_into()?,
                start_col.try_into()?,
                end_row.try_into()?,
                end_col.try_into()?,
                Dictionary::new(),
                &mut err,
            )
        };
        err.into_err_or_else(|| {
            lines
                .into_iter()
                .map(|line| line.try_into().expect("always a string"))
        })
    }

    /// Binding to `nvim_buf_get_var`.
    ///
    /// Gets a buffer-scoped (b:) variable. Fails in the returned object
    /// couldn't be converted into the specified type.
    pub fn get_var<Value>(&self, name: &str) -> Result<Value>
    where
        Value: FromObject,
    {
        let mut err = NvimError::default();
        let obj = unsafe { nvim_buf_get_var(self.0, name.into(), &mut err) };
        err.into_err_or_flatten(|| {
            Value::from_obj(obj).map_err(crate::Error::from)
        })
    }

    /// Binding to `nvim_buf_is_loaded`.
    ///
    /// Checks if a buffer is valid and loaded.
    pub fn is_loaded(&self) -> bool {
        unsafe { nvim_buf_is_loaded(self.0) }
    }

    /// Binding to `nvim_buf_is_valid`.
    ///
    /// Checks if a buffer is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_buf_is_valid(self.0) }
    }

    /// Binding to `nvim_buf_line_count`.
    ///
    /// Returns the number of lines in the given buffer.
    pub fn line_count(&self) -> Result<usize> {
        let mut err = NvimError::default();
        let count = unsafe { nvim_buf_line_count(self.0, &mut err) };
        err.into_err_or_else(|| count.try_into().expect("always positive"))
    }

    // set_keymap

    /// Binding to `nvim_buf_set_lines`.
    ///
    /// Sets (replaces) a line-range in the buffer. Indexing is zero-based,
    /// end-exclusive.
    pub fn set_lines<Int, Line, Lines>(
        &mut self,
        start: Int,
        end: Int,
        strict_indexing: bool,
        replacement: Lines,
    ) -> Result<()>
    where
        Int: Into<Integer>,
        Line: Into<NvimString>,
        Lines: IntoIterator<Item = Line>,
    {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_set_lines(
                LUA_INTERNAL_CALL,
                self.0,
                start.into(),
                end.into(),
                strict_indexing,
                replacement
                    .into_iter()
                    .map(|line| line.into().to_obj())
                    .collect::<Array>(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_mark`.
    ///
    /// Sets a named mark in the buffer. Marks are (1,0)-indexed, and passing 0
    /// as `line` deletes the mark.
    pub fn set_mark(
        &mut self,
        name: &str,
        line: usize,
        col: usize,
    ) -> Result<bool> {
        let mut err = NvimError::default();
        let mark_was_set = unsafe {
            nvim_buf_set_mark(
                self.0,
                name.into(),
                line.try_into()?,
                col.try_into()?,
                Dictionary::new(),
                &mut err,
            )
        };
        err.into_err_or_else(|| mark_was_set)
    }

    /// Binding to `nvim_buf_set_name`.
    ///
    /// Sets the full file name for a buffer.
    pub fn set_name(&mut self, name: impl Into<NvimString>) -> Result<()> {
        let mut err = NvimError::default();
        unsafe { nvim_buf_set_name(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_option`.
    ///
    /// Sets a buffer option value. Passing `None` as value deletes the option
    /// (only works if there's a global fallback).
    pub fn set_option<Value>(&mut self, name: &str, value: Value) -> Result<()>
    where
        Value: ToObject,
    {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_set_option(
                LUA_INTERNAL_CALL,
                self.0,
                name.into(),
                value.to_obj(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_text`.
    ///
    /// Sets (replaces) a range in the buffer. Indexing is zero-based, with
    /// both row and column indices being end-exclusive.
    pub fn set_text<Int, Line, Lines>(
        &mut self,
        start_row: Int,
        start_col: Int,
        end_row: Int,
        end_col: Int,
        replacement: Lines,
    ) -> Result<()>
    where
        Int: Into<Integer>,
        Line: Into<NvimString>,
        Lines: IntoIterator<Item = Line>,
    {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_set_text(
                LUA_INTERNAL_CALL,
                self.0,
                start_row.into(),
                start_col.into(),
                end_row.into(),
                end_col.into(),
                replacement
                    .into_iter()
                    .map(|line| line.into().to_obj())
                    .collect::<Array>(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_var`.
    ///
    /// Sets a buffer-scoped (b:) variable.
    pub fn set_var(&mut self, name: &str, value: impl ToObject) -> Result<()> {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_set_var(self.0, name.into(), value.to_obj(), &mut err)
        };
        err.into_err_or_else(|| ())
    }
}
