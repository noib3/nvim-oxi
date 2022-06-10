use std::fmt;
use std::path::PathBuf;

use nvim_types::{
    array::Array,
    dictionary::Dictionary,
    error::Error as NvimError,
    string::String as NvimString,
    BufHandle,
    Integer,
};

use super::ffi::buffer::*;
use super::opts::*;
use crate::api::types::{CommandInfos, KeymapInfos, Mode};
use crate::lua::{LuaFnOnce, LUA_INTERNAL_CALL};
use crate::object::{FromObject, ToObject};
use crate::Result;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Buffer(BufHandle);

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Buffer").field(&self.0).finish()
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
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
        let mut err = NvimError::new();
        let opts = Dictionary::from(opts);
        let has_attached = unsafe {
            nvim_buf_attach(
                LUA_INTERNAL_CALL,
                self.0,
                send_buffer,
                opts.non_owning(),
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
        let fun = LuaFnOnce::from(fun);
        let mut err = NvimError::new();
        let obj = unsafe { nvim_buf_call(self.0, fun.0, &mut err) };

        err.into_err_or_flatten(move || {
            fun.unref();
            R::from_obj(obj)
        })
    }

    /// Binding to `nvim_buf_create_user_command`.
    ///
    /// Creates a new buffer-local user command.
    pub fn create_user_command(
        &self,
        name: &str,
        command: impl ToObject,
        opts: &CreateCommandOpts,
    ) -> Result<()> {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        let command = command.to_obj()?;
        unsafe {
            nvim_buf_create_user_command(
                self.0,
                name.non_owning(),
                command.non_owning(),
                &opts.into(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_del_keymap`.
    ///
    /// Unmaps a buffer-local mapping for the given mode.
    pub fn del_keymap(&mut self, mode: Mode, lhs: &str) -> Result<()> {
        let mut err = NvimError::new();
        let mode = NvimString::from(mode);
        let lhs = NvimString::from(lhs);
        unsafe {
            nvim_buf_del_keymap(
                LUA_INTERNAL_CALL,
                self.0,
                mode.non_owning(),
                lhs.non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_del_mark`.
    ///
    /// Deletes a named mark in the buffer. If the mark is not set in the
    /// buffer it will return false.
    pub fn del_mark(&mut self, name: char) -> Result<bool> {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        let mark_was_deleted =
            unsafe { nvim_buf_del_mark(self.0, name.non_owning(), &mut err) };
        err.into_err_or_else(|| mark_was_deleted)
    }

    /// Binding to `nvim_buf_del_user_command`.
    pub fn del_user_command(&mut self, name: &str) -> Result<()> {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        unsafe {
            nvim_buf_del_user_command(self.0, name.non_owning(), &mut err)
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_del_var`.
    ///
    /// Removes a buffer-scoped (b:) variable.
    pub fn del_var(&mut self, name: &str) -> Result<()> {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        unsafe { nvim_buf_del_var(self.0, name.non_owning(), &mut err) };
        err.into_err_or_else(|| ())
    }

    // Binding to `nvim_buf_delete`.
    pub fn delete(self, opts: BufDeleteOpts) -> Result<()> {
        let mut err = NvimError::new();
        let opts = Dictionary::from(opts);
        unsafe { nvim_buf_delete(self.0, opts.non_owning(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_get_changedtick`.
    pub fn get_changedtick(&self) -> Result<usize> {
        let mut err = NvimError::new();
        let ct = unsafe { nvim_buf_get_changedtick(self.0, &mut err) };
        err.into_err_or_else(|| ct.try_into().expect("always positive"))
    }

    /// Binding to `nvim_buf_get_commands`.
    ///
    /// Returns an iterator over the buffer-local `CommandInfos`.
    pub fn get_commands(
        &self,
        opts: &GetCommandsOpts,
    ) -> Result<impl Iterator<Item = CommandInfos>> {
        let mut err = NvimError::new();
        let cmds =
            unsafe { nvim_buf_get_commands(self.0, &opts.into(), &mut err) };
        err.into_err_or_else(|| {
            cmds.into_iter().flat_map(|(_, cmd)| CommandInfos::from_obj(cmd))
        })
    }

    /// Binding to `nvim_buf_get_keymap`.
    ///
    /// Returns an iterator over the buffer-local `KeymapInfos`.
    pub fn get_keymap(
        &self,
        mode: Mode,
    ) -> Result<impl Iterator<Item = KeymapInfos>> {
        let mut err = NvimError::new();
        let mode = NvimString::from(mode);
        let maps = unsafe {
            nvim_buf_get_keymap(
                LUA_INTERNAL_CALL,
                self.0,
                mode.non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(|| {
            maps.into_iter().flat_map(KeymapInfos::from_obj)
        })
    }

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
        let mut err = NvimError::new();
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
    pub fn get_mark(&self, name: char) -> Result<(usize, usize)> {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        let mark =
            unsafe { nvim_buf_get_mark(self.0, name.non_owning(), &mut err) };
        err.into_err_or_flatten(|| <(usize, usize)>::from_obj(mark.into()))
    }

    /// Binding to `nvim_buf_get_name`.
    ///
    /// Returns the full filepath of the buffer.
    pub fn get_name(&self) -> Result<PathBuf> {
        let mut err = NvimError::new();
        let name = unsafe { nvim_buf_get_name(self.0, &mut err) };
        err.into_err_or_else(|| name.into())
    }

    /// Binding to `nvim_buf_get_offset`.
    ///
    /// Returns the byte offset of a line (0-indexed, so line 1 has index 0).
    pub fn get_offset(&self, index: impl Into<Integer>) -> Result<usize> {
        let mut err = NvimError::new();
        let offset =
            unsafe { nvim_buf_get_offset(self.0, index.into(), &mut err) };
        err.into_err_or_else(|| offset.try_into().expect("offset is positive"))
    }

    /// Binding to `nvim_buf_get_option`.
    ///
    /// Gets a buffer option value. Fails if the specified type couldn't be
    /// deserialized from the returned object.
    pub fn get_option<Value>(&self, name: &str) -> Result<Value>
    where
        Value: FromObject,
    {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        let obj = unsafe {
            nvim_buf_get_option(self.0, name.non_owning(), &mut err)
        };
        err.into_err_or_flatten(|| Value::from_obj(obj))
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
        let mut err = NvimError::new();
        let dict = Dictionary::new();
        let lines = unsafe {
            nvim_buf_get_text(
                LUA_INTERNAL_CALL,
                self.0,
                start_row.try_into()?,
                start_col.try_into()?,
                end_row.try_into()?,
                end_col.try_into()?,
                dict.non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(|| {
            lines.into_iter().flat_map(NvimString::try_from)
        })
    }

    /// Binding to `nvim_buf_get_var`.
    ///
    /// Gets a buffer-scoped (b:) variable. Fails if the specified type
    /// couldn't be deserialized from the returned object.
    pub fn get_var<Value>(&self, name: &str) -> Result<Value>
    where
        Value: FromObject,
    {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        let obj =
            unsafe { nvim_buf_get_var(self.0, name.non_owning(), &mut err) };
        err.into_err_or_flatten(|| Value::from_obj(obj))
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
        let mut err = NvimError::new();
        let count = unsafe { nvim_buf_line_count(self.0, &mut err) };
        err.into_err_or_else(|| count.try_into().expect("always positive"))
    }

    /// Binding to `nvim_buf_set_keymap`.
    ///
    /// Sets a buffer-local mapping for the given mode.
    pub fn set_keymap(
        &self,
        mode: Mode,
        lhs: &str,
        rhs: Option<&str>,
        opts: &SetKeymapOpts,
    ) -> Result<()> {
        let mut err = NvimError::new();
        let mode = NvimString::from(mode);
        let lhs = NvimString::from(lhs);
        let rhs = NvimString::from(rhs.unwrap_or_default());
        unsafe {
            nvim_buf_set_keymap(
                LUA_INTERNAL_CALL,
                self.0,
                mode.non_owning(),
                lhs.non_owning(),
                rhs.non_owning(),
                &opts.into(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

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
        let mut err = NvimError::new();
        let replacement =
            replacement.into_iter().map(|line| line.into()).collect::<Array>();

        unsafe {
            nvim_buf_set_lines(
                LUA_INTERNAL_CALL,
                self.0,
                start.into(),
                end.into(),
                strict_indexing,
                replacement.non_owning(),
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
        name: char,
        line: usize,
        col: usize,
    ) -> Result<bool> {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        let mark_was_set = unsafe {
            nvim_buf_set_mark(
                self.0,
                name.non_owning(),
                line.try_into()?,
                col.try_into()?,
                Dictionary::new().non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(|| mark_was_set)
    }

    /// Binding to `nvim_buf_set_name`.
    ///
    /// Sets the full file name for a buffer.
    pub fn set_name(&mut self, name: impl Into<NvimString>) -> Result<()> {
        let mut err = NvimError::new();
        let name = name.into();
        unsafe { nvim_buf_set_name(self.0, name.non_owning(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_option`.
    ///
    /// Sets a buffer option value. Passing `None` as value deletes the option
    /// (only works if there's a global fallback).
    pub fn set_option<V>(&mut self, name: &str, value: V) -> Result<()>
    where
        V: ToObject,
    {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        unsafe {
            nvim_buf_set_option(
                LUA_INTERNAL_CALL,
                self.0,
                name.non_owning(),
                value.to_obj()?.non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_text`.
    ///
    /// Sets (replaces) a range in the buffer. Indexing is zero-based, with
    /// both row and column indices being end-exclusive.
    pub fn set_text<Line, Lines>(
        &mut self,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
        replacement: Lines,
    ) -> Result<()>
    where
        Line: Into<NvimString>,
        Lines: IntoIterator<Item = Line>,
    {
        let mut err = NvimError::new();
        unsafe {
            nvim_buf_set_text(
                LUA_INTERNAL_CALL,
                self.0,
                start_row.try_into()?,
                start_col.try_into()?,
                end_row.try_into()?,
                end_col.try_into()?,
                replacement
                    .into_iter()
                    .map(|line| line.into())
                    .collect::<Array>()
                    .non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_var`.
    ///
    /// Sets a buffer-scoped (b:) variable.
    pub fn set_var(&mut self, name: &str, value: impl ToObject) -> Result<()> {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        unsafe {
            nvim_buf_set_var(
                self.0,
                name.non_owning(),
                value.to_obj()?.non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }
}
