use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::ItemFn;

#[inline]
pub fn plugin(entrypoint: ItemFn) -> TokenStream {
    let module_name = &entrypoint.sig.ident;

    let lua_module =
        Ident::new(&format!("luaopen_{module_name}"), Span::call_site());

    quote! {
        #entrypoint

        #[no_mangle]
        unsafe extern "C" fn #lua_module(
            state: *mut ::nvim_oxi::lua::ffi::lua_State,
        ) -> ::core::ffi::c_int {
            ::nvim_oxi::entrypoint::entrypoint(state, #module_name)
        }
    }
}
