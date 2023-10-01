use oxi_types::Object;
#[cfg(feature = "neovim-nightly")]
use oxi_types::{Boolean, BufHandle};

use crate::Buffer;
use crate::{StringOrInt, StringOrListOfStrings};

/// Options passed to [`exec_autocmds()`](crate::exec_autocmds).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct ExecAutocmdsOpts {
    data: Object,
    group: Object,
    buffer: Object,
    patterns: Object,
    modeline: Object,
}

/// Options passed to [`exec_autocmds()`](crate::exec_autocmds).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct ExecAutocmdsOpts {
    /// <modeline><patterns><buffer><group><data>1
    mask: u64,

    /// 3rd in the mask.
    buffer: BufHandle,

    /// 2nd in the mask.
    group: Object,

    /// 5th in the mask.
    modeline: Boolean,

    /// 4th in the mask.
    patterns: Object,

    /// 1st in the mask.
    data: Object,
}

impl ExecAutocmdsOpts {
    #[inline(always)]
    pub fn builder() -> ExecAutocmdsOptsBuilder {
        ExecAutocmdsOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct ExecAutocmdsOptsBuilder(ExecAutocmdsOpts);

impl ExecAutocmdsOptsBuilder {
    /// A specific [`Buffer`] for buffer-local autocommands. Cannot be used
    /// together with [`patterns`](ExecAutocmdsOptsBuilder::patterns).
    #[inline]
    pub fn buffer(&mut self, buffer: Buffer) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.buffer = buffer.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.buffer = buffer.0;
            self.0.mask |= 0b1001;
        }
        self
    }

    #[inline]
    pub fn data(&mut self, any: impl Into<Object>) -> &mut Self {
        self.0.data = any.into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b11;
        }
        self
    }

    /// The autocommand group name or id to match against.
    #[inline]
    pub fn group<Grp>(&mut self, group: Grp) -> &mut Self
    where
        Grp: StringOrInt,
    {
        self.0.group = group.to_object();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b101;
        }
        self
    }

    /// Whether to process the modeline after the autocommands.
    #[inline]
    pub fn modeline(&mut self, modeline: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.modeline = modeline.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.modeline = modeline;
            self.0.mask |= 0b100001;
        }
        self
    }

    /// Patterns to match against. Cannot be used together with
    /// [`buffer`](ExecAutocmdsOptsBuilder::buffer).
    #[inline]
    pub fn patterns<Patterns>(&mut self, patterns: Patterns) -> &mut Self
    where
        Patterns: StringOrListOfStrings,
    {
        self.0.patterns = patterns.to_object();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10001;
        }
        self
    }

    #[inline]
    pub fn build(&mut self) -> ExecAutocmdsOpts {
        std::mem::take(&mut self.0)
    }
}
