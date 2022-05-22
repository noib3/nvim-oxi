use std::fmt;

use nvim_types::error::{ConversionError, Error as NvimError};
use nvim_types::{BufHandle, Dictionary, Integer, NvimString, Object};

use crate::Result;

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/private/helpers.c#L411
    fn find_buffer_by_handle(
        buf: BufHandle,
        err: *mut NvimError,
    ) -> *const buf_T;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1406
    fn nvim_buf_del_user_command(
        buf: BufHandle,
        name: NvimString,
        err: *mut NvimError,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1030
    fn nvim_buf_del_var(buf: BufHandle, name: NvimString, err: *mut NvimError);

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L921
    fn nvim_buf_get_changedtick(
        buf: BufHandle,
        err: *mut NvimError,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1086
    fn nvim_buf_get_name(buf: BufHandle, err: *mut NvimError) -> NvimString;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L876
    fn nvim_buf_get_offset(
        buf: BufHandle,
        index: Integer,
        err: *mut NvimError,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1049
    fn nvim_buf_get_option(
        buf: BufHandle,
        name: NvimString,
        err: *mut NvimError,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1049
    fn nvim_buf_get_var(
        buf: BufHandle,
        name: NvimString,
        err: *mut NvimError,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1135
    fn nvim_buf_is_loaded(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1198
    fn nvim_buf_is_valid(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1049
    fn nvim_buf_line_count(buf: BufHandle, err: *mut NvimError) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1265
    fn nvim_buf_set_mark(
        buf: BufHandle,
        name: NvimString,
        line: Integer,
        col: Integer,
        opts: Dictionary,
        err: *mut NvimError,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1104
    fn nvim_buf_set_name(
        buf: BufHandle,
        name: NvimString,
        err: *mut NvimError,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1013
    fn nvim_buf_set_var(
        buf: BufHandle,
        name: NvimString,
        value: Object,
        err: *mut NvimError,
    );
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Buffer(BufHandle);

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer({})", self.0)
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct buf_T {
    _inner: [u8; 0],
}

// I'd really like to write this as
// `impl<T: Into<BufHandle>> TryFrom<T> for Buffer {..}`
// but can't because of https://github.com/rust-lang/rust/issues/50133, aaargh.
impl TryFrom<BufHandle> for Buffer {
    type Error = crate::error::Error;

    fn try_from(handle: BufHandle) -> std::result::Result<Self, Self::Error> {
        let mut err = NvimError::default();
        let _ = unsafe { find_buffer_by_handle(handle, &mut err) };
        err.into_err_or_else(|| Buffer(handle))
    }
}

#[derive(Default)]
pub struct BufAttachOpts {
    on_lines: Option<
        Box<
            dyn FnMut(
                NvimString,
                BufHandle,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Option<bool>,
        >,
    >,

    on_bytes: Option<Box<dyn FnMut(NvimString, BufHandle) -> Option<bool>>>,

    on_changedtick:
        Option<Box<dyn FnMut(NvimString, BufHandle) -> Option<bool>>>,

    on_detach: Option<Box<dyn FnMut(NvimString, BufHandle) -> Option<bool>>>,

    on_reload: Option<Box<dyn FnMut(NvimString, BufHandle) -> Option<bool>>>,

    utf_sizes: bool,

    utf_preview: bool,
}

impl Buffer {
    /// Creates a `Buffer` from a `BufHandle`. It's only available inside the
    /// crate to disallow creating `Buffer`s explicitely. This way a lot of the
    /// following methods don't have to return a `Result`, since most of the
    /// `nvim_buf_*` Neovim functions only fail when passing invalid
    /// `BufHandle`s.
    pub(crate) fn from(handle: BufHandle) -> Self {
        Buffer(handle)
    }

    /// Binding to `nvim_buf_attach`.
    pub fn attach(
        &self,
        send_buffer: bool,
        opts: BufAttachOpts,
    ) -> Result<()> {
        todo!()
    }

    /// Binding to `vim.api.nvim_buf_call`.
    pub fn call<F: FnMut()>(&self, fun: F) -> Result<()> {
        todo!()
    }

    // create_user_command

    // del_keymap

    // del_mark

    /// Binding to `nvim_buf_del_user_command`.
    pub fn nvim_buf_del_user_command<Name>(&mut self, name: Name) -> Result<()>
    where
        Name: Into<NvimString>,
    {
        let mut err = NvimError::default();
        unsafe { nvim_buf_del_user_command(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_del_var`.
    ///
    /// Removes a buffer-scoped (b:) variable.
    pub fn del_var<Name>(&mut self, name: Name) -> Result<()>
    where
        Name: Into<NvimString>,
    {
        let mut err = NvimError::default();
        unsafe { nvim_buf_del_var(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    // Binding to `nvim_buf_delete`.
    pub fn delete(self, force: bool, unload: bool) -> Result<()> {
        todo!()
    }

    /// Binding to `nvim_buf_get_changedtick`.
    pub fn get_changedtick(&self) -> Result<usize> {
        let mut err = NvimError::default();
        let ct = unsafe { nvim_buf_get_changedtick(self.0, &mut err) };
        err.into_err_or_else(|| ct.try_into().expect("always positive"))
    }

    // get_commands

    // get_keymap

    /// Binding to `vim.api.nvim_buf_get_lines`.
    pub fn get_lines(
        &self,
        start: usize,
        end: usize,
    ) -> Result<Vec<NvimString>> {
        todo!()
    }

    /// Binding to `nvim_buf_get_mark`.
    ///
    /// Returns a tuple `(row, col)` representing the position of the named
    /// mark. Marks are (1,0)-indexed.
    pub fn get_mark<Name>(&self, name: Name) -> Result<(usize, usize)>
    where
        Name: Into<NvimString>,
    {
        todo!()
    }

    /// Binding to `vim.api.nvim_buf_get_name`.
    ///
    /// Returns the full filepath of the buffer, replacing all invalid UTF-8
    /// byte sequences in the path with `U+FFFD REPLACEMENT CHARACTER` (�).
    pub fn get_name(&self) -> String {
        unsafe { nvim_buf_get_name(self.0, &mut NvimError::default()) }
            .to_string_lossy()
            .into_owned()
    }

    /// Binding to `nvim_buf_get_offset`.
    ///
    /// Returns the byte offset of a line (0-indexed, so line 1 has index 0).
    pub fn get_offset<Index>(&self, index: Index) -> Result<Integer>
    where
        Index: Into<Integer>,
    {
        let mut err = NvimError::default();
        let offset =
            unsafe { nvim_buf_get_offset(self.0, index.into(), &mut err) };
        err.into_err_or_else(|| offset)
    }

    /// Binding to `nvim_buf_get_option`.
    ///
    /// Gets a buffer option value. Fails if the returned object couldn't be
    /// converted into the specified type.
    pub fn get_option<Name, Value>(&self, name: Name) -> Result<Value>
    where
        Name: Into<NvimString>,
        Value: TryFrom<Object, Error = ConversionError>,
    {
        let mut err = NvimError::default();

        let obj =
            unsafe { nvim_buf_get_option(self.0, name.into(), &mut err) };

        // TODO: rewrite this as
        //
        // err.into_err_or_else(|| obj.try_into().map_err(crate::Error::from))
        //     .flatten()
        //
        // after https://github.com/rust-lang/rust/issues/70142 becomes stable.

        err.into_err_or_else(|| ())
            .and_then(|_| obj.try_into().map_err(crate::Error::from))
    }

    /// Bindint to `nvim_buf_get_text`.
    ///
    /// Gets a range from the buffer. Indexing is zero-based, with both row and
    /// column indices being end-exclusive.
    pub fn get_text<Int>(
        &self,
        start_row: Int,
        start_col: Int,
        end_row: Int,
        end_col: Int,
    ) -> Result<Vec<NvimString>>
    where
        Int: Into<Integer>,
    {
        todo!()
    }

    /// Binding to `nvim_buf_get_var`.
    ///
    /// Gets a buffer-scoped (b:) variable. Fails in the returned object
    /// couldn't be converted into the specified type.
    pub fn get_var<Name, Value>(&self, name: Name) -> Result<Value>
    where
        Name: Into<NvimString>,
        Value: TryFrom<Object, Error = ConversionError>,
    {
        let mut err = NvimError::default();

        let obj = unsafe { nvim_buf_get_var(self.0, name.into(), &mut err) };

        // TODO: rewrite this as
        //
        // err.into_err_or_else(|| obj.try_into().map_err(crate::Error::from))
        //     .flatten()
        //
        // after https://github.com/rust-lang/rust/issues/70142 becomes stable.

        err.into_err_or_else(|| ())
            .and_then(|_| obj.try_into().map_err(crate::Error::from))
    }

    /// Binding to `nvim_buf_is_valid`.
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
    pub fn set_lines<Line, Lines, Int>(
        &mut self,
        start: Int,
        end: Int,
        strict_indexing: bool,
        replacement: Lines,
    ) -> Result<()>
    where
        Line: Into<NvimString>,
        Lines: IntoIterator<Item = Line>,
        Int: Into<Integer>,
    {
        todo!()
    }

    /// Binding to `nvim_buf_set_mark`.
    ///
    /// Sets a named mark in the buffer. Marks are (1,0)-indexed, and passing 0
    /// as `line` deletes the mark.
    pub fn set_mark<Name, Int>(
        &mut self,
        name: Name,
        line: Int,
        col: Int,
    ) -> Result<bool>
    where
        Name: Into<NvimString>,
        Int: Into<Integer>,
    {
        let mut err = NvimError::default();
        let mark_was_set = unsafe {
            nvim_buf_set_mark(
                self.0,
                name.into(),
                line.into(),
                col.into(),
                Dictionary::new(),
                &mut err,
            )
        };
        err.into_err_or_else(|| mark_was_set)
    }

    /// Binding to `nvim_buf_set_name`.
    ///
    /// Sets the full file name for a buffer.
    pub fn set_name<Name>(&mut self, name: Name) -> Result<()>
    where
        Name: Into<NvimString>,
    {
        let mut err = NvimError::default();
        unsafe { nvim_buf_set_name(self.0, name.into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_buf_set_option`.
    ///
    /// Sets a buffer option value. Passing `None` as value deletes the option
    /// (only works if there's a global fallback).
    pub fn set_option<Name, Value>(
        &mut self,
        name: Name,
        value: Value,
    ) -> Result<()>
    where
        Name: Into<NvimString>,
        Value: Into<Object>,
    {
        todo!()
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
        todo!()
    }

    /// Binding to `nvim_buf_set_var`.
    ///
    /// Sets a buffer-scoped (b:) variable.
    pub fn set_var<Name, Value>(
        &mut self,
        name: Name,
        value: Value,
    ) -> Result<()>
    where
        Name: Into<NvimString>,
        Value: Into<Object>,
    {
        let mut err = NvimError::default();
        unsafe {
            nvim_buf_set_var(self.0, name.into(), value.into(), &mut err)
        };
        err.into_err_or_else(|| ())
    }
}
