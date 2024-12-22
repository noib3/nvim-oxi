//! Tests about the `#[nvim_oxi::test]` macro.

#[should_panic]
#[nvim_oxi::test]
fn panic_is_propagated() {
    panic!();
}

#[nvim_oxi::test]
fn printing_to_stderr_is_ok() {
    eprintln!("AA!");
}
