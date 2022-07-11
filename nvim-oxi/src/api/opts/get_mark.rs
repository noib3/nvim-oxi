use derive_builder::Builder;
use nvim_types::Dictionary;

/// Options passed to [`api::get_mark`](crate::api::get_mark). Currently unused.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetMarkOpts {}

impl GetMarkOpts {
    #[inline(always)]
    pub fn builder() -> GetMarkOptsBuilder {
        GetMarkOptsBuilder::default()
    }
}

impl GetMarkOptsBuilder {
    pub fn build(&mut self) -> GetMarkOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&GetMarkOpts> for Dictionary {
    fn from(_: &GetMarkOpts) -> Self {
        Dictionary::new()
    }
}
