use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, parse_quote, ItemFn, Path, Token};

use crate::common::{DuplicateError, KeyedAttribute};

#[inline]
pub fn plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as Attributes);

    let entrypoint = parse_macro_input!(item as ItemFn);

    let plugin_name = &entrypoint.sig.ident;

    let lua_module =
        Ident::new(&format!("luaopen_{plugin_name}"), Span::call_site());

    let nvim_oxi = attrs.nvim_oxi;

    quote! {
        #entrypoint

        #[no_mangle]
        unsafe extern "C" fn #lua_module(
            state: *mut #nvim_oxi::lua::ffi::lua_State,
        ) -> ::core::ffi::c_int {
            #nvim_oxi::entrypoint::entrypoint(state, #plugin_name)
        }
    }
    .into()
}

#[derive(Default)]
struct Attributes {
    nvim_oxi: NvimOxi,
}

impl Parse for Attributes {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        let mut has_parsed_nvim_oxi = false;

        while !input.is_empty() {
            let keypair = input.parse::<Attribute>()?;

            match keypair {
                Attribute::NvimOxi(nvim_oxi) => {
                    if has_parsed_nvim_oxi {
                        return Err(DuplicateError(nvim_oxi).into());
                    }
                    this.nvim_oxi = nvim_oxi;
                    has_parsed_nvim_oxi = true;
                },
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(this)
    }
}

enum Attribute {
    NvimOxi(NvimOxi),
}

impl Parse for Attribute {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<NvimOxi>().map(Self::NvimOxi)
    }
}

pub(crate) struct NvimOxi {
    key_span: Span,
    value: Path,
}

impl Default for NvimOxi {
    #[inline]
    fn default() -> Self {
        Self { key_span: Span::call_site(), value: parse_quote!(::nvim_oxi) }
    }
}

impl Parse for NvimOxi {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // First, lookahead to see if the key is ours.
        if input.fork().parse::<Ident>()? != Self::KEY {
            return Err(input.error("invalid attribute"));
        }

        let _key = input.parse::<Ident>().expect("just checked");
        let _eq = input.parse::<Token![=]>()?;
        let value = input.parse::<Path>()?;
        Ok(Self { key_span: Span::call_site(), value })
    }
}

impl KeyedAttribute for NvimOxi {
    const KEY: &'static str = "nvim_oxi";

    #[inline]
    fn key_span(&self) -> Span {
        self.key_span
    }
}

impl ToTokens for NvimOxi {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.value.to_tokens(tokens);
    }
}
