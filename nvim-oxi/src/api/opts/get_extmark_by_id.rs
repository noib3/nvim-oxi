use derive_builder::Builder;
use nvim_types::Dictionary;

/// Options passed to `Buffer::get_extmark_by_id`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetExtmarkByIdOpts {
    #[builder(setter(strip_option))]
    /// Whether to include the extmark's `ExtmarkInfos` in the returned tuple.
    details: Option<bool>,
}

impl GetExtmarkByIdOpts {
    #[inline(always)]
    pub fn builder() -> GetExtmarkByIdOptsBuilder {
        GetExtmarkByIdOptsBuilder::default()
    }
}

impl GetExtmarkByIdOptsBuilder {
    pub fn build(&mut self) -> GetExtmarkByIdOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<GetExtmarkByIdOpts> for Dictionary {
    fn from(opts: GetExtmarkByIdOpts) -> Self {
        Self::from_iter([("details", opts.details)])
    }
}
