use core::ops::RangeBounds;
use std::error::Error as StdError;
use std::fmt;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

use luajit::{self as lua, Poppable, Pushable};
use serde::{Deserialize, Serialize};
use types::{
    self as nvim,
    conversion::{self, FromObject, ToObject},
    Array,
    BufHandle,
    Function,
    Integer,
    Object,
};

use crate::choose;
use crate::ffi::buffer::*;
use crate::opts::*;
use crate::types::{KeymapInfos, Mode};
use crate::utils;
use crate::SuperIterator;
use crate::LUA_INTERNAL_CALL;
use crate::{Error, IntoResult, Result};

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
        lstate: *mut lua::ffi::State,
    ) -> std::result::Result<Self, lua::Error> {
        BufHandle::pop(lstate).map(Into::into)
    }
}

impl Pushable for Buffer {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
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

    /// Retrieve buffer's underlying id/handle
    #[inline(always)]
    pub fn handle(&self) -> i32 {
        self.0
    }

    /// Binding to [`nvim_buf_attach()`][1].
    ///
    /// Used to register a set of callbacks on specific buffer events.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_attach()
    pub fn attach(
        &self,
        send_buffer: bool,
        opts: &BufAttachOpts,
    ) -> Result<()> {
        let mut err = nvim::Error::new();

        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        let opts = types::Dictionary::from(opts);

        let has_attached = unsafe {
            nvim_buf_attach(
                LUA_INTERNAL_CALL,
                self.0,
                send_buffer,
                #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
                opts.non_owning(),
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                opts,
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

    /// Binding to [`nvim_buf_call()`][1].
    ///
    /// Calls a function with this buffer as the temporary current buffer.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_call()
    pub fn call<F, Res, Ret>(&self, fun: F) -> Result<Ret>
    where
        F: FnOnce(()) -> Res + 'static,
        Res: IntoResult<Ret>,
        Res::Error: StdError + 'static,
        Ret: Pushable + FromObject,
    {
        let fun = Function::from_fn_once(fun);
        let mut err = nvim::Error::new();

        let obj = if cfg!(not(feature = "neovim-0-10")) {
            // Only on 0.9.
            unsafe { nvim_buf_call(self.0, fun.lua_ref(), &mut err) }
        } else {
            // On 0.10 and Nightly.
            let ref_or_nil =
                unsafe { nvim_buf_call(self.0, fun.lua_ref(), &mut err) };

            let lua_ref = match ref_or_nil.kind() {
                types::ObjectKind::LuaRef => unsafe {
                    ref_or_nil.as_luaref_unchecked()
                },
                types::ObjectKind::Nil => {
                    return Ret::from_object(Object::nil()).map_err(Into::into)
                },
                other => panic!("Unexpected object kind: {other:?}"),
            };

            unsafe {
                lua::with_state(|lstate| {
                    lua::ffi::lua_rawgeti(
                        lstate,
                        lua::ffi::LUA_REGISTRYINDEX,
                        lua_ref,
                    );
                    Object::pop(lstate)
                })
            }
            .map_err(Error::custom)?
        };

        choose!(err, {
            fun.remove_from_lua_registry();
            Ok(Ret::from_object(obj)?)
        })
    }

    /// Binding to [`nvim_buf_del_keymap()`][1].
    ///
    /// Unmaps a buffer-local mapping for the given mode.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_del_keymap()
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

    /// Binding to [`nvim_buf_del_mark()`][1].
    ///
    /// Deletes a named mark in the buffer.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_del_mark()
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

    /// Binding to [`nvim_buf_del_var()`][1].
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

    /// Binding to [`nvim_buf_delete()`][1].
    ///
    /// Deletes the buffer (not allowed while
    /// [`textlock`](https://neovim.io/doc/user/eval.html#textlock) is active).
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_delete()
    pub fn delete(self, opts: &BufDeleteOpts) -> Result<()> {
        let mut err = nvim::Error::new();

        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        let opts = types::Dictionary::from(opts);

        unsafe {
            nvim_buf_delete(
                self.0,
                #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
                opts.non_owning(),
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                opts,
                &mut err,
            )
        };

        choose!(err, ())
    }

    /// Binding to [`nvim_buf_get_changedtick()`][1].
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_changedtick()
    pub fn get_changedtick(&self) -> Result<u32> {
        let mut err = nvim::Error::new();
        let ct = unsafe { nvim_buf_get_changedtick(self.0, &mut err) };
        choose!(err, Ok(ct.try_into().expect("always positive")))
    }

    /// Binding to [`nvim_buf_get_keymap()`][1].
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_keymap()
    pub fn get_keymap(
        &self,
        mode: Mode,
    ) -> Result<impl SuperIterator<KeymapInfos>> {
        let mut err = nvim::Error::new();
        let mode = nvim::String::from(mode);
        let maps = unsafe {
            nvim_buf_get_keymap(
                self.0,
                mode.non_owning(),
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
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

    /// Binding to [`nvim_buf_get_lines()`][1].
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
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
                #[cfg(any(
                    feature = "neovim-0-9",
                    feature = "neovim-nightly"
                ))]
                // The nvim_buf_get_lines() function returns no line if we use
                // an actual lstate here.
                core::ptr::null_mut(),
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

    /// Binding to [`nvim_buf_get_mark()`][1].
    ///
    /// Returns a (1-0) indexed `(row, col)` tuple representing the position
    /// of the named mark.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_mark()
    pub fn get_mark(&self, name: char) -> Result<(usize, usize)> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let mark = unsafe {
            nvim_buf_get_mark(
                self.0,
                name.non_owning(),
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
                &mut err,
            )
        };
        choose!(err, {
            let mut iter = mark.into_iter().map(usize::from_object);
            let row = iter.next().expect("row is present")?;
            let col = iter.next().expect("col is present")?;
            Ok((row, col))
        })
    }

    /// Binding to [`nvim_buf_get_name()`][1].
    ///
    /// Returns the full filepath of the buffer.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_name()
    pub fn get_name(&self) -> Result<PathBuf> {
        let mut err = nvim::Error::new();
        let name =
            unsafe { nvim_buf_get_name(self.0, types::arena(), &mut err) };
        choose!(err, Ok(name.into()))
    }

    /// Binding to [`nvim_buf_get_offset()`][1].
    ///
    /// Returns the 0-indexed byte offset of a line.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_offset()
    pub fn get_offset(&self, index: usize) -> Result<usize> {
        let mut err = nvim::Error::new();
        let offset =
            unsafe { nvim_buf_get_offset(self.0, index as Integer, &mut err) };
        choose!(err, Ok(offset.try_into().expect("offset is positive")))
    }

    /// Binding to [`nvim_buf_get_text()`][1].
    ///
    /// Gets a range from the buffer. This differs from `Buffer::get_lines` in
    /// that it allows retrieving only portions of a line.
    ///
    /// Indexing is zero-based, with both row and column indices being
    /// end-exclusive.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_text()
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
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        let opts = types::Dictionary::from(opts);
        let (start, end) = utils::range_to_limits(line_range);
        let lines = unsafe {
            nvim_buf_get_text(
                LUA_INTERNAL_CALL,
                self.0,
                start,
                start_col.try_into()?,
                end,
                end_col.try_into()?,
                #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
                opts.non_owning(),
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                opts,
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
                #[cfg(any(
                    feature = "neovim-0-9",
                    feature = "neovim-nightly"
                ))]
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

    /// Binding to [`nvim_buf_get_var()`][1].
    ///
    /// Gets a buffer-scoped (`b:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_var()
    pub fn get_var<Var>(&self, name: &str) -> Result<Var>
    where
        Var: FromObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let obj = unsafe {
            nvim_buf_get_var(
                self.0,
                name.non_owning(),
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
                &mut err,
            )
        };
        choose!(err, Ok(Var::from_object(obj)?))
    }

