use std::fmt;

use nvim_types::{
    Error as NvimError,
    Object,
    String as NvimString,
    WinHandle,
};
use serde::{Deserialize, Serialize};

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
}
