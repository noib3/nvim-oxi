/// Options passed to [`Buffer::get_text()`](crate::Buffer::get_text).
/// Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetTextOpts {
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    #[builder(mask)]
    mask: u64,
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl From<&GetTextOpts> for types::Dictionary {
    #[inline]
    fn from(_: &GetTextOpts) -> Self {
        Self::new()
    }
}
