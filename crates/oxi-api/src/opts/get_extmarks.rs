#[cfg(feature = "neovim-nightly")]
use oxi_types::{self as nvim, Boolean, Integer, String as NvimString};
#[cfg(not(feature = "neovim-nightly"))]
use oxi_types::{Dictionary, Object};

/// Options passed to
/// [`Buffer::get_extmarks()`](crate::Buffer::get_extmarks).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetExtmarksOpts {
    details: Object,
    limits: Object,
}

#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetExtmarksOpts {
    /// <overlap><hl_name><details><limit><type>1
    mask: u64,

    /// 2nd in the mask.
    limits: Integer,

    /// 3rd in the mask.
    details: Boolean,

    /// 4th in the mask.
    hl_name: Boolean,

    /// 5th in the mask.
    overlap: Boolean,

    /// 1st in the mask.
    ty: NvimString,
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

#[cfg(not(feature = "neovim-nightly"))]
impl From<&GetExtmarksOpts> for Dictionary {
    fn from(opts: &GetExtmarksOpts) -> Self {
        Self::from_iter([
            ("details", opts.details.clone()),
            ("limits", opts.limits.clone()),
        ])
    }
}
