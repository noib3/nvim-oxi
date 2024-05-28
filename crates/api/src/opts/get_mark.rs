/// Options passed to [`get_mark()`](crate::get_mark). Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetMarkOpts {
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    #[builder(mask)]
    mask: u64,
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl From<&GetMarkOpts> for types::Dictionary {
    fn from(_: &GetMarkOpts) -> Self {
        Self::new()
    }
}
