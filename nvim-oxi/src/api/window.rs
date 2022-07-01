use std::fmt;

use nvim_types::{self as types, Array, Integer, Object, WinHandle};
use serde::{Deserialize, Serialize};

use super::ffi::window::*;
use crate::object::FromObject;
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

    /// Binding to `nvim_win_get_cursor`.
    ///
    /// Gets the (1,0)-indexed cursor position in the window.
    pub fn get_cursor(&self) -> Result<(usize, usize)> {
        let mut err = types::Error::new();
        let arr = unsafe { nvim_win_get_cursor(self.0, &mut err) };
        err.into_err_or_flatten(|| {
            let mut iter = arr.into_iter();
            let line = iter.next().unwrap().try_into()?;
            let col = iter.next().unwrap().try_into()?;
            Ok((line, col))
        })
    }

    /// Binding to `nvim_win_set_cursor`.
    ///
    /// Sets the (1,0)-indexed cursor in the window. This will scroll the
    /// window even if it not the current one.
    pub fn set_cursor(&mut self, line: usize, col: usize) -> Result<()> {
        let mut err = types::Error::new();
        let pos = Array::from_iter([line as Integer, col as Integer]);
        unsafe { nvim_win_set_cursor(self.0, pos.non_owning(), &mut err) };
        err.into_err_or_else(|| ())
    }
}
