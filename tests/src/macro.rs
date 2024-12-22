//! Tests about the `#[nvim_oxi::test]` macro.

#[nvim_oxi::test]
fn printing_to_stderr_is_ok() {
    eprintln!("AA!");
}
