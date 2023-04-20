use nvim_types::Object;

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
    buffer: Object,
    group: Object,
    modeline: Object,
    patterns: Object,
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
        self.0.data = buffer.into();
        self
    }

    #[inline]
    pub fn data(&mut self, any: impl Into<Object>) -> &mut Self {
        self.0.data = any.into();
        self
    }

    /// The autocommand group name or id to match against.
    #[inline]
    pub fn group<Grp>(&mut self, group: Grp) -> &mut Self
    where
        Grp: StringOrInt,
    {
        self.0.group = group.to_object();
        self
    }

    /// Whether to process the modeline after the autocommands.
    #[inline]
    pub fn modeline(&mut self, modeline: bool) -> &mut Self {
        self.0.modeline = modeline.into();
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
        self
    }

    #[inline]
    pub fn build(&mut self) -> ExecAutocmdsOpts {
        std::mem::take(&mut self.0)
    }
}
