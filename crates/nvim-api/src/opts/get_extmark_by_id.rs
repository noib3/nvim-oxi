use nvim_types::{Dictionary, Object};

/// Options passed to
/// [`Buffer::get_extmark_by_id()`](crate::Buffer::get_extmark_by_id).
#[derive(Clone, Debug, Default)]
pub struct GetExtmarkByIdOpts {
    details: Object,
}

impl GetExtmarkByIdOpts {
    #[inline]
    pub fn builder() -> GetExtmarkByIdOptsBuilder {
        GetExtmarkByIdOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct GetExtmarkByIdOptsBuilder(GetExtmarkByIdOpts);

impl GetExtmarkByIdOptsBuilder {
    /// Whether to include the extmark's
    /// [`ExtmarkInfos`](crate::types::ExtmarkInfos) as the last element of the
    /// tuple returned by
    /// [`Buffer::get_extmark_by_id`](crate::Buffer::get_extmark_by_id).
    #[inline]
    pub fn details(&mut self, details: bool) -> &mut Self {
        self.0.details = details.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> GetExtmarkByIdOpts {
        std::mem::take(&mut self.0)
    }
}

impl From<&GetExtmarkByIdOpts> for Dictionary {
    fn from(opts: &GetExtmarkByIdOpts) -> Self {
        Self::from_iter([("details", opts.details.clone())])
    }
}
