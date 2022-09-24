use nvim_oxi::{self as oxi, r#loop};

#[oxi::module]
fn libuv() -> oxi::Result<()> {
    oxi::print!("hey there");

    Ok(())
}
