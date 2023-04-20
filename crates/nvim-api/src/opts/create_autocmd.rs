use nvim_types::{self as nvim, Array, Function, Object};

use crate::types::AutocmdCallbackArgs;
use crate::Buffer;
use crate::StringOrInt;

pub type ShouldDeleteAutocmd = bool;

/// Options passed to [`create_autocmd()`](crate::create_autocmd).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateAutocmdOpts {
    desc: Object,
    once: Object,
    group: Object,
    buffer: Object,
    nested: Object,
    command: Object,
    pattern: Object,
    callback: Object,
}

/// Options passed to [`create_autocmd()`](crate::create_autocmd).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateAutocmdOpts {
    buffer: Object,
    callback: Object,
    command: Object,
    desc: Object,
    group: Object,
    nested: Object,
    once: Object,
    pattern: Object,
}

impl CreateAutocmdOpts {
    #[inline(always)]
    pub fn builder() -> CreateAutocmdOptsBuilder {
        CreateAutocmdOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct CreateAutocmdOptsBuilder(CreateAutocmdOpts);

impl CreateAutocmdOptsBuilder {
    /// A specific `Buffer` for buffer-local autocommands.
    #[inline]
    pub fn buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.0.buffer = buffer.into();
        self
    }

    /// Callback to execute when the autocommand is triggered. Cannot be used
    /// together with `command`.
    #[inline]
    pub fn callback<F>(&mut self, callback: F) -> &mut Self
    where
        F: Into<Function<AutocmdCallbackArgs, ShouldDeleteAutocmd>>,
    {
        self.0.callback = callback.into().into();
        self
    }

    /// Vim command to execute when the autocommand is triggered. Cannot be
    /// used together with `callback`.
    #[inline]
    pub fn command<S>(&mut self, command: S) -> &mut Self
    where
        S: Into<nvim::String>,
    {
        self.0.command = command.into().into();
        self
    }

    /// Description of the autocommand.
    #[inline]
    pub fn desc<S>(&mut self, desc: S) -> &mut Self
    where
        S: Into<nvim::String>,
    {
        self.0.desc = desc.into().into();
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

    /// Run nested autocommands.
    #[inline]
    pub fn nested(&mut self, nested: bool) -> &mut Self {
        self.0.nested = nested.into();
        self
    }

    /// Only run the autocommand once.
    #[inline]
    pub fn once(&mut self, once: bool) -> &mut Self {
        self.0.once = once.into();
        self
    }

    /// Patterns to match against.
    #[inline]
    pub fn patterns<'a, I>(&mut self, patterns: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.0.pattern = Array::from_iter(patterns).into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> CreateAutocmdOpts {
        std::mem::take(&mut self.0)
    }
}
