/// Options passed to [`Buffer::get_text()`](crate::Buffer::get_text).
/// Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetTextOpts {
    #[cfg(feature = "neovim-nightly")]
    #[builder(mask)]
    mask: u64,
}

#[cfg(not(feature = "neovim-nightly"))]
impl From<&GetTextOpts> for types::Dictionary {
    #[inline]
    fn from(_: &GetTextOpts) -> Self {
        Self::new()
    }
}
