use std::fmt;
use std::result::Result as StdResult;

use luajit::{self as lua, Poppable, Pushable};
use serde::{Deserialize, Serialize};
use types::{
    self as nvim,
    Object,
    TabHandle,
    conversion::{self, FromObject, ToObject},
};

use crate::Result;
use crate::SuperIterator;
use crate::Window;
use crate::choose;
use crate::ffi::tabpage::*;

/// A wrapper around a Neovim tab handle.
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
        lstate: *mut lua::ffi::State,
    ) -> std::result::Result<Self, lua::Error> {
        TabHandle::pop(lstate).map(Into::into)
    }
}

impl Pushable for TabPage {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
    ) -> std::result::Result<std::ffi::c_int, lua::Error> {
        self.0.push(lstate)
    }
}

impl FromObject for TabPage {
    fn from_object(obj: Object) -> StdResult<Self, conversion::Error> {
        Ok(TabHandle::from_object(obj)?.into())
    }
}

#[cfg(feature = "mlua")]
impl mlua::IntoLua for TabPage {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        self.handle().into_lua(lua)
    }
}

#[cfg(feature = "mlua")]
impl mlua::FromLua for TabPage {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        TabHandle::from_lua(value, lua).map(Into::into)
    }
}

impl TabPage {
    /// Shorthand for [`get_current_tabpage`](crate::get_current_tabpage).
    #[inline(always)]
    pub fn current() -> Self {
        crate::get_current_tabpage()
    }

    /// Retrieve tabpage's underlying id/handle
    #[inline(always)]
    pub fn handle(&self) -> i32 {
        self.0
    }

    /// Binding to [`nvim_tabpage_del_var()`][1].
    ///
    /// Removes a tab-scoped (`t:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_tabpage_del_var()
    pub fn del_var(&mut self, name: &str) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe { nvim_tabpage_del_var(self.0, name.as_nvim_str(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_tabpage_get_number()`][1].
    ///
    /// Gets the tabpage number.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_tabpage_get_number()
    pub fn get_number(&self) -> Result<u32> {
        let mut err = nvim::Error::new();
        let number = unsafe { nvim_tabpage_get_number(self.0, &mut err) };
        choose!(err, Ok(number.try_into().expect("always positive")))
    }

    /// Binding to [`nvim_tabpage_get_var()`][1].
    ///
    /// Gets a tab-scoped (`t:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_tabpage_get_var()
    pub fn get_var<Var>(&self, name: &str) -> Result<Var>
    where
        Var: FromObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let obj = unsafe {
            nvim_tabpage_get_var(
                self.0,
                name.as_nvim_str(),
                types::arena(),
                &mut err,
            )
        };
        choose!(err, Ok(Var::from_object(obj)?))
    }

    /// Binding to [`nvim_tabpage_get_win()`][1].
    ///
    /// Gets the current window in a tabpage.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_tabpage_get_win()
    pub fn get_win(&self) -> Result<Window> {
        let mut err = nvim::Error::new();
        let handle = unsafe { nvim_tabpage_get_win(self.0, &mut err) };
        choose!(err, Ok(handle.into()))
    }

    /// Binding to [`nvim_tabpage_is_valid()`][1].
    ///
    /// Checks if a tabpage is valid.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_tabpage_is_valid()
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_tabpage_is_valid(self.0) }
    }

    /// Binding to [`nvim_tabpage_list_wins()`][1].
    ///
    /// Gets the windows in a tabpage.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_tabpage_list_wins()
    pub fn list_wins(&self) -> Result<impl SuperIterator<Window> + use<>> {
        let mut err = nvim::Error::new();
        let list = unsafe {
            nvim_tabpage_list_wins(self.0, types::arena(), &mut err)
        };
        choose!(
            err,
            Ok({
                list.into_iter().map(|obj| Window::from_object(obj).unwrap())
            })
        )
    }

    /// Binding to [`nvim_tabpage_set_var()`][1].
    ///
    /// Sets a tab-scoped (`t:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_tabpage_set_var()
    pub fn set_var<Var>(&mut self, name: &str, value: Var) -> Result<()>
    where
        Var: ToObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe {
            nvim_tabpage_set_var(
                self.0,
                name.as_nvim_str(),
                value.to_object()?.non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_tabpage_set_win()`][1].
    ///
    /// Sets the current window in the tabpage.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_tabpage_set_win()
    pub fn set_win(&mut self, win: &Window) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_tabpage_set_win(self.0, win.0, &mut err) };
        choose!(err, ())
    }
}
