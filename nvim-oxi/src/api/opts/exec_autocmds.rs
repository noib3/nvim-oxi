use derive_builder::Builder;
use nvim_types::{Array, NonOwning, Object};

use crate::api::Buffer;

/// Options passed to `crate::api::exec_autocmds`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct ExecAutocmdsOpts {
    /// A specific `Buffer` for buffer-local autocommands. Cannot be used
    /// together with `patterns`.
    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,

    // TODO: what to put here?
    data: (),

    /// The autocommand group name or id to match against.
    #[builder(setter(into))]
    group: Object,

    /// Whether to process the modeline after the autocommands.
    modeline: bool,

    /// Patterns to match against. Cannot be used together with `buffer`.
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
    pub fn patterns<'a, I>(&mut self, patterns: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.patterns = Some(Array::from_iter(patterns).into());
        self
    }

    pub fn build(&mut self) -> ExecAutocmdsOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_exec_autocmds<'a> {
    data: Object,
    group: NonOwning<'a, Object>,
    buffer: Object,
    pattern: NonOwning<'a, Object>,
    modeline: Object,
}

impl<'a> From<&'a ExecAutocmdsOpts> for KeyDict_exec_autocmds<'a> {
    fn from(opts: &'a ExecAutocmdsOpts) -> KeyDict_exec_autocmds<'a> {
        Self {
            data: opts.data.into(),
            group: opts.group.non_owning(),
            buffer: opts.buffer.into(),
            pattern: opts.patterns.non_owning(),
            modeline: opts.modeline.into(),
        }
    }
}
