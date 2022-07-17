use nvim_oxi::{self as nvim, Dictionary, Function, Object};

#[nvim::module]
fn serialize() -> nvim::Result<()> {
    Ok(())
}
