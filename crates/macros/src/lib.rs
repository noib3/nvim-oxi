use proc_macro::TokenStream;
use syn::parse_macro_input;

mod derive_opts;

#[cfg(feature = "module")]
mod module;

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

// *Heavily* inspired by mlua's `lua_module` proc macro.
//
/// Marks the plugin entrypoint.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi as nvim;
///
/// #[nvim::module]
/// fn foo() -> nvim::Result<()> {
///     Ok(())
/// }
/// ```
#[cfg(feature = "module")]
#[proc_macro_attribute]
pub fn module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::ItemFn);
    module::module(item).into()
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
