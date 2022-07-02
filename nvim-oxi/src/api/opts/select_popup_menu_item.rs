use derive_builder::Builder;
use nvim_types::Dictionary;

/// Options passed to `crate::api::select_popupmenu_item`. It's currently
/// reserved for future use and doesn't have any methods.
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
