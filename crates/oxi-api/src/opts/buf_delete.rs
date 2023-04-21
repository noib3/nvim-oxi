use oxi_types::{Dictionary, Object};

/// Options passed to [`Buffer::delete()`](crate::Buffer::delete).
#[derive(Clone, Debug, Default)]
pub struct BufDeleteOpts {
    force: Object,
    unload: Object,
}

impl BufDeleteOpts {
    #[inline(always)]
    pub fn builder() -> BufDeleteOptsBuilder {
        BufDeleteOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct BufDeleteOptsBuilder(BufDeleteOpts);

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

impl From<&BufDeleteOpts> for Dictionary {
    fn from(opts: &BufDeleteOpts) -> Self {
        Self::from_iter([
            ("force", opts.force.clone()),
            ("unload", opts.unload.clone()),
        ])
    }
}
