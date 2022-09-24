use nvim_oxi::{self as oxi, libuv};

#[oxi::module]
fn libuv() -> oxi::Result<()> {
    oxi::print!("hey there");

    Ok(())
}
