use crate::Buffer;
use crate::StringOrInt;

/// Options passed to [`clear_autocmds()`](crate::clear_autocmds).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct ClearAutocmdsOpts {
    event: types::Object,
    group: types::Object,
    buffer: types::Object,
    pattern: types::Object,
}

#[cfg(not(feature = "neovim-nightly"))]
impl ClearAutocmdsOpts {
    /// Creates a new [`ClearAutocmdsOptsBuilder`].
    #[inline(always)]
    pub fn builder() -> ClearAutocmdsOptsBuilder {
        ClearAutocmdsOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Default)]
pub struct ClearAutocmdsOptsBuilder(ClearAutocmdsOpts);

#[cfg(not(feature = "neovim-nightly"))]
impl ClearAutocmdsOptsBuilder {
    /// Only clear the autocommands local to a specific `Buffer`. Cannot be
    /// used together with [`patterns`](ClearAutocmdsOptsBuilder::patterns).
    #[inline]
    pub fn buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.0.buffer = buffer.into();
        self
    }

    /// Clear all the autocommands triggered by one or more of the specified
    /// events.
    #[inline]
    pub fn events<'a, I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.0.event = types::Array::from_iter(iter).into();
        self
    }

    /// Only clear the autocommands matching specific patterns. For example, if
    /// you have `"*.py"` as a pattern for a particular autocommand, you must
    /// pass that exact pattern to clear it. Cannot be used together with
    /// [`buffer`](ClearAutocmdsOptsBuilder::buffer).
    #[inline]
    pub fn patterns<'a, I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.0.pattern = types::Array::from_iter(iter).into();
        self
    }

    /// Only clear the autocommands belonging to a specific augroup. The
    /// augroup can be specified by both id and name.
    #[inline]
    pub fn group<Grp>(&mut self, group: Grp) -> &mut Self
    where
        Grp: StringOrInt,
    {
        self.0.group = group.to_object();
        self
    }

    #[inline]
    pub fn build(&mut self) -> ClearAutocmdsOpts {
        std::mem::take(&mut self.0)
    }
}

/// Options passed to [`clear_autocmds()`](crate::clear_autocmds).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct ClearAutocmdsOpts {
    #[builder(mask)]
    mask: u64,

    /// Only clear the autocommands local to a specific `Buffer`. Cannot be
    /// used together with [`patterns`](ClearAutocmdsOptsBuilder::patterns).
    #[builder(argtype = "Buffer", inline = "{0}.0")]
    buffer: types::BufHandle,

    /// Clear all the autocommands triggered by one or more of the specified
    /// events.
    #[builder(
        generics = "'a, I: IntoIterator<Item = &'a str>",
        method = "events",
        argtype = "I",
        inline = "types::Array::from_iter({0}).into()"
    )]
    event: types::Object,

    /// Only clear the autocommands matching specific patterns. For example, if
    /// you have `"*.py"` as a pattern for a particular autocommand, you must
    /// pass that exact pattern to clear it. Cannot be used together with
    /// [`buffer`](ClearAutocmdsOptsBuilder::buffer).
    #[builder(
        generics = "G: StringOrInt",
        argtype = "G",
        inline = "{0}.to_object()"
    )]
    group: types::Object,

    /// Only clear the autocommands belonging to a specific augroup. The
    /// augroup can be specified by both id and name.
    #[builder(
        generics = "'a, I: IntoIterator<Item = &'a str>",
        method = "patterns",
        argtype = "I",
        inline = "types::Array::from_iter({0}).into()"
    )]
    pattern: types::Object,
}
