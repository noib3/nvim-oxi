use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Error};

/// Tests a piece of code inside a Neovim session.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi::{self as nvim, api};
///
/// #[nvim::test]
/// fn set_get_del_var() {
///     api::set_var("foo", 42).unwrap();
///     assert_eq!(Ok(42), api::get_var("foo"));
///     assert_eq!(Ok(()), api::del_var("foo"));
/// }
/// ```
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

            let mut target_filename = String::from("__");
            target_filename.push_str(stringify!(#test_name));

            #[cfg(not(target_os = "macos"))]
            target_filename.push_str(::std::env::consts::DLL_SUFFIX);

            #[cfg(target_os = "macos")]
            target_filename.push_str(".so");

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

            let target_filepath =
                target_dir.join("oxi-test").join("lua").join(target_filename);

            if !target_filepath.parent().unwrap().exists() {
                if let Err(err) = ::std::fs::create_dir_all(
                    target_filepath.parent().unwrap(),
                ) {
                    // It might happen that another test created the `lua`
                    // directory between the first if and the `create_dir_all`.
                    if !matches!(
                        err.kind(),
                        ::std::io::ErrorKind::AlreadyExists
                    ) {
                        panic!("{}", err)
                    }
                }
            }

            #[cfg(unix)]
            let res = ::std::os::unix::fs::symlink(
                &library_filepath,
                &target_filepath,
            );

            #[cfg(windows)]
            let res = ::std::os::windows::fs::symlink_file(
                &library_filepath,
                &target_filepath,
            );

            if let Err(err) = res {
                if !matches!(err.kind(), ::std::io::ErrorKind::AlreadyExists) {
                    panic!("{}", err)
                }
            }

            let out = ::std::process::Command::new("nvim")
                .args(["-u", "NONE", "--headless"])
                .args(["-c", "set noswapfile"])
                .args([
                    "-c",
                    &format!(
                        "set rtp+={}",
                        target_dir.join("oxi-test").display()
                    ),
                ])
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
                let stderr = {
                    let lines = stderr.lines().collect::<Vec<_>>();
                    let len = lines.len();
                    lines[..lines.len() - 2].join("\n")
                };

                // The first 31 bytes are `thread '<unnamed>' panicked at `.
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
