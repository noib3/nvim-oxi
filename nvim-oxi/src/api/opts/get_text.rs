use derive_builder::Builder;
use nvim_types::Dictionary;

/// Options passed to [`Buffer::get_text`](crate::api::Buffer::get_text).
/// Currently unused.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetTextOpts {}

impl GetTextOpts {
    #[inline(always)]
    pub fn builder() -> GetTextOptsBuilder {
        GetTextOptsBuilder::default()
    }
}

impl GetTextOptsBuilder {
    pub fn build(&mut self) -> GetTextOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&GetTextOpts> for Dictionary {
    fn from(_: &GetTextOpts) -> Self {
        Dictionary::new()
    }
}
