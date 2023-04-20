use nvim_types::Dictionary;

/// Options passed to
/// [`select_popupmenu_item()`](crate::select_popupmenu_item).
/// Currently unused.
#[derive(Clone, Debug, Default)]
pub struct SelectPopupMenuItemOpts {}

impl SelectPopupMenuItemOpts {
    #[inline]
    pub fn builder() -> SelectPopupMenuItemOptsBuilder {
        SelectPopupMenuItemOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct SelectPopupMenuItemOptsBuilder(SelectPopupMenuItemOpts);

impl SelectPopupMenuItemOptsBuilder {
    #[inline]
    pub fn build(&mut self) -> SelectPopupMenuItemOpts {
        std::mem::take(&mut self.0)
    }
}

impl From<&SelectPopupMenuItemOpts> for Dictionary {
    fn from(_: &SelectPopupMenuItemOpts) -> Self {
        Dictionary::new()
    }
}
