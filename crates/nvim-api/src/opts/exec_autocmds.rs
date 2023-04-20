use derive_builder::Builder;
use nvim_types::{NonOwning, Object};

use crate::Buffer;
use crate::StringOrInt;

/// Options passed to [`api::exec_autocmds`](crate::exec_autocmds).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct ExecAutocmdsOpts {
    /// A specific [`Buffer`] for buffer-local autocommands. Cannot be used
    /// together with [`patterns`](ExecAutocmdsOptsBuilder::patterns).
    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,

    #[builder(setter(custom))]
    data: Object,

    #[builder(setter(custom))]
    group: Object,

    /// Whether to process the modeline after the autocommands.
    modeline: bool,

    #[builder(setter(custom))]
    patterns: Object,
}

impl ExecAutocmdsOpts {
    #[inline(always)]
    pub fn builder() -> ExecAutocmdsOptsBuilder {
        ExecAutocmdsOptsBuilder::default()
    }
}

impl ExecAutocmdsOptsBuilder {
    pub fn data(&mut self, any: impl Into<Object>) -> &mut Self {
        self.data = Some(any.into());
        self
    }

    /// The autocommand group name or id to match against.
    pub fn group<Grp>(&mut self, group: Grp) -> &mut Self
    where
        Grp: StringOrInt,
    {
        self.group = Some(group.to_object());
        self
    }

    /// Patterns to match against. Cannot be used together with
    /// [`buffer`](ExecAutocmdsOptsBuilder::buffer).
    pub fn patterns<Patterns>(&mut self, patterns: Patterns) -> &mut Self
    where
        Patterns: crate::trait_utils::StringOrListOfStrings,
    {
        self.patterns = Some(patterns.to_object());
        self
    }

    pub fn build(&mut self) -> ExecAutocmdsOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[cfg(not(feature = "neovim-nightly"))]
#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_exec_autocmds<'a> {
    data: NonOwning<'a, Object>,
    group: NonOwning<'a, Object>,
    buffer: Object,
    pattern: NonOwning<'a, Object>,
    modeline: Object,
}

#[cfg(feature = "neovim-nightly")]
#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_exec_autocmds<'a> {
    buffer: Object,
    group: NonOwning<'a, Object>,
    modeline: Object,
    pattern: NonOwning<'a, Object>,
    data: NonOwning<'a, Object>,
}

impl<'a> From<&'a ExecAutocmdsOpts> for KeyDict_exec_autocmds<'a> {
    fn from(opts: &'a ExecAutocmdsOpts) -> KeyDict_exec_autocmds<'a> {
        Self {
            data: opts.data.non_owning(),
            group: opts.group.non_owning(),
            buffer: opts.buffer.as_ref().into(),
            pattern: opts.patterns.non_owning(),
            modeline: opts.modeline.into(),
        }
    }
}
