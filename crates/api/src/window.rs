use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;

use luajit::{self as lua, Poppable, Pushable};
use serde::{Deserialize, Serialize};
use types::{
    self as nvim,
    conversion::{self, FromObject, ToObject},
    Array,
    Function,
    Integer,
    Object,
    WinHandle,
};

use crate::choose;
use crate::ffi::window::*;
#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
use crate::opts::WinTextHeightOpts;
#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
use crate::types::WinTextHeightInfos;
use crate::Result;
use crate::{Buffer, IntoResult, TabPage};

/// A wrapper around a Neovim window handle.
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl From<&Window> for Object {
    fn from(win: &Window) -> Self {
        win.0.into()
    }
}

impl FromObject for Window {
    fn from_object(obj: Object) -> StdResult<Self, conversion::Error> {
        Ok(WinHandle::from_object(obj)?.into())
    }
}

impl Poppable for Window {
    unsafe fn pop(
        lstate: *mut lua::ffi::State,
    ) -> std::result::Result<Self, lua::Error> {
        WinHandle::pop(lstate).map(Into::into)
    }
}

impl Pushable for Window {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
    ) -> std::result::Result<std::ffi::c_int, lua::Error> {
        self.0.push(lstate)
    }
}

impl Window {
    /// Shorthand for [`get_current_win`](crate::get_current_win).
    #[inline(always)]
    pub fn current() -> Self {
        crate::get_current_win()
    }

    /// Retrieve window's underlying id/handle
    #[inline(always)]
    pub fn handle(&self) -> i32 {
        self.0
    }

