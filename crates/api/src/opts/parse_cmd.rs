/// Options passed to [`parse_cmd()`](crate::parse_cmd). Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct ParseCmdOpts {
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    #[builder(mask)]
    mask: u64,
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl From<&ParseCmdOpts> for types::Dictionary {
    fn from(_: &ParseCmdOpts) -> Self {
        Self::new()
    }
}
