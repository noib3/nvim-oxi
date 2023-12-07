/// Options passed to [`Buffer::set_mark()`](crate::Buffer::get_mark).
/// Currently unused.
#[derive(Clone, Debug, Default, oxi_macros::OptsBuilder)]
#[repr(C)]
pub struct SetMarkOpts {
    #[cfg(feature = "neovim-nightly")]
    #[builder(mask)]
    mask: u64,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl From<&SetMarkOpts> for oxi_types::Dictionary {
    fn from(_: &SetMarkOpts) -> Self {
        Self::new()
    }
}
