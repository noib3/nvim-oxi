// Suppress warnings when testing deprecated functions.
#![allow(deprecated)]

mod api;
mod r#macro;

// Libuv bindings don't work on Windows.
#[cfg(not(any(target_os = "windows", target_env = "msvc")))]
mod libuv;

#[nvim_oxi::test]
fn fooo() {
    println!("Hello from stdout");
    eprintln!("Hello from stderr");
    panic!("foo!");
}
