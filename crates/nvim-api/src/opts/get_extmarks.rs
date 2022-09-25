use derive_builder::Builder;
use nvim_types::{Dictionary, Object};

/// Options passed to
/// [`Buffer::get_extmarks`](crate::Buffer::get_extmarks).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetExtmarksOpts {
    /// Whether to include the extmark's
    /// [`ExtmarkInfos`](crate::types::ExtmarkInfos) as the last element of the
    /// tuples returned by
    /// [`Buffer::get_extmarks`](crate::Buffer::get_extmarks).
    #[builder(setter(strip_option))]
    details: Option<bool>,

    /// Maximum number of extmarks to return.
    #[builder(setter(strip_option))]
    limits: Option<u32>,
}

impl GetExtmarksOpts {
    #[inline(always)]
    /// Creates a new [`GetExtmarksOptsBuilder`].
    pub fn builder() -> GetExtmarksOptsBuilder {
        GetExtmarksOptsBuilder::default()
    }
}

impl GetExtmarksOptsBuilder {
    pub fn build(&mut self) -> GetExtmarksOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&GetExtmarksOpts> for Dictionary {
    fn from(opts: &GetExtmarksOpts) -> Self {
        Self::from_iter([
            ("details", opts.details.into()),
            ("limits", Object::from(opts.limits)),
        ])
    }
}
