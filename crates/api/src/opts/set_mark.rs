/// Options passed to [`Buffer::set_mark()`](crate::Buffer::get_mark).
/// Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct SetMarkOpts {
    #[cfg(feature = "neovim-nightly")]
    #[builder(mask)]
    mask: u64,
}

#[cfg(not(feature = "neovim-nightly"))]
impl From<&SetMarkOpts> for types::Dictionary {
    fn from(_: &SetMarkOpts) -> Self {
        Self::new()
    }
}
