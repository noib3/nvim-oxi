use crate::Buffer;
use crate::{StringOrInt, StringOrListOfStrings};

/// Options passed to [`exec_autocmds()`](crate::exec_autocmds).
#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct ExecAutocmdsOpts {
    #[builder(mask)]
    mask: u64,

    /// A specific [`Buffer`] for buffer-local autocommands. Cannot be used
    /// together with [`patterns`](ExecAutocmdsOptsBuilder::patterns).
    #[builder(argtype = "Buffer", inline = "{0}.0")]
    buffer: types::BufHandle,

    /// The autocommand group name or id to match against.
    #[builder(
        generics = "G: StringOrInt",
        argtype = "G",
        inline = "{0}.to_object()"
    )]
    group: types::Object,

    /// Whether to process the modeline after the autocommands.
    #[builder(argtype = "bool")]
    modeline: types::Boolean,

    /// Patterns to match against. Cannot be used together with
    /// [`buffer`](ExecAutocmdsOptsBuilder::buffer).
    #[builder(
        generics = "P: StringOrListOfStrings",
        method = "patterns",
        argtype = "P",
        inline = "{0}.to_object()"
    )]
    pattern: types::Object,

    #[builder(
        generics = "D: Into<types::Object>",
        argtype = "D",
        inline = "{0}.into()"
    )]
    data: types::Object,
}

/// Options passed to [`exec_autocmds()`](crate::exec_autocmds).
#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct ExecAutocmdsOpts {
    data: types::Object,
    group: types::Object,
    buffer: types::Object,
    patterns: types::Object,
    modeline: types::Object,
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl ExecAutocmdsOpts {
    #[inline(always)]
    pub fn builder() -> ExecAutocmdsOptsBuilder {
        ExecAutocmdsOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Clone, Default)]
pub struct ExecAutocmdsOptsBuilder(ExecAutocmdsOpts);

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl ExecAutocmdsOptsBuilder {
    /// A specific [`Buffer`] for buffer-local autocommands. Cannot be used
    /// together with [`patterns`](ExecAutocmdsOptsBuilder::patterns).
    #[inline]
    pub fn buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.0.buffer = buffer.into();
        self
    }

    #[inline]
    pub fn data(&mut self, any: impl Into<types::Object>) -> &mut Self {
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
