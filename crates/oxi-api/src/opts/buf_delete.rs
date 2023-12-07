use oxi_types as types;

/// Options passed to [`Buffer::delete()`](crate::Buffer::delete).
#[derive(Clone, Debug, Default)]
#[cfg(not(feature = "neovim-nightly"))]
pub struct BufDeleteOpts {
    force: types::Object,
    unload: types::Object,
}

#[cfg(not(feature = "neovim-nightly"))]
impl BufDeleteOpts {
    #[inline(always)]
    pub fn builder() -> BufDeleteOptsBuilder {
        BufDeleteOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
#[cfg(not(feature = "neovim-nightly"))]
pub struct BufDeleteOptsBuilder(BufDeleteOpts);

#[cfg(not(feature = "neovim-nightly"))]
impl BufDeleteOptsBuilder {
    /// Force deletion ignoring unsaved changes.
    #[inline]
    pub fn force(&mut self, force: bool) -> &mut Self {
        self.0.force = force.into();
        self
    }

    /// If `true` the buffer will only be unloaded, not deleted.
    #[inline]
    pub fn unload(&mut self, unload: bool) -> &mut Self {
        self.0.unload = unload.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> BufDeleteOpts {
        std::mem::take(&mut self.0)
    }
}

#[cfg(not(feature = "neovim-nightly"))]
impl From<&BufDeleteOpts> for types::Dictionary {
    fn from(opts: &BufDeleteOpts) -> Self {
        Self::from_iter([
            ("force", opts.force.clone()),
            ("unload", opts.unload.clone()),
        ])
    }
}

/// Options passed to [`Buffer::attach`](crate::Buffer::attach).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, oxi_macros::OptsBuilder)]
#[repr(C)]
pub struct BufDeleteOpts {
    #[builder(mask)]
    mask: u64,

    /// Force deletion ignoring unsaved changes.
    #[builder(argtype = "bool")]
    force: types::Boolean,

    /// If `true` the buffer will only be unloaded, not deleted.
    #[builder(argtype = "bool")]
    unload: types::Boolean,
}
