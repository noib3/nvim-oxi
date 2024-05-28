use types::{Array, Object};

use crate::trait_utils::StringOrInt;
use crate::Buffer;

/// Options passed to [`get_autocmds()`](crate::get_autocmds).
#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetAutocmdsOpts {
    #[builder(mask)]
    mask: u64,

    /// Get all the autocommands triggered by one or more of the specified
    /// events.
    #[builder(
        generics = "'a, E: IntoIterator<Item = &'a str>",
        method = "events",
        argtype = "E",
        inline = "Array::from_iter({0}).into()"
    )]
    event: Object,

    /// Only get the autocommands belonging to a specific augroup. The
    /// augroup can be specified by both id and name.
    #[builder(
        generics = "G: StringOrInt",
        method = "group",
        argtype = "G",
        inline = "{0}.to_object()"
    )]
    group: Object,

    /// Only get the autocommands that match specific patterns. For example, if
    /// you have `"*.py"` as a pattern for a particular autocommand, you must
    /// pass that exact pattern to clear it. Cannot be used together with
    /// `buffer`.
    #[builder(
        generics = "'a, P: IntoIterator<Item = &'a str>",
        method = "patterns",
        argtype = "P",
        inline = "Array::from_iter({0}).into()"
    )]
    pattern: Object,

    /// Get the autocommands local to a specific `Buffer`. Cannot be used
    /// together with `patterns`.
    #[builder(argtype = "Buffer", inline = "{0}.into()")]
    buffer: Object,
}

/// Options passed to [`get_autocmds()`](crate::get_autocmds).
#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetAutocmdsOpts {
    events: Object,
    group: Object,
    buffer: Object,
    patterns: Object,
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl GetAutocmdsOpts {
    #[inline(always)]
    pub fn builder() -> GetAutocmdsOptsBuilder {
        GetAutocmdsOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Clone, Default)]
pub struct GetAutocmdsOptsBuilder(GetAutocmdsOpts);

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl GetAutocmdsOptsBuilder {
    /// Get the autocommands local to a specific `Buffer`. Cannot be used
    /// together with `patterns`.
    #[inline]
    pub fn buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.0.buffer = buffer.into();
        self
    }

    /// Get all the autocommands triggered by one or more of the specified
    /// events.
    #[inline]
    pub fn events<'a, I>(&mut self, events: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.0.events = Array::from_iter(events).into();
        self
    }

    /// Only get the autocommands belonging to a specific augroup. The
    /// augroup can be specified by both id and name.
    #[inline]
    pub fn group<Group>(&mut self, group: Group) -> &mut Self
    where
        Group: StringOrInt,
    {
        self.0.group = group.to_object();
        self
    }

    /// Only get the autocommands that match specific patterns. For example, if
    /// you have `"*.py"` as a pattern for a particular autocommand, you must
    /// pass that exact pattern to clear it. Cannot be used together with
    /// `buffer`.
    #[inline]
    pub fn patterns<'a, I>(&mut self, patterns: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.0.patterns = Array::from_iter(patterns).into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> GetAutocmdsOpts {
        std::mem::take(&mut self.0)
    }
}