    /// Binding to [`nvim_win_call()`][1].
    ///
    /// Calls a function with this window as the temporary current window.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_call()
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
            unsafe { nvim_win_call(self.0, fun.lua_ref(), &mut err) }
        } else {
            // On 0.10 and Nightly.
            let ref_or_nil =
                unsafe { nvim_win_call(self.0, fun.lua_ref(), &mut err) };

            let lua_ref = match ref_or_nil.kind() {
                types::ObjectKind::LuaRef => unsafe {
                    ref_or_nil.as_luaref_unchecked()
                },
                types::ObjectKind::Nil => {
                    return Ret::from_object(Object::nil()).map_err(Into::into)
                },
                other => panic!("Unexpected object kind: {:?}", other),
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
            .map_err(crate::Error::custom)?
        };

        choose!(err, {
            fun.remove_from_lua_registry();
            Ok(Ret::from_object(obj)?)
        })
    }

    /// Binding to [`nvim_win_close()`][1].
    ///
    /// Closes the window. Not allowed when
    /// [`textlock`](https://neovim.io/doc/user/eval.html#textlock) is active.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_close()
    pub fn close(self, force: bool) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_close(self.0, force, &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_win_del_var()`][1].
    ///
    /// Removes a window-scoped (`w:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_del_var()
    pub fn del_var(&mut self, name: &str) -> Result<()> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe { nvim_win_del_var(self.0, name.non_owning(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_win_get_buf()`][1].
    ///
    /// Gets the current [`Buffer`] in the window.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_buf()
    pub fn get_buf(&self) -> Result<Buffer> {
        let mut err = nvim::Error::new();
        let handle = unsafe { nvim_win_get_buf(self.0, &mut err) };
        choose!(err, Ok(handle.into()))
    }

    /// Binding to [`nvim_win_get_cursor()`][1].
    ///
    /// Gets the (1,0)-indexed cursor position in the window.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_cursor()
    pub fn get_cursor(&self) -> Result<(usize, usize)> {
        let mut err = nvim::Error::new();
        let arr = unsafe {
            nvim_win_get_cursor(
                self.0,
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
                &mut err,
            )
        };
        choose!(err, {
            let mut iter = arr.into_iter();
            let line = usize::from_object(iter.next().unwrap())?;
            let col = usize::from_object(iter.next().unwrap())?;
            Ok((line, col))
        })
    }

    /// Binding to [`nvim_win_get_height()`][1].
    ///
    /// Gets the window height as a count of rows.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_height()
    pub fn get_height(&self) -> Result<u32> {
        let mut err = nvim::Error::new();
        let height = unsafe { nvim_win_get_height(self.0, &mut err) };
        choose!(err, Ok(height.try_into().expect("always positive")))
    }

    /// Binding to [`nvim_win_get_number()`][1].
    ///
    /// Gets the window number.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_number()
    pub fn get_number(&self) -> Result<u32> {
        let mut err = nvim::Error::new();
        let nr = unsafe { nvim_win_get_number(self.0, &mut err) };
        choose!(err, Ok(nr.try_into().expect("always positive")))
    }

    /// Binding to [`nvim_win_get_position()`][1].
    ///
    /// Gets the window position in display cells.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_position()
    pub fn get_position(&self) -> Result<(usize, usize)> {
        let mut err = nvim::Error::new();
        let arr = unsafe {
            nvim_win_get_position(
                self.0,
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
                &mut err,
            )
        };
        choose!(err, {
            let mut iter = arr.into_iter();
            let line = usize::from_object(iter.next().unwrap())?;
            let col = usize::from_object(iter.next().unwrap())?;
            Ok((line, col))
        })
    }

    /// Binding to [`nvim_win_get_tabpage()`][1].
    ///
    /// Gets the window's `TabPage`.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_tabpage()
    pub fn get_tabpage(&self) -> Result<TabPage> {
        let mut err = nvim::Error::new();
        let handle = unsafe { nvim_win_get_tabpage(self.0, &mut err) };
        choose!(err, Ok(handle.into()))
    }

    /// Binding to [`nvim_win_get_var()`][1].
    ///
    /// Gets a window-scoped (`w:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_var()
    pub fn get_var<Var>(&self, name: &str) -> Result<Var>
    where
        Var: FromObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let obj = unsafe {
            nvim_win_get_var(
                self.0,
                name.non_owning(),
                #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
                types::arena(),
                &mut err,
            )
        };
        choose!(err, Ok(Var::from_object(obj)?))
    }

    /// Binding to [`nvim_win_get_width()`][1].
    ///
    /// Gets the window width as a number of columns.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_width()
    pub fn get_width(&self) -> Result<u32> {
        let mut err = nvim::Error::new();
        let width = unsafe { nvim_win_get_width(self.0, &mut err) };
        choose!(err, Ok(width.try_into().expect("always positive")))
    }

    /// Binding to [`nvim_win_hide()`][1].
    ///
    /// Closes the window and hides the buffer it contains.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_hide()
    pub fn hide(self) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_hide(self.0, &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_win_is_valid()`][1].
    ///
    /// Checks if the window is valid.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_is_valid()
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_win_is_valid(self.0) }
    }

    /// Binding to [`nvim_win_set_buf()`][1].
    ///
    /// Sets `buffer` as the current buffer in the window.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_set_buf()
    pub fn set_buf(&mut self, buffer: &Buffer) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_set_buf(self.0, buffer.0, &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_win_set_cursor()`][1].
    ///
    /// Sets the (1,0)-indexed cursor in the window. This will scroll the
    /// window even if it's not the current one.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_set_cursor()
    pub fn set_cursor(&mut self, line: usize, col: usize) -> Result<()> {
        let mut err = nvim::Error::new();
        let pos = Array::from_iter([line as Integer, col as Integer]);
        unsafe { nvim_win_set_cursor(self.0, pos.non_owning(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_win_set_height()`][1].
    ///
    /// Sets the window height.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_set_height()
    pub fn set_height(&mut self, height: u32) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_set_height(self.0, height.into(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_win_set_hl_ns()`][1].
    ///
    /// Sets the highlight namespace for this window. This will the highlights
    /// defined with [`set_hl`](crate::set_hl) for the given namespace, but
    /// fall back to global highlights (`ns_id = 0`) if those are missing.
    ///
    /// This takes precedence over the `winhighlight` option.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_set_hl_ns()
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-10", feature = "neovim-nightly")))
    )]
    pub fn set_hl_ns(&mut self, ns_id: u32) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_set_hl_ns(self.0, ns_id.into(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_win_set_var()`][1].
    ///
    /// Sets a window-scoped (`w:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_set_var()
    pub fn set_var<Var>(&mut self, name: &str, value: Var) -> Result<()>
    where
        Var: ToObject,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe {
            nvim_win_set_var(
                self.0,
                name.non_owning(),
                value.to_object()?.non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }

    /// Binding to [`nvim_win_set_width()`][1].
    ///
    /// Sets the window width.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_set_width()
    pub fn set_width(&mut self, width: u32) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_set_width(self.0, width.into(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_win_text_height()`][1].
    ///
    /// Computes the number of screen lines occupied by a range of text in a
    /// given window. Works for off-screen text and takes folds into account.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_text_height()
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-10", feature = "neovim-nightly")))
    )]
    pub fn text_height(
        &self,
        opts: &WinTextHeightOpts,
    ) -> Result<WinTextHeightInfos> {
        let mut err = nvim::Error::new();
        let dict = unsafe {
            nvim_win_text_height(self.0, opts, types::arena(), &mut err)
        };
        choose!(err, dict.try_into().map_err(Into::into))
    }
}
