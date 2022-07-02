use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Error};

#[proc_macro_attribute]
pub fn oxi_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);

    if !args.is_empty() {
        return Error::new(Span::call_site(), "no attributes are supported")
            .to_compile_error()
            .into();
    }

    let item = parse_macro_input!(item as syn::ItemFn);

    let syn::ItemFn { sig, block, .. } = item;
    let test_name = sig.ident;
    let test_body = block;

    let module_name = Ident::new(&format!("__{test_name}"), Span::call_site());

    quote! {
        #[test]
        fn #test_name() {
            use std::path::PathBuf;
            use std::process::Command;

            #[cfg(all(unix, not(target_os = "macos")))]
            mod consts {
                pub const COMPILED_LIB_FILENAME: &str =
                    concat!("lib", env!("CARGO_CRATE_NAME"), ".so");

                pub const TARGET_LIB_FILENAME: &str =
                    concat!("__", stringify!(#test_name), ".so");
            }

            #[cfg(target_os = "macos")]
            mod consts {
                pub const COMPILED_LIB_FILENAME: &str =
                    concat!("lib", env!("CARGO_CRATE_NAME"), ".dylib");

                pub const TARGET_LIB_FILENAME: &str =
                    concat!("__", stringify!(#test_name), ".so");
            }

            #[cfg(target_os = "windows")]
            mod consts {
                pub const COMPILED_LIB_FILENAME: &str =
                    concat!(env!("CARGO_CRATE_NAME"), ".dll");

                pub const TARGET_LIB_FILENAME: &str =
                    concat!("__", stringify!(#test_name), ".dll");
            }

            let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

            let from_path = root
                .join("target")
                .join("release")
                .join(consts::COMPILED_LIB_FILENAME);

            let to_path = root.join("lua").join(consts::TARGET_LIB_FILENAME);

            if !from_path.exists() {
                panic!(
                    "Compiled library not found in '{}'. Please run `cargo \
                     build --release` before running the tests.",
                    from_path.display()
                )
            }

            if !to_path.parent().unwrap().exists() {
                std::fs::create_dir(to_path.parent().unwrap()).unwrap();
            }

            std::fs::copy(&from_path, &to_path).unwrap();

            let out = Command::new("nvim")
                .args(["-u", "NONE", "--headless"])
                .args(["-c", &format!("set rtp+={}", root.display())])
                .args([
                    "-c",
                    &format!("lua require('__{}')", stringify!(#test_name)),
                ])
                .args(["+quit"])
                .output()
                .expect("Couldn't find `nvim` binary in $PATH!");

            let stderr = String::from_utf8_lossy(&out.stderr);
            if !stderr.is_empty() {
                // Remove the last 2 lines from stderr for a cleaner error msg.
                let lines = stderr.lines().collect::<Vec<_>>();
                let len = lines.len();
                // let stderr = if len >= 2 {
                //     lines[..lines.len() - 2].join("\n")
                // } else {
                //     lines.join("\n")
                // };
                let stderr = &lines[..lines.len() - 2].join("\n");
                // The first 31 bytes are `thread '<unnamed>' panicked at `
                let (_, stderr) = stderr.split_at(31);
                panic!("{}", stderr)
            }
        }

        #[::nvim_oxi::module]
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
    .into()
}
