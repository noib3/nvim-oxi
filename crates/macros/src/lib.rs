use proc_macro::TokenStream;
use syn::parse_macro_input;

mod derive_opts;

#[cfg(feature = "plugin")]
mod plugin;

#[cfg(feature = "test")]
mod test;

/// TODO: docs
#[proc_macro_derive(OptsBuilder, attributes(builder))]
pub fn derive_opts_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    derive_opts::expand_derive_opts_builder(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Marks the function as the entrypoint of the plugin.
///
/// The function wrapped by this macro will be called by Lua when the user
/// loads the plugin by passing its name to the `require` function. It can
/// return any type that implements [`Pushable`], and the value will be
/// returned on the Lua side by `require`.
///
/// # Examples
///
/// Let's say your crate only consists of this single `lib.rs` file:
///
/// ```ignore
/// // lib.rs
///
/// #[nvim_oxi::plugin]
/// fn my_plugin() -> u32 {
///     42
/// }
/// ```
///
/// Once the crate compiled and the resulting dynamic library is placed under
/// `lua/my_plugin.{so|dylib|dll}` somewhere in Neovim's [`runtimepath`], it
/// can be loaded with:
///
/// ```lua
/// local ret = require("my_plugin")
/// assert(ret == 42)
/// ```
///
/// [`Pushable`]: https://docs.rs/nvim-oxi/latest/nvim_oxi/lua/trait.Pushable.html
/// [`runtimepath`]: https://neovim.io/doc/user/options.html#'runtimepath'
#[cfg(feature = "plugin")]
#[proc_macro_attribute]
pub fn plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    plugin::plugin(attr, item)
}

/// Tests a piece of code inside a Neovim session.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi::{self as nvim, api};
///
/// #[nvim::test]
/// fn set_get_del_var() {
///     api::set_var("foo", 42).unwrap();
///     assert_eq!(Ok(42), api::get_var("foo"));
///     assert_eq!(Ok(()), api::del_var("foo"));
/// }
/// ```
#[cfg(feature = "test")]
#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::ItemFn);
    test::test(item).into()
}
