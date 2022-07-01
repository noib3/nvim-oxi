use std::fmt;

use nvim_types::{
    Error as NvimError,
    Object,
    String as NvimString,
    TabHandle,
    WinHandle,
};
use serde::{Deserialize, Serialize};

use super::ffi::tabpage::*;
use crate::object::{FromObject, ToObject};
use crate::Result;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TabPage(pub(crate) TabHandle);

impl fmt::Debug for TabPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("TabPage").field(&self.0).finish()
    }
}

impl fmt::Display for TabPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<H: Into<TabHandle>> From<H> for TabPage {
    fn from(handle: H) -> Self {
        Self(handle.into())
    }
}

impl From<TabPage> for Object {
    fn from(tabpage: TabPage) -> Self {
        tabpage.0.into()
    }
}

impl FromObject for TabPage {
    fn from_obj(obj: Object) -> Result<Self> {
        Ok(TabHandle::try_from(obj)?.into())
    }
}

impl TabPage {
    /// Shorthand for `nvim_oxi::api::get_current_tabpage`.
    #[inline(always)]
    pub fn current() -> Self {
        crate::api::get_current_tabpage()
    }

    /// Binding to `nvim_tabpage_del_var`.
    ///
    /// Removes a tab-scoped (t:) variable.
    pub fn del_var(&mut self, name: &str) -> Result<()> {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        unsafe { nvim_tabpage_del_var(self.0, name.non_owning(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to `nvim_tabpage_get_number`.
    ///
    /// Gets the tabpage number.
    pub fn get_number(&self) -> Result<usize> {
        let mut err = NvimError::new();
        let number = unsafe { nvim_tabpage_get_number(self.0, &mut err) };
        err.into_err_or_else(|| number.try_into().expect("always positive"))
    }

    /// Binding to `nvim_tabpage_get_var`.
    ///
    /// Gets a tab-scoped (t:) variable.
    pub fn get_var<Value>(&self, name: &str) -> Result<Value>
    where
        Value: FromObject,
    {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        let obj = unsafe {
            nvim_tabpage_get_var(self.0, name.non_owning(), &mut err)
        };
        err.into_err_or_flatten(|| Value::from_obj(obj))
    }

    /// Binding to `nvim_tabpage_get_win`.
    ///
    /// Gets the current window in a tabpage.
    // TODO: return `Window` when that's implemented
    pub fn get_win(&self) -> Result<WinHandle> {
        let mut err = NvimError::new();
        let handle = unsafe { nvim_tabpage_get_win(self.0, &mut err) };
        err.into_err_or_else(|| handle)
    }

    /// Binding to `nvim_tabpage_is_valid`.
    ///
    /// Checks if a tabpage is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_tabpage_is_valid(self.0) }
    }

    /// Binding to `nvim_tabpage_list_wins`.
    ///
    /// Gets the windows in a tabpage.
    // TODO: return `Window` when that's implemented
    pub fn list_wins(&self) -> Result<impl Iterator<Item = WinHandle>> {
        let mut err = NvimError::new();
        let list = unsafe { nvim_tabpage_list_wins(self.0, &mut err) };
        err.into_err_or_else(|| {
            list.into_iter().map(|win| {
                WinHandle::try_from(win).expect("object is a window handle")
            })
        })
    }

    /// Binding to `nvim_tabpage_set_var`.
    ///
    /// Sets a tab-scoped (t:) variable.
    pub fn set_var(&mut self, name: &str, value: impl ToObject) -> Result<()> {
        let mut err = NvimError::new();
        let name = NvimString::from(name);
        unsafe {
            nvim_tabpage_set_var(
                self.0,
                name.non_owning(),
                value.to_obj()?.non_owning(),
                &mut err,
            )
        };
        err.into_err_or_else(|| ())
    }
}
