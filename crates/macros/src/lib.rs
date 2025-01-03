use proc_macro::TokenStream;
use syn::parse_macro_input;

mod common;
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

/// Marks a function as the entrypoint of the plugin.
///
/// The function wrapped by this macro will be called by Neovim when the user
/// loads the plugin by passing its name to the `require` function. It can
/// return any type that implements the [`Pushable`] trait, and the value will
/// be returned on the Lua side by `require`.
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
/// Once the crate is compiled and the resulting dynamic library is placed
/// under `lua/my_plugin.{so|dylib|dll}` somewhere in Neovim's [`runtimepath`],
/// it can be loaded with:
///
/// ```lua
/// local ret = require("my_plugin")
/// assert(ret == 42)
/// ```
///
/// [`Pushable`]: https://docs.rs/nvim-oxi/latest/nvim_oxi/lua/trait.Pushable.html
/// [`runtimepath`]: https://neovim.io/doc/user/options.html#'runtimepath'
///
/// # Attributes
///
/// ## `nvim-oxi`
///
/// The code generated by this macro includes calls to functions defined in the
/// `nvim-oxi` crate, which is expected to be in scope under `::nvim_oxi`. This
/// can cause problems if you renamed the crate in your `Cargo.toml` or if it's
/// re-exported from another crate.
///
/// In these cases, you can use the `nvim_oxi` attribute to specify the path to
/// `nvim-oxi`.
///
/// For example, let's say your crate has a single dependency called `foo`
/// whose whose `lib.rs` re-exports `nvim-oxi` as `nvim`:
///
/// ```ignore
/// // foo's lib.rs
/// pub use nvim_oxi as nvim;
/// ```
///
/// Doing this would generate a compilation error because `nvim_oxi` is not in
/// scope:
///
/// ```compile_fail
/// #[foo::nvim::plugin]
/// fn my_plugin() {}
/// ```
///
/// To fix this, you can use the `nvim_oxi` attribute to specify the correct
/// path:
///
/// ```ignore
/// #[foo::nvim::plugin(nvim_oxi = foo::nvim)]
/// fn my_plugin() {}
/// ```
#[cfg(feature = "plugin")]
#[proc_macro_attribute]
pub fn plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    plugin::plugin(attr, item)
}

/// Tests a piece of code from inside Neovim.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi::api;
///
/// #[nvim_oxi::test]
/// fn set_get_del_var() {
///     api::set_var("foo", 42).unwrap();
///     assert_eq!(Ok(42), api::get_var("foo"));
///     assert_eq!(Ok(()), api::del_var("foo"));
/// }
/// ```
///
/// The test function can also return a `Result<(), T>` if `T` implements
/// `Debug`:
///
/// ```ignore
/// # use nvim_oxi::api;
/// #[nvim_oxi::test]
/// fn print_42() -> Result<(), api::Error> {
///     api::command("lua print(42)")
/// }
/// ```
///
/// # Attributes
///
/// ## `nvim-oxi`
///
/// Exactly the same as the `nvim-oxi` attribute on the [`macro@plugin`] macro.
/// See [its documentation](macro@plugin#nvim-oxi) for more information.
///
/// ## `cmd`
///
/// The `cmd` attribute is used to specify an Ex command that will be executed
/// by Neovim before the test's body. This can be useful to configure the
/// environment in which the test will run.
///
/// ```ignore
/// # use nvim_oxi::api;
/// #[nvim_oxi::test(cmd = "lua print('The answer is...')")]
/// fn print_42() -> Result<(), api::Error> {
///     api::command("lua print(42)")
/// }
/// ```
///
/// If the given string spans multiple lines, it will be joined into a single
/// line using `;` as the separator.
/// ```
#[cfg(feature = "test")]
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    test::test(attr, item)
}
