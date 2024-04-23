/// Options passed to [`parse_cmd()`](crate::parse_cmd). Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct ParseCmdOpts {
    #[cfg(feature = "neovim-nightly")]
    #[builder(mask)]
    mask: u64,
}

#[cfg(not(feature = "neovim-nightly"))]
impl From<&ParseCmdOpts> for types::Dictionary {
    fn from(_: &ParseCmdOpts) -> Self {
        Self::new()
    }
}
