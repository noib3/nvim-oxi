use derive_builder::Builder;
use nvim_types::{Dictionary, Object};

/// Options passed to `Buffer::get_extmark_by_id`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetExtmarksOpts {
    /// Maximum number of extmarks to return.
    #[builder(setter(into, strip_option))]
    limits: Option<u32>,

    /// Whether to include the extmark's `ExtmarkInfos` in the returned tuple.
    #[builder(setter(strip_option))]
    details: Option<bool>,
}

impl GetExtmarksOpts {
    #[inline(always)]
    pub fn builder() -> GetExtmarksOptsBuilder {
        GetExtmarksOptsBuilder::default()
    }
}

impl GetExtmarksOptsBuilder {
    pub fn build(&mut self) -> GetExtmarksOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<GetExtmarksOpts> for Dictionary {
    fn from(opts: GetExtmarksOpts) -> Self {
        Self::from_iter([
            ("limits", Object::from(opts.limits)),
            ("details", opts.details.into()),
        ])
    }
}
