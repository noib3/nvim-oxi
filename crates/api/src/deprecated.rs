use types::conversion::{FromObject, ToObject};
use types::{Dictionary, Integer, Object};

use crate::LUA_INTERNAL_CALL;
use crate::Result;
use crate::choose;
use crate::ffi::deprecated::*;
use crate::types::*;
use crate::{Buffer, Window};

/// Binding to [`nvim_exec()`][1].
///
/// Executes a multiline block of Ex commands. If `output` is true the
/// output is captured and returned.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_exec()
#[cfg_attr(
    feature = "neovim-0-11", // On 0.11 and Nightly.
    deprecated(since = "0.5.0", note = "use `exec2` instead")
)]
pub fn exec(src: &str, output: bool) -> Result<Option<String>> {
    let src = types::String::from(src);
    let mut err = types::Error::new();
    let output = unsafe {
        nvim_exec(LUA_INTERNAL_CALL, src.as_nvim_str(), output, &mut err)
    };
    choose!(err, {
        Ok((!output.is_empty()).then(|| output.to_string_lossy().into()))
    })
}

/// Binding to [`nvim_get_hl_by_id()`][1].
///
/// Gets a highlight definition by id.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_hl_by_id[1]
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_win()
#[cfg_attr(
    feature = "neovim-0-11", // On 0.11 and Nightly.
    deprecated(since = "0.5.0", note = "use `get_hl` instead")
)]
pub fn get_hl_by_id(hl_id: u32, rgb: bool) -> Result<HighlightInfos> {
    let mut err = types::Error::new();

    let hl = unsafe {
        nvim_get_hl_by_id(hl_id.into(), rgb, core::ptr::null_mut(), &mut err)
    };

    choose!(err, Ok(HighlightInfos::from_object(hl.into())?))
}

/// Binding to [`nvim_get_hl_by_name()`][1].
///
/// Gets a highlight definition by name.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_hl_by_name()
#[cfg_attr(
    feature = "neovim-0-11", // On 0.11 and Nightly.
    deprecated(since = "0.5.0", note = "use `get_hl` instead")
)]
pub fn get_hl_by_name(name: &str, rgb: bool) -> Result<HighlightInfos> {
    let name = types::String::from(name);
    let mut err = types::Error::new();
    let hl = unsafe {
        nvim_get_hl_by_name(
            name.as_nvim_str(),
            rgb,
            core::ptr::null_mut(),
            &mut err,
        )
    };
    choose!(err, Ok(HighlightInfos::from_object(hl.into())?))
}

/// Binding to [`nvim_get_option()`][1].
///
/// Gets the value of a global option.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_option()
#[cfg_attr(
    feature = "neovim-0-11", // On 0.11 and Nightly.
    deprecated(since = "0.5.0", note = "use `get_option_value` instead")
)]
pub fn get_option<Opt>(name: &str) -> Result<Opt>
where
    Opt: FromObject,
{
    let name = types::String::from(name);
    let mut err = types::Error::new();
    let obj = unsafe { nvim_get_option(name.as_nvim_str(), &mut err) };
    choose!(err, Ok(Opt::from_object(obj)?))
}

/// Binding to [`nvim_get_option_info()`][1].
///
/// Gets all the informations related to an option.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_option_info()
#[cfg_attr(
    feature = "neovim-0-11", // On 0.11 and Nightly.
    deprecated(since = "0.5.0", note = "use `get_option_info2` instead")
)]
pub fn get_option_info(name: &str) -> Result<OptionInfos> {
    let name = types::String::from(name);
    let mut err = types::Error::new();
    let obj = unsafe {
        nvim_get_option_info(name.as_nvim_str(), types::arena(), &mut err)
    };
    choose!(err, Ok(OptionInfos::from_object(obj.into())?))
}

