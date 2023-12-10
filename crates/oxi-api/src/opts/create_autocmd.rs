use oxi_types as types;

use crate::types::AutocmdCallbackArgs;
use crate::Buffer;
use crate::StringOrInt;

pub type ShouldDeleteAutocmd = bool;

/// Options passed to [`create_autocmd()`](crate::create_autocmd).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, oxi_macros::OptsBuilder)]
#[repr(C)]
pub struct CreateAutocmdOpts {
    #[builder(mask)]
    mask: u64,

    /// A specific `Buffer` for buffer-local autocommands.
    #[builder(argtype = "Buffer", inline = "{0}.0")]
    buffer: types::BufHandle,

    /// Callback to execute when the autocommand is triggered. Cannot be used
    /// together with `command`.
    #[builder(
        generics = r#"F: Into<types::Function<AutocmdCallbackArgs, ShouldDeleteAutocmd>>"#,
        argtype = "F",
        inline = "{0}.into().into()"
    )]
    callback: types::Object,

    /// Vim command to execute when the autocommand is triggered. Cannot be
    /// used together with `callback`.
    // TODO: fix builder(Into).
    #[builder(
        generics = "S: Into<types::String>",
        argtype = "S",
        inline = "{0}.into()"
    )]
    command: types::String,

    /// Description of the autocommand.
    // TODO: fix builder(Into).
    #[builder(
        generics = "S: Into<types::String>",
        argtype = "S",
        inline = "{0}.into()"
    )]
    desc: types::String,

    /// The autocommand group name or id to match against.
    #[builder(
        generics = "G: StringOrInt",
        argtype = "G",
        inline = "{0}.to_object()"
    )]
    group: types::Object,

    /// Run nested autocommands.
    #[builder(argtype = "bool")]
    nested: types::Boolean,

    /// Only run the autocommand once.
    #[builder(argtype = "bool")]
    once: types::Boolean,

    /// Patterns to match against.
    #[builder(
        generics = "'a, I: IntoIterator<Item = &'a str>",
        method = "patterns",
        argtype = "I",
        inline = "types::Array::from_iter({0}).into()"
    )]
    pattern: types::Object,
}

/// Options passed to [`create_autocmd()`](crate::create_autocmd).
#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateAutocmdOpts {
    desc: types::Object,
    once: types::Object,
    group: types::Object,
    buffer: types::Object,
    nested: types::Object,
    command: types::Object,
    pattern: types::Object,
    callback: types::Object,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl CreateAutocmdOpts {
    #[inline(always)]
    pub fn builder() -> CreateAutocmdOptsBuilder {
        CreateAutocmdOptsBuilder::default()
    }
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Default)]
pub struct CreateAutocmdOptsBuilder(CreateAutocmdOpts);

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
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
        F: Into<types::Function<AutocmdCallbackArgs, ShouldDeleteAutocmd>>,
    {
        self.0.callback = callback.into().into();
        self
    }

    /// Vim command to execute when the autocommand is triggered. Cannot be
    /// used together with `callback`.
    #[inline]
    pub fn command<S>(&mut self, command: S) -> &mut Self
    where
        S: Into<types::String>,
    {
        self.0.command = command.into().into();
        self
    }

    /// Description of the autocommand.
    #[inline]
    pub fn desc<S>(&mut self, desc: S) -> &mut Self
    where
        S: Into<types::String>,
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
        self.0.pattern = types::Array::from_iter(patterns).into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> CreateAutocmdOpts {
        std::mem::take(&mut self.0)
    }
}
