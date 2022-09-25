use derive_builder::Builder;
use nvim_types::Dictionary;

/// Options passed to
/// [`Buffer::get_extmark_by_id`](crate::Buffer::get_extmark_by_id).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetExtmarkByIdOpts {
    /// Whether to include the extmark's
    /// [`ExtmarkInfos`](crate::types::ExtmarkInfos) as the last element of the
    /// tuple returned by
    /// [`Buffer::get_extmark_by_id`](crate::Buffer::get_extmark_by_id).
    #[builder(setter(strip_option))]
    details: Option<bool>,
}

impl GetExtmarkByIdOpts {
    #[inline(always)]
    /// Creates a new [`GetExtmarkByIdOptsBuilder`].
    pub fn builder() -> GetExtmarkByIdOptsBuilder {
        GetExtmarkByIdOptsBuilder::default()
    }
}

impl GetExtmarkByIdOptsBuilder {
    pub fn build(&mut self) -> GetExtmarkByIdOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&GetExtmarkByIdOpts> for Dictionary {
    fn from(opts: &GetExtmarkByIdOpts) -> Self {
        Self::from_iter([("details", opts.details)])
    }
}
