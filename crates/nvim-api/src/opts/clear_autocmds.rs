use nvim_types::{Array, Object};

use crate::Buffer;
use crate::StringOrInt;

/// Options passed to [`clear_autocmds()`](crate::clear_autocmds).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct ClearAutocmdsOpts {
    event: Object,
    group: Object,
    buffer: Object,
    pattern: Object,
}

/// Options passed to [`clear_autocmds()`](crate::clear_autocmds).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct ClearAutocmdsOpts {
    buffer: Object,
    event: Object,
    group: Object,
    pattern: Object,
}

impl ClearAutocmdsOpts {
    /// Creates a new [`ClearAutocmdsOptsBuilder`].
    #[inline(always)]
    pub fn builder() -> ClearAutocmdsOptsBuilder {
        ClearAutocmdsOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct ClearAutocmdsOptsBuilder(ClearAutocmdsOpts);

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
        self.0.event = Array::from_iter(iter).into();
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
        self.0.pattern = Array::from_iter(iter).into();
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
