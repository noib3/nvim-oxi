use std::fmt;

use nvim_types::error::{ConversionError, Error as NvimError};
use nvim_types::{BufHandle, Integer, NvimString, Object};

use crate::Result;

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/private/helpers.c#L411
    fn find_buffer_by_handle(
        buf: BufHandle,
        err: *mut NvimError,
    ) -> *const buf_T;

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

    // del_user_command

    // del_var

    // delete

    // detach

    // get_changedtick

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

    // get_mark

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
    /// Returns a buffer option value. Fails if the returned object couldn't be
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

    // get_text

    // get_var

    // is_loaded

    // is_valid

    // line_count

    // set_keymap

    /// Binding to `vim.api.nvim_buf_set_lines`.
    pub fn set_lines<
        Line: Into<NvimString>,
        Lines: IntoIterator<Item = Line>,
    >(
        &mut self,
        start: usize,
        end: usize,
        strict_indexing: bool,
        replacement: Lines,
    ) -> Result<()> {
        todo!()
    }

    // set_mark

    // set_name

    /// Binding to `vim.api.nvim_buf_set_option`.
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

    /// Binding to `vim.api.nvim_buf_set_option`.
    pub fn set_text<
        Line: Into<NvimString>,
        Lines: IntoIterator<Item = Line>,
    >(
        &mut self,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
        replacement: Lines,
    ) -> Result<()> {
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
        let _ = unsafe {
            nvim_buf_set_var(self.0, name.into(), value.into(), &mut err)
        };
        err.into_err_or_else(|| ())
    }
}
