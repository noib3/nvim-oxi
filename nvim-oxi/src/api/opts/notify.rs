use derive_builder::Builder;
use nvim_types::Dictionary;

/// Options passed to `crate::api::notify`. It's currently reserved for
/// future use and doesn't have any methods.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct NotifyOpts {}

impl NotifyOpts {
    #[inline(always)]
    pub fn builder() -> NotifyOptsBuilder {
        NotifyOptsBuilder::default()
    }
}

impl NotifyOptsBuilder {
    pub fn build(&mut self) -> NotifyOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&NotifyOpts> for Dictionary {
    fn from(_: &NotifyOpts) -> Self {
        Dictionary::new()
    }
}
