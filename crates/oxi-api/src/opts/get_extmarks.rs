use oxi_types as types;

/// Options passed to
/// [`Buffer::get_extmarks()`](crate::Buffer::get_extmarks).
#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetExtmarksOpts {
    details: types::Object,
    limits: types::Object,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl GetExtmarksOpts {
    #[inline(always)]
    /// Creates a new [`GetExtmarksOptsBuilder`].
    pub fn builder() -> GetExtmarksOptsBuilder {
        GetExtmarksOptsBuilder::default()
    }
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Default)]
pub struct GetExtmarksOptsBuilder(GetExtmarksOpts);

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl GetExtmarksOptsBuilder {
    /// Whether to include the extmark's
    /// [`ExtmarkInfos`](crate::types::ExtmarkInfos) as the last element of
    /// the tuples returned by
    /// [`Buffer::get_extmarks()`](crate::Buffer::get_extmarks).
    #[inline]
    pub fn details(&mut self, details: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.details = details.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.details = details;
            self.0.mask |= 0b1001;
        }
        self
    }

    #[cfg(feature = "neovim-nightly")]
    #[inline]
    pub fn hl_name(&mut self, hl_name: bool) -> &mut Self {
        self.0.hl_name = hl_name;
        self.0.mask |= 0b10001;
        self
    }

    #[inline]
    pub fn limits(&mut self, limits: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.limits = limits.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.limits = limits as Integer;
            self.0.mask |= 0b101;
        }
        self
    }

    #[cfg(feature = "neovim-nightly")]
    #[inline]
    pub fn overlap(&mut self, overlap: bool) -> &mut Self {
        self.0.overlap = overlap;
        self.0.mask |= 0b100001;
        self
    }

    #[cfg(feature = "neovim-nightly")]
    #[inline]
    pub fn ty<S: Into<nvim::String>>(&mut self, ty: S) -> &mut Self {
        self.0.ty = ty.into();
        self.0.mask |= 0b11;
        self
    }

    /// Maximum number of extmarks to return.
    #[inline]
    pub fn build(&mut self) -> GetExtmarksOpts {
        std::mem::take(&mut self.0)
    }
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl From<&GetExtmarksOpts> for types::Dictionary {
    fn from(opts: &GetExtmarksOpts) -> Self {
        Self::from_iter([
            ("details", opts.details.clone()),
            ("limits", opts.limits.clone()),
        ])
    }
}

#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, oxi_macros::OptsBuilder)]
#[repr(C)]
/// Options passed to
/// [`Buffer::get_extmarks()`](crate::Buffer::get_extmarks).
pub struct GetExtmarksOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(
        method = "limits",
        argtype = "bool",
        inline = "{0} as types::Integer"
    )]
    limit: types::Integer,

    /// Whether to include the extmark's
    /// [`ExtmarkInfos`](crate::types::ExtmarkInfos) as the last element of
    /// the tuples returned by
    /// [`Buffer::get_extmarks()`](crate::Buffer::get_extmarks).
    #[builder(argtype = "bool")]
    details: types::Boolean,

    #[builder(argtype = "bool")]
    hl_name: types::Boolean,

    #[builder(argtype = "bool")]
    overlap: types::Boolean,

    // TODO: fix `Into`.
    // TODO: name it `type` instead of `ty`.
    // #[builder(Into)]
    #[builder(
        generics = "S: Into<types::String>",
        argtype = "S",
        inline = "{0}.into()"
    )]
    ty: types::String,
}
