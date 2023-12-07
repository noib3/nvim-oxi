/// Options passed to
/// [`select_popupmenu_item()`](crate::select_popupmenu_item).
/// Currently unused.
#[derive(Clone, Debug, Default, oxi_macros::OptsBuilder)]
#[repr(C)]
pub struct SelectPopupMenuItemOpts {
    #[cfg(feature = "neovim-nightly")]
    #[builder(mask)]
    mask: u64,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl From<&SelectPopupMenuItemOpts> for oxi_types::Dictionary {
    fn from(_: &SelectPopupMenuItemOpts) -> Self {
        Self::new()
    }
}
