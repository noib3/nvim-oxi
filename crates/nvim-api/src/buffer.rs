use std::fmt;
use std::ops::RangeBounds;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

use luajit_bindings::{self as lua, Poppable, Pushable};
use nvim_types::{
    self as nvim,
    conversion::{self, FromObject, ToObject},
    Array,
    BufHandle,
    Dictionary,
    Function,
    Integer,
    Object,
};
use serde::{Deserialize, Serialize};

use crate::choose;
use crate::ffi::buffer::*;
use crate::iterator::SuperIterator;
use crate::opts::*;
use crate::types::{CommandArgs, CommandInfos, KeymapInfos, Mode};
use crate::utils;
use crate::StringOrFunction;
use crate::LUA_INTERNAL_CALL;
use crate::{Error, Result};

/// A wrapper around a Neovim buffer handle.
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Buffer(pub(crate) BufHandle);

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
    #[inline(always)]
    fn from(handle: H) -> Self {
        Self(handle.into())
    }
}

impl From<Buffer> for Object {
    #[inline(always)]
    fn from(buf: Buffer) -> Self {
        buf.0.into()
    }
}

impl From<&Buffer> for Object {
    #[inline(always)]
    fn from(buf: &Buffer) -> Self {
        buf.0.into()
    }
}

impl FromObject for Buffer {
    #[inline(always)]
    fn from_object(obj: Object) -> StdResult<Self, conversion::Error> {
        Ok(BufHandle::from_object(obj)?.into())
    }
}

impl Poppable for Buffer {
    unsafe fn pop(
        lstate: *mut lua::ffi::lua_State,
    ) -> std::result::Result<Self, lua::Error> {
        BufHandle::pop(lstate).map(Into::into)
    }
}

impl Pushable for Buffer {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::lua_State,
    ) -> std::result::Result<std::ffi::c_int, lua::Error> {
        self.0.push(lstate)
    }
}

impl Buffer {
    /// Shorthand for [`get_current_buf`](crate::get_current_buf).
    #[inline(always)]
    pub fn current() -> Self {
        crate::get_current_buf()
    }

    /// Binding to [`nvim_buf_attach`](https://neovim.io/doc/user/api.html#nvim_buf_attach()).
    ///
    /// Used to register a set of callbacks on specific buffer events.
    pub fn attach(
        &self,
        send_buffer: bool,
        opts: &BufAttachOpts,
    ) -> Result<()> {
        let mut err = nvim::Error::new();
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
        choose!(
            err,
            match has_attached {
                true => Ok(()),
                _ => Err(Error::custom("Attaching to buffer failed")),
            }
        )
    }

    /// Binding to [`nvim_buf_call`](https://neovim.io/doc/user/api.html#nvim_buf_call()).
    ///
    /// Calls a function with this buffer as the temporary current buffer.
    pub fn call<F, R>(&self, fun: F) -> Result<R>
    where
        F: FnOnce(()) -> Result<R> + 'static,
        R: Pushable + FromObject,
    {
        let fun = Function::from_fn_once(fun);
        let mut err = nvim::Error::new();
        let obj = unsafe { nvim_buf_call(self.0, fun.lua_ref(), &mut err) };

        choose!(err, {
            fun.remove_from_lua_registry();
            Ok(R::from_object(obj)?)
        })
    }

