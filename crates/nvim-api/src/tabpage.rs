use std::fmt;

use luajit_bindings::{self as lua, Poppable, Pushable};
use nvim_types::{
    self as nvim,
    FromObject,
    FromObjectResult,
    Object,
    TabHandle,
    ToObject,
};
use serde::{Deserialize, Serialize};

use super::ffi::tabpage::*;
use super::Window;
use crate::iterator::SuperIterator;
use crate::Result;

/// A newtype struct wrapping a Neovim tabpage. All the `nvim_tabpage_*`
/// functions taking a tabpage handle as their first argument are implemented
/// as methods on this object.
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl Poppable for TabPage {
    unsafe fn pop(
        lstate: *mut lua::ffi::lua_State,
    ) -> std::result::Result<Self, lua::Error> {
        TabHandle::pop(lstate).map(Into::into)
    }
}

impl Pushable for TabPage {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::lua_State,
    ) -> std::result::Result<std::ffi::c_int, lua::Error> {
        self.0.push(lstate)
    }
}

impl FromObject for TabPage {
    fn from_obj(obj: Object) -> FromObjectResult<Self> {
        Ok(TabHandle::from_obj(obj)?.into())
    }
}

impl TabPage {
    /// Shorthand for
    /// [`api::get_current_tabpage`](crate::api::get_current_tabpage).
    #[inline(always)]
    pub fn current() -> Self {
        crate::get_current_tabpage()
    }

    /// Binding to [`nvim_tabpage_del_var`](https://neovim.io/doc/user/api.html#nvim_tabpage_del_var()).
    ///
    /// Removes a tab-scoped (`t:`) variable.
    pub fn del_var(&mut self, name: &str) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe { nvim_tabpage_del_var(self.0, name.non_owning(), &mut err) };
        err.into_err_or_else(|| ())
    }

    /// Binding to [`nvim_tabpage_get_number`](https://neovim.io/doc/user/api.html#nvim_tabpage_get_number()).
    ///
    /// Gets the tabpage number.
    pub fn get_number(&self) -> Result<u32> {
        let mut err = nvim::Error::new();
        let number = unsafe { nvim_tabpage_get_number(self.0, &mut err) };
        err.into_err_or_else(|| number.try_into().expect("always positive"))
    }

    /// Binding to [`nvim_tabpage_get_var`](https://neovim.io/doc/user/api.html#nvim_tabpage_get_var()).
    ///
    /// Gets a tab-scoped (`t:`) variable.
    pub fn get_var<Var>(&self, name: &str) -> Result<Var>
    where
        Var: FromObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let obj = unsafe {
            nvim_tabpage_get_var(self.0, name.non_owning(), &mut err)
        };
        err.into_err_or_flatten(|| Ok(Var::from_obj(obj)?))
    }

    /// Binding to [`nvim_tabpage_get_win`](https://neovim.io/doc/user/api.html#nvim_tabpage_get_win()).
    ///
    /// Gets the current window in a tabpage.
    pub fn get_win(&self) -> Result<Window> {
        let mut err = nvim::Error::new();
        let handle = unsafe { nvim_tabpage_get_win(self.0, &mut err) };
        err.into_err_or_else(|| handle.into())
    }

    /// Binding to [`nvim_tabpage_is_valid`](https://neovim.io/doc/user/api.html#nvim_tabpage_is_valid()).
    ///
    /// Checks if a tabpage is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_tabpage_is_valid(self.0) }
    }

    /// Binding to [`nvim_tabpage_list_wins`](https://neovim.io/doc/user/api.html#nvim_tabpage_list_wins()).
    ///
    /// Gets the windows in a tabpage.
    pub fn list_wins(&self) -> Result<impl SuperIterator<Window>> {
        let mut err = nvim::Error::new();
        let list = unsafe { nvim_tabpage_list_wins(self.0, &mut err) };
        err.into_err_or_else(|| {
            list.into_iter().map(|obj| Window::from_obj(obj).unwrap())
        })
    }

    /// Binding to [`nvim_tabpage_set_var`](https://neovim.io/doc/user/api.html#nvim_tabpage_set_var()).
    ///
    /// Sets a tab-scoped (`t:`) variable.
    pub fn set_var<Var>(&mut self, name: &str, value: Var) -> Result<()>
    where
        Var: ToObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
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
