use oxi_types::Object;

/// Options passed to [`create_augroup()`](crate::create_augroup).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateAugroupOpts {
    clear: Object,
}

impl CreateAugroupOpts {
    #[inline(always)]
    pub fn builder() -> CreateAugroupOptsBuilder {
        CreateAugroupOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct CreateAugroupOptsBuilder(CreateAugroupOpts);

impl CreateAugroupOptsBuilder {
    /// Whether to clear existing commands if the group already exists.
    #[inline]
    pub fn clear(&mut self, clear: bool) -> &mut Self {
        self.0.clear = clear.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> CreateAugroupOpts {
        std::mem::take(&mut self.0)
    }
}
