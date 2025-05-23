/// Options passed to
/// [`select_popupmenu_item()`](crate::select_popupmenu_item).
/// Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct SelectPopupMenuItemOpts {
    #[builder(mask)]
    mask: u64,
}
