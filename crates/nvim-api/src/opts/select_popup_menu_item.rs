use derive_builder::Builder;
use nvim_types::Dictionary;

/// Options passed to
/// [`api::select_popupmenu_item`](crate::select_popupmenu_item).
/// Currently unused.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct SelectPopupMenuItemOpts {}

impl SelectPopupMenuItemOpts {
    #[inline(always)]
    pub fn builder() -> SelectPopupMenuItemOptsBuilder {
        SelectPopupMenuItemOptsBuilder::default()
    }
}

impl SelectPopupMenuItemOptsBuilder {
    pub fn build(&mut self) -> SelectPopupMenuItemOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&SelectPopupMenuItemOpts> for Dictionary {
    fn from(_: &SelectPopupMenuItemOpts) -> Self {
        Dictionary::new()
    }
}
