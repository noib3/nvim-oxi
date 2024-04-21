use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, ItemFn, LitStr, Token};

use crate::common::{DuplicateError, Keyed, KeyedAttribute};
use crate::plugin::NvimOxi;

#[inline]
pub fn test(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attrs as Attributes);

    let ItemFn { sig, block, .. } = parse_macro_input!(item as syn::ItemFn);

    let test_name = sig.ident;

    let plugin_name = Ident::new(&format!("__{test_name}"), Span::call_site());

    let ret = &sig.output;

    let nvim_oxi = &attrs.nvim_oxi;

    let extra_cmd = match &attrs.cmd {
        Some(Cmd { cmd, .. }) => quote! { Some(#cmd) },
        None => quote! { None },
    };

    let plugin_body = match &attrs.test_fn {
        Some(TestFn { name, .. }) => {
            quote! { #nvim_oxi::tests::plugin_body(#name) }
        },
        None => quote! {
            fn __test_fn() #ret {
                #block
            }
            #nvim_oxi::tests::plugin_body(__test_fn)
        },
    };

    quote! {
        #[test]
        fn #test_name() -> ::core::result::Result<(), ::std::string::String> {
            #nvim_oxi::tests::test_body(
                env!("CARGO_CRATE_NAME"),
                env!("CARGO_MANIFEST_DIR"),
                stringify!(#plugin_name),
                #extra_cmd,
            )
        }

        #[#nvim_oxi::plugin(nvim_oxi = #nvim_oxi)]
        fn #plugin_name()  {
            #plugin_body
        }
    }
    .into()
}

#[derive(Default)]
struct Attributes {
    cmd: Option<Cmd>,
    nvim_oxi: NvimOxi,
    test_fn: Option<TestFn>,
}

impl Parse for Attributes {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        let mut has_parsed_nvim_oxi = false;

        while !input.is_empty() {
            match input.parse::<Attribute>()? {
                Attribute::Cmd(cmd) => {
                    if this.cmd.is_some() {
                        return Err(DuplicateError(cmd).into());
                    }
                    this.cmd = Some(cmd);
                },
                Attribute::NvimOxi(nvim_oxi) => {
                    if has_parsed_nvim_oxi {
                        return Err(DuplicateError(nvim_oxi).into());
                    }
                    this.nvim_oxi = nvim_oxi;
                    has_parsed_nvim_oxi = true;
                },
                Attribute::TestFn(test_fn) => {
                    if this.test_fn.is_some() {
                        return Err(DuplicateError(test_fn).into());
                    }
                    this.test_fn = Some(test_fn);
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
    Cmd(Cmd),
    NvimOxi(NvimOxi),
    TestFn(TestFn),
}

impl Parse for Attribute {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input
            .parse::<Cmd>()
            .map(Self::Cmd)
            .or_else(|_| input.parse::<NvimOxi>().map(Self::NvimOxi))
            .or_else(|_| input.parse::<TestFn>().map(Self::TestFn))
    }
}

/// The command that will be passed to the Neovim CLI.
struct Cmd {
    key_span: Span,
    cmd: LitStr,
}

impl KeyedAttribute for Cmd {
    const KEY: &'static str = "cmd";

    type Value = LitStr;

    #[inline]
    fn key_span(&self) -> Span {
        self.key_span
    }
}

impl Parse for Cmd {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key_span: Span::call_site(),
            cmd: input.parse::<Keyed<Self>>()?.value,
        })
    }
}

impl ToTokens for Cmd {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let str = self.cmd.value().lines().collect::<Vec<_>>().join(";");
        let lit = LitStr::new(&str, self.cmd.span());
        lit.to_tokens(tokens);
    }
}

/// The name of the function that will be executed in the entrypoint of the
/// test.
struct TestFn {
    key_span: Span,
    name: Ident,
}

impl KeyedAttribute for TestFn {
    const KEY: &'static str = "test_fn";

    type Value = Ident;

    #[inline]
    fn key_span(&self) -> Span {
        self.key_span
    }
}

impl Parse for TestFn {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key_span: Span::call_site(),
            name: input.parse::<Keyed<Self>>()?.value,
        })
    }
}
