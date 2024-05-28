/// Options passed to
/// [`select_popupmenu_item()`](crate::select_popupmenu_item).
/// Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct SelectPopupMenuItemOpts {
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    #[builder(mask)]
    mask: u64,
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl From<&SelectPopupMenuItemOpts> for types::Dictionary {
    fn from(_: &SelectPopupMenuItemOpts) -> Self {
        Self::new()
    }
}
