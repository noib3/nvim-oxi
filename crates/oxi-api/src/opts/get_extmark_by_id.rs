use oxi_types as types;

/// Options passed to
/// [`Buffer::get_extmark_by_id()`](crate::Buffer::get_extmark_by_id).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
pub struct GetExtmarkByIdOpts {
    details: types::Object,
}

#[cfg(not(feature = "neovim-nightly"))]
impl GetExtmarkByIdOpts {
    #[inline]
    pub fn builder() -> GetExtmarkByIdOptsBuilder {
        GetExtmarkByIdOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Default)]
pub struct GetExtmarkByIdOptsBuilder(GetExtmarkByIdOpts);

#[cfg(not(feature = "neovim-nightly"))]
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

#[cfg(not(feature = "neovim-nightly"))]
impl From<&GetExtmarkByIdOpts> for types::Dictionary {
    fn from(opts: &GetExtmarkByIdOpts) -> Self {
        Self::from_iter([("details", opts.details.clone())])
    }
}

/// Options passed to
/// [`Buffer::get_extmark_by_id()`](crate::Buffer::get_extmark_by_id).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, oxi_macros::OptsBuilder)]
#[repr(C)]
pub struct GetExtmarkByIdOpts {
    #[builder(mask)]
    mask: u64,

    /// Whether to include the extmark's
    /// [`ExtmarkInfos`](crate::types::ExtmarkInfos) as the last element of the
    /// tuple returned by
    /// [`Buffer::get_extmark_by_id`](crate::Buffer::get_extmark_by_id).
    #[builder(argtype = "bool")]
    details: types::Boolean,

    #[builder(argtype = "bool")]
    hl_name: types::Boolean,
}
