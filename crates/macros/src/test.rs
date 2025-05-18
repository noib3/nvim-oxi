use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, AttrStyle, ItemFn, LitStr, Meta, Token};

use crate::common::{DuplicateError, Keyed, KeyedAttribute};
use crate::plugin::NvimOxi;

#[inline]
pub fn test(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attrs as Attributes);

    let ItemFn { attrs: test_attrs, sig, block, .. } =
        parse_macro_input!(item as syn::ItemFn);

    let should_panic = test_attrs.iter().any(|attr| {
        let AttrStyle::Outer = &attr.style else { return false };
        let Meta::Path(path) = &attr.meta else { return false };
        path.segments.iter().any(|segment| segment.ident == "should_panic")
    });

    let test_attrs = test_attrs
        .into_iter()
        .map(ToTokens::into_token_stream)
        .collect::<proc_macro2::TokenStream>();

    let test_name = sig.ident;

    let test_ret = if should_panic {
        quote!()
    } else {
        quote! {
            -> ::core::result::Result<(), ::std::string::String>
        }
    };

    let nvim_oxi = &attrs.nvim_oxi;

    let ret = &sig.output;

    let plugin_name = Ident::new(&format!("__{test_name}"), Span::call_site());

    let extra_cmd = match &attrs.cmd {
        Some(Cmd { cmd, .. }) => quote! { ::core::option::Option::Some(#cmd) },
        None => quote! { ::core::option::Option::None },
    };

    let maybe_ignore_err = should_panic.then(|| quote!(let _ = ));

    let maybe_semicolon = should_panic.then(|| quote!(;));

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
        fn #test_name() #test_ret {
            #maybe_ignore_err #nvim_oxi::tests::test_macro::test_body(
                env!("CARGO_MANIFEST_PATH"),
                stringify!(#plugin_name),
                #extra_cmd,
            )#maybe_semicolon
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
}

impl Parse for Attribute {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input
            .parse::<Cmd>()
            .map(Self::Cmd)
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
