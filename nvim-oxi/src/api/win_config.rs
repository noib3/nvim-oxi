use nvim_types as nvim;

use super::ffi::win_config::*;
use super::types::*;
use super::{Buffer, Window};
use crate::object::FromObject;
use crate::Result;

/// Binding to `nvim_open_win`.
///
/// Opens a new floating or external window.
pub fn open_win(
    buf: impl Into<Buffer>,
    enter: bool,
    config: &WindowConfig,
) -> Result<Window> {
    let mut err = nvim::Error::new();
    let handle = unsafe {
        nvim_open_win(buf.into().0, enter, &config.into(), &mut err)
    };
    err.into_err_or_else(|| handle.into())
}

impl Window {
    /// Binding to `nvim_win_get_config`.
    ///
    /// Gets the window configuration.
    pub fn get_config(&self) -> Result<WindowConfig> {
        let mut err = nvim::Error::new();
        let mut dict = unsafe { nvim_win_get_config(self.0, &mut err) };
        let win = dict.get(&"win").map(|obj| unsafe {
            // SAFETY: if the `win` key is present it's set to an integer
            // representing a window handle.
            obj.data.integer as i32
        });
        if let Some(handle) = win {
            dict["relative"] = handle.into();
        }
        err.into_err_or_flatten(|| WindowConfig::from_obj(dict.into()))
    }

    /// Binding to `nvim_win_get_config`.
    ///
    /// Configures the window layout. Only for floating and external windows.
    pub fn set_config(&mut self, config: &WindowConfig) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_set_config(self.0, &config.into(), &mut err) };
        err.into_err_or_else(|| ())
    }
}
