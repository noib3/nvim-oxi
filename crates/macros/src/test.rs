use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{ItemFn, LitStr, Token};

use crate::common::{DuplicateError, Keyed, KeyedAttribute};
use crate::plugin::NvimOxi;

#[inline]
pub fn test(item: ItemFn) -> TokenStream {
    let ItemFn { sig, block, .. } = item;

    // TODO: here we'd need to append something like the module path of the
    // call site to `test_name` to avoid collisions between equally named tests
    // across different modules. Unfortunately that doesn't seem to be possible
    // yet?
    // See https://www.reddit.com/r/rust/comments/a3fgp6/procmacro_determining_the_callers_module_path/
    let test_name = sig.ident;
    let test_body = block;

    let module_name = Ident::new(&format!("__{test_name}"), Span::call_site());

    quote! {
        #[test]
        fn #test_name() {
            let mut library_filename = String::new();
            library_filename.push_str(::std::env::consts::DLL_PREFIX);
            library_filename.push_str(env!("CARGO_CRATE_NAME"));
            library_filename.push_str(::std::env::consts::DLL_SUFFIX);

            let manifest_dir = env!("CARGO_MANIFEST_DIR");
            let target_dir = nvim_oxi::__test::get_target_dir(manifest_dir.as_ref()).join("debug");

            let library_filepath = target_dir.join(library_filename);

            if !library_filepath.exists() {
                panic!(
                    "Compiled library not found in '{}'. Please run `cargo \
                     build` before running the tests.",
                    library_filepath.display()
                )
            }

            let out = ::std::process::Command::new("nvim")
                .args(["-u", "NONE", "--headless"])
                .args(["-i", "NONE"])
                .args(["-c", "set noswapfile"])
                .args([
                    "-c",
                    &format!(
                        "lua local f = package.loadlib([[{}]], 'luaopen___{}'); f()",
                        library_filepath.display(),
                        stringify!(#test_name),
                    ),
                ])
                .args(["+quit"])
                .output()
                .expect("Couldn't find `nvim` binary in $PATH");

            if out.status.success() {
                return;
            }

            let stderr = String::from_utf8_lossy(&out.stderr);

            if !stderr.is_empty() {
                // Remove the last 2 lines from stderr for a cleaner error msg.
                let stderr = {
                    let lines = stderr.lines().collect::<Vec<_>>();
                    let len = lines.len();
                    lines[..lines.len() - 2].join("\n")
                };

                // The first 31 bytes are `thread '<unnamed>' panicked at `.
                let (_, stderr) = stderr.split_at(31);

                panic!("{}", stderr)
            } else if let Some(code) = out.status.code() {
                panic!("Neovim exited with non-zero exit code: {}", code);
            } else {
                panic!("Neovim segfaulted");
            }
        }

        #[::nvim_oxi::plugin]
        fn #module_name() -> ::nvim_oxi::Result<()> {
            let result = ::std::panic::catch_unwind(|| {
                #test_body
            });

            ::std::process::exit(match result {
                Ok(_) => 0,
                Err(err) => {
                    eprintln!("{:?}", err);
                    1
                },
            })
        }
    }
}

#[derive(Default)]
struct Attributes {
    cmd: Option<Cmd>,
    nvim_oxi: Option<NvimOxi>,
    test_fn: Option<TestFn>,
}

impl Parse for Attributes {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        while !input.is_empty() {
            match input.parse::<Attribute>()? {
                Attribute::Cmd(cmd) => {
                    if this.cmd.is_some() {
                        return Err(DuplicateError(cmd).into());
                    }
                    this.cmd = Some(cmd);
                },
                Attribute::NvimOxi(nvim_oxi) => {
                    if this.nvim_oxi.is_some() {
                        return Err(DuplicateError(nvim_oxi).into());
                    }
                    this.nvim_oxi = Some(nvim_oxi);
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
