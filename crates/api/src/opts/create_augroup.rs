/// Options passed to [`create_augroup()`](crate::create_augroup).
#[cfg(feature = "neovim-nightly")] // Only on Nightly.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct CreateAugroupOpts {
    #[builder(mask)]
    mask: u64,

    /// Whether to clear existing commands if the group already exists.
    #[builder(argtype = "bool")]
    clear: types::Boolean,
}

/// Options passed to [`create_augroup()`](crate::create_augroup).
#[cfg(not(feature = "neovim-nightly"))] // Only on 0.10.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateAugroupOpts {
    clear: types::Object,
}

#[cfg(not(feature = "neovim-nightly"))] // Only on 0.10.
impl CreateAugroupOpts {
    #[inline(always)]
    pub fn builder() -> CreateAugroupOptsBuilder {
        CreateAugroupOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-nightly"))] // Only on 0.10.
#[derive(Clone, Default)]
pub struct CreateAugroupOptsBuilder(CreateAugroupOpts);

#[cfg(not(feature = "neovim-nightly"))] // Only on 0.10.
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
