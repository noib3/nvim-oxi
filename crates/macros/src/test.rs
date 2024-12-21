use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, ItemFn, LitStr, Token};

use crate::common::{DuplicateError, Keyed, KeyedAttribute};
use crate::plugin::NvimOxi;

#[inline]
pub fn test(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attrs as Attributes);

    let ItemFn { attrs: test_attrs, sig, block, .. } =
        parse_macro_input!(item as syn::ItemFn);

    let test_attrs = test_attrs
        .into_iter()
        .map(ToTokens::into_token_stream)
        .collect::<proc_macro2::TokenStream>();

    let test_name = sig.ident;

    let plugin_name = Ident::new(&format!("__{test_name}"), Span::call_site());

    let ret = &sig.output;

    let nvim_oxi = &attrs.nvim_oxi;

    let library_path = match &attrs.library_path {
        Some(LibraryPath { path, .. }) => {
            quote! { ::core::option::Option::Some(#path) }
        },
        None => quote! { ::core::option::Option::<&str>::None },
    };

    let extra_cmd = match &attrs.cmd {
        Some(Cmd { cmd, .. }) => quote! { ::core::option::Option::Some(#cmd) },
        None => quote! { ::core::option::Option::None },
    };

    #[cfg(feature = "test-terminator")]
    let plugin_body = match &sig.inputs.first() {
        Some(terminator) => quote! {
           fn __test_fn(#terminator) #ret {
               #block
           }
           #nvim_oxi::tests::test_macro::plugin_body_with_terminator(__test_fn)
        },
        None => quote! {
            fn __test_fn() #ret {
                #block
            }
            #nvim_oxi::tests::test_macro::plugin_body(__test_fn)
        },
    };

    #[cfg(not(feature = "test-terminator"))]
    let plugin_body = quote! {
        fn __test_fn() #ret {
            #block
        }
        #nvim_oxi::tests::test_macro::plugin_body(__test_fn)
    };

    quote! {
        #[test]
        #test_attrs
        fn #test_name() -> ::core::result::Result<(), ::std::string::String> {
            #nvim_oxi::tests::test_macro::test_body(
                env!("CARGO_CRATE_NAME"),
                env!("CARGO_MANIFEST_PATH"),
                stringify!(#plugin_name),
                #library_path,
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
    library_path: Option<LibraryPath>,
    nvim_oxi: NvimOxi,
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
                Attribute::LibraryPath(library_path) => {
                    if this.library_path.is_some() {
                        return Err(DuplicateError(library_path).into());
                    }
                    this.library_path = Some(library_path);
                },
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
    Cmd(Cmd),
    LibraryPath(LibraryPath),
    NvimOxi(NvimOxi),
}

impl Parse for Attribute {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input
            .parse::<Cmd>()
            .map(Self::Cmd)
            .or_else(|_| input.parse::<LibraryPath>().map(Self::LibraryPath))
            .or_else(|_| input.parse::<NvimOxi>().map(Self::NvimOxi))
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

/// The path to the compiled test library.
struct LibraryPath {
    key_span: Span,
    path: Expr,
}

impl KeyedAttribute for LibraryPath {
    const KEY: &'static str = "library_path";

    type Value = Expr;

    #[inline]
    fn key_span(&self) -> Span {
        self.key_span
    }
}

impl Parse for LibraryPath {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key_span: Span::call_site(),
            path: input.parse::<Keyed<Self>>()?.value,
        })
    }
}
