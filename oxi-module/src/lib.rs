use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Error};

// *Heavily* inspired by mlua's `lua_module` proc macro.
//
/// Marks the plugin entrypoint.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi as oxi;
///
/// #[oxi::module]
/// fn foo() -> oxi::Result<()> {
///     Ok(())
/// }
/// ```
///
/// the compiled library can then be loaded from Neovim with `require("foo")`.
#[proc_macro_attribute]
pub fn oxi_module(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);

    if !args.is_empty() {
        return Error::new(Span::call_site(), "no attributes are supported")
            .to_compile_error()
            .into();
    }

    let item = parse_macro_input!(item as syn::ItemFn);

    let module_name = item.sig.ident.clone();
    let lua_module =
        Ident::new(&format!("luaopen_{module_name}"), Span::call_site());

    let module_body = quote! {
        #item

        #[no_mangle]
        unsafe extern "C" fn #lua_module(
            state: *mut ::nvim_oxi::lua::lua_State,
        ) -> ::std::os::raw::c_int {
            ::nvim_oxi::lua::module_entrypoint(state, #module_name)
        }
    };

    module_body.into()
}
