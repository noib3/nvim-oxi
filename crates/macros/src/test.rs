use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Block, ItemFn, LitStr, Token};

use crate::common::{DuplicateError, Keyed, KeyedAttribute};
use crate::plugin::NvimOxi;

#[inline]
pub fn test(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attrs as Attributes);

    let ItemFn { sig, block, .. } = parse_macro_input!(item as syn::ItemFn);

    let test_name = sig.ident;

    let plugin_name = Ident::new(&format!("__{test_name}"), Span::call_site());

    let test_body = test_body(&attrs, &plugin_name);

    let plugin_body = plugin_body(&attrs, &block, &sig.output);

    let nvim_oxi = &attrs.nvim_oxi;

    quote! {
        #[test]
        fn #test_name() {
            #test_body
        }

        #[#nvim_oxi::plugin(nvim_oxi = #nvim_oxi)]
        fn #plugin_name() -> #nvim_oxi::Object {
            #plugin_body
        }
    }
    .into()
}

fn test_body(
    attrs: &Attributes,
    plugin_name: &Ident,
) -> proc_macro2::TokenStream {
    let nvim_oxi = &attrs.nvim_oxi;

    let cmd = match &attrs.cmd {
        Some(cmd) => quote! { ["-c", #cmd] },
        None => quote! { ::core::iter::empty::<&str>() },
    };

    quote! {
        let library_name = {
            let mut s = ::std::string::String::new();
            s.push_str(::std::env::consts::DLL_PREFIX);
            s.push_str(env!("CARGO_CRATE_NAME"));
            s.push_str(::std::env::consts::DLL_SUFFIX);
            s
        };

        let manifest_dir = env!("CARGO_MANIFEST_DIR");

        // The full path to the compiled library.
        let library_path = #nvim_oxi::__test::get_target_dir(manifest_dir.as_ref())
            .join("debug")
            .join(library_name);

        if !library_path.exists() {
            panic!(
                "Compiled library not found in '{}'. Please run `cargo \
                 build` before running the tests.",
                library_path.display()
            )
        }

        let load_library = format!(
            "lua local f = package.loadlib([[{}]], 'luaopen_{}'); f()",
            library_path.display(),
            stringify!(#plugin_name),
        );

        let out = ::std::process::Command::new("nvim")
            .args(["-u", "NONE", "--headless"])
            .args(["-i", "NONE"])
            .args(["-c", "set noswapfile"])
            .args(#cmd)
            .args(["-c", &load_library])
            .args(["+quit"])
            .output()
            .expect("Couldn't find `nvim` binary in $PATH");

        if out.status.success() {
            return;
        }

        let mut stderr = ::std::string::String::from_utf8_lossy(&out.stderr);

        if !stderr.is_empty() {
            let print_from =
                // The test failed due to a panic.
                if stderr.starts_with("thread") {
                    // Remove the last 2 lines for a cleaner error msg.
                    stderr = {
                        let lines = stderr.lines().collect::<Vec<_>>();
                        let up_to = lines.len().saturating_sub(2);
                        ::std::borrow::Cow::Owned(lines[..up_to].join("\n"))
                    };

                    let panicked_at = "panicked at ";

                    stderr.match_indices(panicked_at)
                        .next()
                        .map(|(offset, _)| offset + panicked_at.len())
                        .unwrap_or(0)
                }
                // The test failed because an error was returned.
                else {
                    0
                };

            panic!("{}", &stderr[print_from..]);
        } else if let Some(code) = out.status.code() {
            panic!("Neovim exited with non-zero exit code: {}", code);
        } else {
            panic!("Neovim segfaulted");
        }
    }
}

fn plugin_body(
    attrs: &Attributes,
    test_body: &Block,
    test_return_ty: &syn::ReturnType,
) -> proc_macro2::TokenStream {
    if let Some(test_fn) = &attrs.test_fn {
        let fn_name = &test_fn.name;
        quote! { #fn_name().into() }
    } else {
        quote! {
            trait __IntoResult {
                type Error: ::core::fmt::Debug;
                fn into_result(self) -> ::core::result::Result<(), Self::Error>;
            }

            impl __IntoResult for () {
                type Error = ::core::convert::Infallible;
                fn into_result(self) -> ::core::result::Result<(), Self::Error> {
                    Ok(())
                }
            }

            impl<E: ::core::fmt::Debug> __IntoResult for ::core::result::Result<(), E> {
                type Error = E;
                fn into_result(self) -> ::core::result::Result<(), E> {
                    self
                }
            }

            fn __test_fn() #test_return_ty {
                #test_body
            }

            let result = ::std::panic::catch_unwind(|| {
                __IntoResult::into_result(__test_fn())
            });

            let exit_code = match result {
                Ok(Ok(())) => 0,

                Ok(Err(err)) => {
                    eprintln!("{:?}", err);
                    1
                },

                Err(panic) => {
                    eprintln!("{:?}", panic);
                    1
                },
            };

            ::std::process::exit(exit_code)
        }
    }
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
