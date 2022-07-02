use std::fmt;

use nvim_types::{self as nvim, Array, Integer, Object, WinHandle};
use serde::{Deserialize, Serialize};

use super::ffi::window::*;
use super::{Buffer, TabPage};
use crate::lua::LuaFun;
use crate::object::{FromObject, ToObject};
use crate::Result;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Window(pub(crate) WinHandle);

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Window").field(&self.0).finish()
    }
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<H: Into<WinHandle>> From<H> for Window {
    fn from(handle: H) -> Self {
        Self(handle.into())
    }
}

impl From<Window> for Object {
    fn from(win: Window) -> Self {
        win.0.into()
    }
}

impl FromObject for Window {
    fn from_obj(obj: Object) -> Result<Self> {
        Ok(WinHandle::try_from(obj)?.into())
    }
}

impl Window {
    /// Shorthand for `nvim_oxi::api::get_current_win`.
    #[inline(always)]
    pub fn current() -> Self {
        crate::api::get_current_win()
    }

    /// Binding to `nvim_win_call`.
    ///
    /// Calls a function with this window as the temporary current window.
    pub fn call<R, F>(&self, fun: F) -> Result<R>
    where
        R: ToObject + FromObject,
        F: FnOnce(()) -> Result<R> + 'static,
    {
        let fun = LuaFun::from_fn_once(fun);
        let mut err = nvim::Error::new();
        let obj = unsafe { nvim_win_call(self.0, fun.0, &mut err) };
        err.into_err_or_flatten(move || {
            fun.unref();
            R::from_obj(obj)
        })
    }

    /// Binding to `nvim_win_close`.
    ///
    /// Closes the window. When allowed when `textlock` is active.
    // TODO: take owned `self`?
    pub fn close(&mut self, force: bool) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_close(self.0, force, &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_win_del_var`.
    ///
    /// Removes a window-scoped (w:) variable.
    pub fn del_var(&mut self, name: &str) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe { nvim_win_del_var(self.0, name.non_owning(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_win_get_buf`.
    ///
    /// Gets the current `Buffer` in the window.
    pub fn get_buf(&self) -> Result<Buffer> {
        let mut err = nvim::Error::new();
        let handle = unsafe { nvim_win_get_buf(self.0, &mut err) };
        err.into_err_or_else(|| handle.into())
    }

    /// Binding to `nvim_win_get_cursor`.
    ///
    /// Gets the (1,0)-indexed cursor position in the window.
    pub fn get_cursor(&self) -> Result<(usize, usize)> {
        let mut err = nvim::Error::new();
        let arr = unsafe { nvim_win_get_cursor(self.0, &mut err) };
        err.into_err_or_flatten(|| {
            let mut iter = arr.into_iter();
            let line = iter.next().unwrap().try_into()?;
            let col = iter.next().unwrap().try_into()?;
            Ok((line, col))
        })
    }

    /// Binding to `nvim_win_get_height`.
    ///
    /// Gets the window height as a count of rows.
    pub fn get_height(&self) -> Result<u32> {
        let mut err = nvim::Error::new();
        let height = unsafe { nvim_win_get_height(self.0, &mut err) };
        err.into_err_or_else(|| height.try_into().expect("always positive"))
    }

    /// Binding to `nvim_win_get_number`.
    ///
    /// Gets the window number.
    pub fn get_number(&self) -> Result<usize> {
        let mut err = nvim::Error::new();
        let nr = unsafe { nvim_win_get_number(self.0, &mut err) };
        err.into_err_or_else(|| nr.try_into().expect("always positive"))
    }

    /// Binding to `nvim_win_get_position`.
    ///
    /// Gets the window position in display cells.
    pub fn get_position(&self) -> Result<(usize, usize)> {
        let mut err = nvim::Error::new();
        let arr = unsafe { nvim_win_get_position(self.0, &mut err) };
        err.into_err_or_flatten(|| {
            let mut iter = arr.into_iter();
            let line = iter.next().unwrap().try_into()?;
            let col = iter.next().unwrap().try_into()?;
            Ok((line, col))
        })
    }

    /// Binding to `nvim_win_get_tabpage`.
    ///
    /// Gets the window's `TabPage`.
    pub fn get_tabpage(&self) -> Result<TabPage> {
        let mut err = nvim::Error::new();
        let handle = unsafe { nvim_win_get_tabpage(self.0, &mut err) };
        err.into_err_or_else(|| handle.into())
    }

    /// Binding to `nvim_win_get_var`.
    ///
    /// Gets a window-scoped (w:) variable.
    pub fn get_var<V>(&self, name: &str) -> Result<V>
    where
        V: FromObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let obj =
            unsafe { nvim_win_get_var(self.0, name.non_owning(), &mut err) };
        err.into_err_or_flatten(|| V::from_obj(obj))
    }

    /// Binding to `nvim_win_get_width`.
    ///
    /// Gets the window width as a number of columns.
    pub fn get_width(&self) -> Result<u32> {
        let mut err = nvim::Error::new();
        let width = unsafe { nvim_win_get_width(self.0, &mut err) };
        err.into_err_or_else(|| width.try_into().expect("always positive"))
    }

    /// Binding to `nvim_win_hide`.
    ///
    /// Closes the window and hides the buffer it contains.
    // TODO: take owned `self`?
    pub fn hide(&mut self) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_hide(self.0, &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_win_is_valid`.
    ///
    /// Checks if the window is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_win_is_valid(self.0) }
    }

    /// Binding to `nvim_win_set_buf`.
    ///
    /// Sets `buffer` as the current buffer in the window.
    pub fn set_buf(&mut self, buffer: &Buffer) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_set_buf(self.0, buffer.0, &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_win_set_cursor`.
    ///
    /// Sets the (1,0)-indexed cursor in the window. This will scroll the
    /// window even if it not the current one.
    pub fn set_cursor(&mut self, line: usize, col: usize) -> Result<()> {
        let mut err = nvim::Error::new();
        let pos = Array::from_iter([line as Integer, col as Integer]);
        unsafe { nvim_win_set_cursor(self.0, pos.non_owning(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_win_set_height`.
    ///
    /// Sets the window height.
    pub fn set_height(&mut self, height: impl Into<u32>) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_set_height(self.0, height.into().into(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_win_set_var`.
    ///
    /// Sets a window-scoped (w:) variable.
    pub fn set_var(&mut self, name: &str, value: impl ToObject) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe {
            nvim_win_set_var(
                self.0,
                name.non_owning(),
                value.to_obj()?.non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_win_set_width`.
    ///
    /// Sets the window width.
    pub fn set_width(&mut self, width: impl Into<u32>) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_set_width(self.0, width.into().into(), &mut err) };
        err.into_err_or_else(|| ())
    }
}
