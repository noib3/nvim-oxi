/// Options passed to [`get_mark()`](crate::get_mark). Currently unused.
#[derive(Clone, Debug, Default, oxi_macros::OptsBuilder)]
#[repr(C)]
pub struct GetMarkOpts {
    #[cfg(feature = "neovim-nightly")]
    #[builder(mask)]
    mask: u64,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl From<&GetMarkOpts> for oxi_types::Dictionary {
    fn from(_: &GetMarkOpts) -> Self {
        Self::new()
    }
}