    /// Binding to [`nvim_buf_create_user_command`](https://neovim.io/doc/user/api.html#nvim_buf_create_user_command()).
    ///
    /// Creates a new buffer-local user command.
    pub fn create_user_command<Cmd>(
        &mut self,
        name: &str,
        command: Cmd,
        opts: &CreateCommandOpts,
    ) -> Result<()>
    where
        Cmd: StringOrFunction<CommandArgs, ()>,
    {
        let opts = KeyDict_user_command::from(opts);
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let command = command.to_object();
        unsafe {
            nvim_buf_create_user_command(
                self.0,
                name.non_owning(),
                command.non_owning(),
                &opts,
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_del_keymap`](https://neovim.io/doc/user/api.html#nvim_buf_del_keymap()).
    ///
    /// Unmaps a buffer-local mapping for the given mode.
    pub fn del_keymap(&mut self, mode: Mode, lhs: &str) -> Result<()> {
        let mut err = nvim::Error::new();
        let mode = nvim::String::from(mode);
        let lhs = nvim::String::from(lhs);
        unsafe {
            nvim_buf_del_keymap(
                LUA_INTERNAL_CALL,
                self.0,
                mode.non_owning(),
                lhs.non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_del_mark`](https://neovim.io/doc/user/api.html#nvim_buf_del_mark()).
    ///
    /// Deletes a named mark in the buffer.
    pub fn del_mark(&mut self, name: char) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let was_deleted =
            unsafe { nvim_buf_del_mark(self.0, name.non_owning(), &mut err) };
        choose!(
            err,
            match was_deleted {
                true => Ok(()),
                _ => Err(Error::custom("Couldn't delete mark")),
            }
        )
    }

    /// Binding to [`nvim_buf_del_user_command`][1].
    ///
    /// Deletes a buffer-local user-command. Use
    /// [`del_user_command`](crate::del_user_command) to delete a global
    /// command.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_del_user_command()
    pub fn del_user_command(&mut self, name: &str) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe {
            nvim_buf_del_user_command(self.0, name.non_owning(), &mut err)
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_del_var`][1].
    ///
    /// Removes a buffer-scoped (`b:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_del_var()
    pub fn del_var(&mut self, name: &str) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe { nvim_buf_del_var(self.0, name.non_owning(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_delete`](https://neovim.io/doc/user/api.html#nvim_buf_delete()).
    ///
    /// Deletes the buffer (not allowed while
    /// [`textlock`](https://neovim.io/doc/user/eval.html#textlock) is active).
    pub fn delete(self, opts: &BufDeleteOpts) -> Result<()> {
        let mut err = nvim::Error::new();
        let opts = Dictionary::from(opts);
        unsafe { nvim_buf_delete(self.0, opts.non_owning(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_get_changedtick`](https://neovim.io/doc/user/api.html#nvim_buf_get_changedtick()).
    pub fn get_changedtick(&self) -> Result<u32> {
        let mut err = nvim::Error::new();
        let ct = unsafe { nvim_buf_get_changedtick(self.0, &mut err) };
        choose!(err, Ok(ct.try_into().expect("always positive")))
    }

    /// Binding to [`nvim_buf_get_commands`](https://neovim.io/doc/user/api.html#nvim_buf_get_commands()).
    pub fn get_commands(
        &self,
        opts: &GetCommandsOpts,
    ) -> Result<impl SuperIterator<CommandInfos>> {
        let mut err = nvim::Error::new();
        let opts = KeyDict_get_commands::from(opts);
        let cmds = unsafe { nvim_buf_get_commands(self.0, &opts, &mut err) };
        choose!(
            err,
            Ok({
                cmds.into_iter()
                    .map(|(_, cmd)| CommandInfos::from_object(cmd).unwrap())
            })
        )
    }

    /// Binding to [`nvim_buf_get_keymap`](https://neovim.io/doc/user/api.html#nvim_buf_get_keymap()).
    pub fn get_keymap(
        &self,
        mode: Mode,
    ) -> Result<impl SuperIterator<KeymapInfos>> {
        let mut err = nvim::Error::new();
        let mode = nvim::String::from(mode);
        let maps = unsafe {
            nvim_buf_get_keymap(
                LUA_INTERNAL_CALL,
                self.0,
                mode.non_owning(),
                &mut err,
            )
        };
        choose!(
            err,
            Ok({
                maps.into_iter()
                    .map(|obj| KeymapInfos::from_object(obj).unwrap())
            })
        )
    }

    /// Binding to [`nvim_buf_get_lines`][1].
    ///
    /// Gets a line range from the buffer. Indexing is zero-based,
    /// end-exclusive.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_lines()
    pub fn get_lines<R>(
        &self,
        line_range: R,
        strict_indexing: bool,
    ) -> Result<impl SuperIterator<nvim::String>>
    where
        R: RangeBounds<usize>,
    {
        let mut err = nvim::Error::new();
        let (start, end) = utils::range_to_limits(line_range);
        let lines = unsafe {
            nvim_buf_get_lines(
                LUA_INTERNAL_CALL,
                self.0,
                start,
                end,
                strict_indexing,
                #[cfg(feature = "neovim-nightly")]
                // The nvim_buf_get_lines() function returns no line if we use an actual lstate here
                std::ptr::null_mut(),
                &mut err,
            )
        };
        choose!(
            err,
            Ok({
                lines
                    .into_iter()
                    .map(|line| nvim::String::from_object(line).unwrap())
            })
        )
    }

    /// Binding to [`nvim_buf_get_mark`](https://neovim.io/doc/user/api.html#nvim_buf_get_mark()).
    ///
    /// Returns a (1-0) indexed `(row, col)` tuple representing the position
    /// of the named mark.
    pub fn get_mark(&self, name: char) -> Result<(usize, usize)> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let mark =
            unsafe { nvim_buf_get_mark(self.0, name.non_owning(), &mut err) };
        choose!(err, {
            let mut iter = mark.into_iter().map(usize::from_object);
            let row = iter.next().expect("row is present")?;
            let col = iter.next().expect("col is present")?;
            Ok((row, col))
        })
    }

    /// Binding to [`nvim_buf_get_name`](https://neovim.io/doc/user/api.html#nvim_buf_get_name()).
    ///
    /// Returns the full filepath of the buffer.
    pub fn get_name(&self) -> Result<PathBuf> {
        let mut err = nvim::Error::new();
        let name = unsafe { nvim_buf_get_name(self.0, &mut err) };
        choose!(err, Ok(name.into()))
    }

    /// Binding to [`nvim_buf_get_offset`](https://neovim.io/doc/user/api.html#nvim_buf_get_offset()).
    ///
    /// Returns the 0-indexed byte offset of a line.
    pub fn get_offset(&self, index: usize) -> Result<usize> {
        let mut err = nvim::Error::new();
        let offset =
            unsafe { nvim_buf_get_offset(self.0, index as Integer, &mut err) };
        choose!(err, Ok(offset.try_into().expect("offset is positive")))
    }

    /// Binding to [`nvim_buf_get_option`](https://neovim.io/doc/user/api.html#nvim_buf_get_option()).
    ///
    /// Gets a buffer option value.
    pub fn get_option<Opt>(&self, name: &str) -> Result<Opt>
    where
        Opt: FromObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let obj = unsafe {
            nvim_buf_get_option(self.0, name.non_owning(), &mut err)
        };
        choose!(err, Ok(Opt::from_object(obj)?))
    }

    /// Binding to [`nvim_buf_get_text`](https://neovim.io/doc/user/api.html#nvim_buf_get_text()).
    ///
    /// Gets a range from the buffer. This differs from `Buffer::get_lines` in
    /// that it allows retrieving only portions of a line.
    ///
    /// Indexing is zero-based, with both row and column indices being
    /// end-exclusive.
    pub fn get_text<R>(
        &self,
        line_range: R,
        start_col: usize,
        end_col: usize,
        opts: &GetTextOpts,
    ) -> Result<impl SuperIterator<nvim::String>>
    where
        R: RangeBounds<usize>,
    {
        let mut err = nvim::Error::new();
        let opts = Dictionary::from(opts);
        let (start, end) = utils::range_to_limits(line_range);
        let lines = unsafe {
            nvim_buf_get_text(
                LUA_INTERNAL_CALL,
                self.0,
                start,
                start_col.try_into()?,
                end,
                end_col.try_into()?,
                opts.non_owning(),
                #[cfg(feature = "neovim-nightly")]
                // The nvim_buf_get_text() function returns no line if we use an actual lstate here
                std::ptr::null_mut(),
                &mut err,
            )
        };
        choose!(
            err,
            Ok({
                lines
                    .into_iter()
                    .map(|line| nvim::String::from_object(line).unwrap())
            })
        )
    }

    /// Binding to [`nvim_buf_get_var`](https://neovim.io/doc/user/api.html#nvim_buf_get_var()).
    ///
    /// Gets a buffer-scoped (`b:`) variable.
    pub fn get_var<Var>(&self, name: &str) -> Result<Var>
    where
        Var: FromObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let obj =
            unsafe { nvim_buf_get_var(self.0, name.non_owning(), &mut err) };
        choose!(err, Ok(Var::from_object(obj)?))
    }

    /// Binding to [`nvim_buf_is_loaded`](https://neovim.io/doc/user/api.html#nvim_buf_is_loaded()).
    ///
    /// Checks if a buffer is valid and loaded.
    pub fn is_loaded(&self) -> bool {
        unsafe { nvim_buf_is_loaded(self.0) }
    }

    /// Binding to [`nvim_buf_is_valid`](https://neovim.io/doc/user/api.html#nvim_buf_is_valid()).
    ///
    /// Checks if a buffer is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_buf_is_valid(self.0) }
    }

    /// Binding to [`nvim_buf_line_count`](https://neovim.io/doc/user/api.html#nvim_buf_line_count()).
    ///
    /// Returns the number of lines in the given buffer.
    pub fn line_count(&self) -> Result<usize> {
        let mut err = nvim::Error::new();
        let count = unsafe { nvim_buf_line_count(self.0, &mut err) };
        choose!(err, Ok(count.try_into().expect("always positive")))
    }

    /// Binding to [`nvim_buf_set_keymap`][1].
    ///
    /// Sets a buffer-local mapping for the given mode. To set a global mapping
    /// use [`set_keymap`](crate::set_keymap) instead.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_set_keymap()
    pub fn set_keymap(
        &mut self,
        mode: Mode,
        lhs: &str,
        rhs: &str,
        opts: &SetKeymapOpts,
    ) -> Result<()> {
        let mode = nvim::String::from(mode);
        let lhs = nvim::String::from(lhs);
        let rhs = nvim::String::from(rhs);
        let opts = KeyDict_keymap::from(opts);
        let mut err = nvim::Error::new();
        unsafe {
            nvim_buf_set_keymap(
                LUA_INTERNAL_CALL,
                self.0,
                mode.non_owning(),
                lhs.non_owning(),
                rhs.non_owning(),
                &opts,
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_set_lines`][1].
    ///
    /// Sets (replaces) a line-range in the buffer. Indexing is zero-based,
    /// end-exclusive.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_set_lines()
    pub fn set_lines<Line, Lines, R>(
        &mut self,
        line_range: R,
        strict_indexing: bool,
        replacement: Lines,
    ) -> Result<()>
    where
        R: RangeBounds<usize>,
        Lines: IntoIterator<Item = Line>,
        Line: Into<nvim::String>,
    {
        let rpl = replacement.into_iter().map(Into::into).collect::<Array>();
        let mut err = nvim::Error::new();
        let (start, end) = utils::range_to_limits(line_range);
        unsafe {
            nvim_buf_set_lines(
                LUA_INTERNAL_CALL,
                self.0,
                start,
                end,
                strict_indexing,
                rpl.non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_set_mark`](https://neovim.io/doc/user/api.html#nvim_buf_set_mark()).
    ///
    /// Sets a named mark in the buffer. Marks are (1,0)-indexed, and passing 0
    /// as `line` deletes the mark.
    pub fn set_mark(
        &mut self,
        name: char,
        line: usize,
        col: usize,
    ) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
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
        choose!(
            err,
            match mark_was_set {
                true => Ok(()),
                _ => Err(Error::custom("Couldn't set mark")),
            }
        )
    }

    /// Binding to [`nvim_buf_set_name`](https://neovim.io/doc/user/api.html#nvim_buf_set_name()).
    ///
    /// Sets the full file name for a buffer.
    pub fn set_name<Name: AsRef<Path>>(&mut self, name: Name) -> Result<()> {
        let name = nvim::String::from(name.as_ref().to_owned());
        let mut err = nvim::Error::new();
        unsafe { nvim_buf_set_name(self.0, name.non_owning(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_set_option`](https://neovim.io/doc/user/api.html#nvim_buf_set_option()).
    ///
    /// Sets a buffer option value. Passing `None` as value deletes the option
    /// (only works if there's a global fallback).
    pub fn set_option<V>(&mut self, name: &str, value: V) -> Result<()>
    where
        V: ToObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe {
            nvim_buf_set_option(
                LUA_INTERNAL_CALL,
                self.0,
                name.non_owning(),
                value.to_object()?.non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_set_text`](https://neovim.io/doc/user/api.html#nvim_buf_set_text()).
    ///
    /// Sets (replaces) a range in the buffer. Indexing is zero-based, with
    /// both row and column indices being end-exclusive.
    pub fn set_text<Line, Lines, R>(
        &mut self,
        line_range: R,
        start_col: usize,
        end_col: usize,
        replacement: Lines,
    ) -> Result<()>
    where
        R: RangeBounds<usize>,
        Lines: IntoIterator<Item = Line>,
        Line: Into<nvim::String>,
    {
        let mut err = nvim::Error::new();
        let (start, end) = utils::range_to_limits(line_range);
        unsafe {
            nvim_buf_set_text(
                LUA_INTERNAL_CALL,
                self.0,
                start,
                start_col.try_into()?,
                end,
                end_col.try_into()?,
                replacement
                    .into_iter()
                    .map(|line| line.into())
                    .collect::<Array>()
                    .non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_set_var`][1].
    ///
    /// Sets a buffer-scoped (`b:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_set_var()
    pub fn set_var<V>(&mut self, name: &str, value: V) -> Result<()>
    where
        V: ToObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe {
            nvim_buf_set_var(
                self.0,
                name.non_owning(),
                value.to_object()?.non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }
}
