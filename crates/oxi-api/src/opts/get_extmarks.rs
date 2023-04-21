use oxi_types::{Dictionary, Object};

/// Options passed to
/// [`Buffer::get_extmarks()`](crate::Buffer::get_extmarks).
#[derive(Clone, Debug, Default)]
pub struct GetExtmarksOpts {
    details: Object,
    limits: Object,
}

impl GetExtmarksOpts {
    #[inline(always)]
    /// Creates a new [`GetExtmarksOptsBuilder`].
    pub fn builder() -> GetExtmarksOptsBuilder {
        GetExtmarksOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct GetExtmarksOptsBuilder(GetExtmarksOpts);

impl GetExtmarksOptsBuilder {
    /// Whether to include the extmark's
    /// [`ExtmarkInfos`](crate::types::ExtmarkInfos) as the last element of
    /// the tuples returned by
    /// [`Buffer::get_extmarks()`](crate::Buffer::get_extmarks).
    #[inline]
    pub fn details(&mut self, details: bool) -> &mut Self {
        self.0.details = details.into();
        self
    }

    #[inline]
    pub fn limits(&mut self, limits: bool) -> &mut Self {
        self.0.limits = limits.into();
        self
    }

    /// Maximum number of extmarks to return.
    #[inline]
    pub fn build(&mut self) -> GetExtmarksOpts {
        std::mem::take(&mut self.0)
    }
}

impl From<&GetExtmarksOpts> for Dictionary {
    fn from(opts: &GetExtmarksOpts) -> Self {
        Self::from_iter([
            ("details", opts.details.clone()),
            ("limits", opts.limits.clone()),
        ])
    }
}