    /// Binding to [`nvim_buf_is_loaded()`][1].
    ///
    /// Checks if a buffer is valid and loaded.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_is_loaded()
    pub fn is_loaded(&self) -> bool {
        unsafe { nvim_buf_is_loaded(self.0) }
    }

    /// Binding to [`nvim_buf_is_valid()`][1].
    ///
    /// Checks if a buffer is valid.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_is_valid()
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_buf_is_valid(self.0) }
    }

    /// Binding to [`nvim_buf_line_count()`][1].
    ///
    /// Returns the number of lines in the given buffer.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_line_count()
    pub fn line_count(&self) -> Result<usize> {
        let mut err = nvim::Error::new();
        let count = unsafe { nvim_buf_line_count(self.0, &mut err) };
        choose!(err, Ok(count.try_into().expect("always positive")))
    }

    /// Binding to [`nvim_buf_set_keymap()`][1].
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
        let mut err = nvim::Error::new();
        unsafe {
            nvim_buf_set_keymap(
                LUA_INTERNAL_CALL,
                self.0,
                mode.non_owning(),
                lhs.non_owning(),
                rhs.non_owning(),
                opts,
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_set_lines()`][1].
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
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_set_mark()`][1].
    ///
    /// Sets a named mark in the buffer. Marks are (1,0)-indexed, and passing 0
    /// as `line` deletes the mark.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_set_mark()
    pub fn set_mark(
        &mut self,
        name: char,
        line: usize,
        col: usize,
        opts: &SetMarkOpts,
    ) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        let opts = types::Dictionary::from(opts);
        let mark_was_set = unsafe {
            nvim_buf_set_mark(
                self.0,
                name.non_owning(),
                line.try_into()?,
                col.try_into()?,
                #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
                opts.non_owning(),
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                opts,
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

    /// Binding to [`nvim_buf_set_name()`][1].
    ///
    /// Sets the full file name for a buffer.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_set_name()
    pub fn set_name<Name: AsRef<Path>>(&mut self, name: Name) -> Result<()> {
        let name = nvim::String::from(name.as_ref());
        let mut err = nvim::Error::new();
        unsafe { nvim_buf_set_name(self.0, name.non_owning(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_set_text()`][1].
    ///
    /// Sets (replaces) a range in the buffer. Indexing is zero-based, with
    /// both row and column indices being end-exclusive.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_set_text()
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
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_buf_set_var()`][1].
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
