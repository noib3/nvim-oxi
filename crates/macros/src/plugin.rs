use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, parse_quote, ItemFn, Path, Token};

#[inline]
pub fn plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as PluginAttributes);

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
struct PluginAttributes {
    nvim_oxi: NvimOxi,
}

impl Parse for PluginAttributes {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Self::default();

        let mut has_parsed_nvim_oxi = false;

        while !input.is_empty() {
            let keypair = input.parse::<AttributeKeyPair>()?;

            match keypair {
                AttributeKeyPair::NvimOxi(nvim_oxi) => {
                    if has_parsed_nvim_oxi {
                        return Err(DuplicateError(nvim_oxi).into());
                    }
                    attrs.nvim_oxi = nvim_oxi;
                    has_parsed_nvim_oxi = true;
                },
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(attrs)
    }
}

enum AttributeKeyPair {
    NvimOxi(NvimOxi),
}

impl Parse for AttributeKeyPair {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse::<Ident>()?;
        let _eq = input.parse::<Token![=]>()?;

        match key {
            key if key == NvimOxi::KEY => {
                let mut nvim_oxi = input.parse::<NvimOxi>()?;
                nvim_oxi.key_span = key.span();
                Ok(Self::NvimOxi(nvim_oxi))
            },

            _ => Err(syn::Error::new(key.span(), "invalid attribute")),
        }
    }
}

type Key = &'static str;

trait KeyPair: Default + Parse {
    const KEY: Key;

    fn key_span(&self) -> Span;
}

struct NvimOxi {
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
        let value = input.parse::<Path>()?;
        Ok(Self { key_span: Span::call_site(), value })
    }
}

impl KeyPair for NvimOxi {
    const KEY: Key = "nvim_oxi";

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

struct DuplicateError<T>(T);

impl<T: KeyPair> From<DuplicateError<T>> for syn::Error {
    #[inline]
    fn from(DuplicateError(keypair): DuplicateError<T>) -> Self {
        struct ErrorMsg(Key);

        impl core::fmt::Display for ErrorMsg {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "duplicate attribute: `{}`", self.0)
            }
        }

        syn::Error::new(keypair.key_span(), ErrorMsg(T::KEY))
    }
}
