/// Options passed to [`parse_cmd()`](crate::parse_cmd). Currently unused.
#[derive(Clone, Debug, Default, oxi_macros::OptsBuilder)]
#[repr(C)]
pub struct ParseCmdOpts {
    #[cfg(feature = "neovim-nightly")]
    #[builder(mask)]
    mask: u64,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl From<&ParseCmdOpts> for oxi_types::Dictionary {
    fn from(_: &ParseCmdOpts) -> Self {
        Self::new()
    }
}
