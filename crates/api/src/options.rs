use types::{
    self as nvim,
    conversion::{FromObject, ToObject},
};

use crate::choose;
use crate::ffi::options::*;
use crate::opts::*;
use crate::types::*;
use crate::Result;
use crate::SuperIterator;

/// Binding to [`nvim_get_all_options_info()`][1].
///
/// Gets the option information for all options.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_all_options_info()
pub fn get_all_options_info() -> Result<impl SuperIterator<OptionInfos>> {
    let mut err = nvim::Error::new();
    let infos = unsafe {
        nvim_get_all_options_info(
            #[cfg(feature = "neovim-nightly")]
            types::arena(),
            &mut err,
        )
    };
    choose!(
        err,
        Ok({
            infos
                .into_iter()
                .map(|(_, optinf)| OptionInfos::from_object(optinf).unwrap())
        })
    )
}

/// Binding to [`nvim_get_option_info2()`][1].
///
/// Gets the option information for one option from an arbitrary buffer or
/// window.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_option_info2()
#[cfg(feature = "neovim-nightly")]
#[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
pub fn get_option_info2(
    name: &str,
    opts: &OptionValueOpts,
) -> Result<OptionInfos> {
    let name = types::String::from(name);
    let mut err = types::Error::new();
    let dict = unsafe {
        nvim_get_option_info2(
            name.non_owning(),
            opts,
            types::arena(),
            &mut err,
        )
    };
    choose!(err, Ok(OptionInfos::from_object(dict.into())?))
}

/// Binding to [`nvim_get_option_value()`][1].
///
/// Gets the local value of an option if it exists, or the global value
/// otherwise. Local values always correspond to the current buffer or window.
///
/// To get a buffer-local orr window-local option for a specific buffer of
/// window consider using [`Buffer::get_option`] or [`Window::get_option`] instead.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_option_value()
pub fn get_option_value<Opt>(name: &str, opts: &OptionValueOpts) -> Result<Opt>
where
    Opt: FromObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let obj =
        unsafe { nvim_get_option_value(name.non_owning(), opts, &mut err) };
    choose!(err, Ok(Opt::from_object(obj)?))
}

/// Binding to [`nvim_set_option_value()`][1].
///
/// Sets the value of an option. The behaviour of this function matches that of
/// `:set`: for global-local options, both the global and local value are set
/// unless specified otherwise in the [`scope`](OptionValueOptsBuilder::scope)
/// field of `opts`.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_option_value()
pub fn set_option_value<Opt>(
    name: &str,
    value: Opt,
    opts: &OptionValueOpts,
) -> Result<()>
where
    Opt: ToObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_option_value(
            #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
            crate::LUA_INTERNAL_CALL,
            name.non_owning(),
            value.to_object()?.non_owning(),
            opts,
            &mut err,
        )
    };
    choose!(err, ())
}