/// Binding to [`nvim_notify()`][1].
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_notify()
#[cfg_attr(
    feature = "neovim-0-11", // On 0.11 and Nightly.
    deprecated(since = "0.6.0", note = "use `echo` instead")
)]
pub fn notify(
    msg: &str,
    log_level: LogLevel,
    opts: &Dictionary,
) -> Result<Object> {
    let msg = types::String::from(msg);
    let mut err = types::Error::new();
    let obj = unsafe {
        nvim_notify(
            msg.as_nvim_str(),
            log_level as Integer,
            opts.non_owning(),
            types::arena(),
            &mut err,
        )
    };
    choose!(err, Ok(obj))
}

/// Binding to [`nvim_set_option()`][1].
///
/// Sets the global value of an option.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_option()
#[cfg_attr(
    feature = "neovim-0-11", // On 0.11 and Nightly.
    deprecated(since = "0.5.0", note = "use `set_option_value` instead")
)]
pub fn set_option<Opt>(name: &str, value: Opt) -> Result<()>
where
    Opt: ToObject,
{
    let name = types::String::from(name);
    let mut err = types::Error::new();
    unsafe {
        nvim_set_option(
            LUA_INTERNAL_CALL,
            name.as_nvim_str(),
            value.to_object()?.non_owning(),
            &mut err,
        )
    };
    choose!(err, ())
}

impl Buffer {
    /// Binding to [`nvim_buf_get_option()`][1].
    ///
    /// Gets a buffer option value.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_get_option()
    #[cfg_attr(
        feature = "neovim-0-11", // On 0.11 and Nightly.
        deprecated(since = "0.5.0", note = "use `get_option_value` instead")
    )]
    pub fn get_option<Opt>(&self, name: &str) -> Result<Opt>
    where
        Opt: FromObject,
    {
        let mut err = types::Error::new();
        let name = types::String::from(name);
        let obj = unsafe {
            nvim_buf_get_option(self.0, name.as_nvim_str(), &mut err)
        };
        choose!(err, Ok(Opt::from_object(obj)?))
    }

    /// Binding to [`nvim_buf_set_option()`][1].
    ///
    /// Sets a buffer option value. Passing `None` as value deletes the option
    /// (only works if there's a global fallback).
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_buf_set_option()
    #[cfg_attr(
        feature = "neovim-0-11", // On 0.11 and Nightly.
        deprecated(since = "0.5.0", note = "use `set_option_value` instead")
    )]
    pub fn set_option<V>(&mut self, name: &str, value: V) -> Result<()>
    where
        V: ToObject,
    {
        let mut err = types::Error::new();
        let name = types::String::from(name);
        unsafe {
            nvim_buf_set_option(
                LUA_INTERNAL_CALL,
                self.0,
                name.as_nvim_str(),
                value.to_object()?.non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }
}

impl Window {
    /// Binding to [`nvim_win_get_option()`][1].
    ///
    /// Gets a window option value.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_option()
    #[cfg_attr(
        feature = "neovim-0-11", // On 0.11 and Nightly.
        deprecated(since = "0.5.0", note = "use `get_option_value` instead")
    )]
    pub fn get_option<Opt>(&self, name: &str) -> Result<Opt>
    where
        Opt: FromObject,
    {
        let mut err = types::Error::new();
        let name = types::String::from(name);
        let obj = unsafe {
            nvim_win_get_option(self.0, name.as_nvim_str(), &mut err)
        };
        choose!(err, Ok(Opt::from_object(obj)?))
    }

    /// Binding to [`nvim_win_set_option()`][1].
    ///
    /// Sets a window option value. Passing `None` as value deletes the option
    /// (only works if there's a global fallback).
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_set_option()
    #[cfg_attr(
        feature = "neovim-0-11", // On 0.11 and Nightly.
        deprecated(since = "0.5.0", note = "use `set_option_value` instead")
    )]
    pub fn set_option<Opt>(&mut self, name: &str, value: Opt) -> Result<()>
    where
        Opt: ToObject,
    {
        let mut err = types::Error::new();
        let name = types::String::from(name);
        unsafe {
            nvim_win_set_option(
                LUA_INTERNAL_CALL,
                self.0,
                name.as_nvim_str(),
                value.to_object()?.non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }
}
