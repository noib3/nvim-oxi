use derive_builder::Builder;
use nvim_types::{NonOwning, Object};

use crate::api::Buffer;
use crate::trait_utils::StringOrInt;

/// Options passed to `crate::api::exec_autocmds`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct ExecAutocmdsOpts {
    /// A specific `Buffer` for buffer-local autocommands. Cannot be used
    /// together with `patterns`.
    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,

    #[cfg(feature = "nightly")]
    #[builder(setter(custom))]
    data: Object,

    /// The autocommand group name or id to match against.
    #[builder(setter(custom))]
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
    #[cfg(feature = "nightly")]
    pub fn data(&mut self, any: impl Into<Object>) -> &mut Self {
        self.data = Some(any.into());
        self
    }

    pub fn group(&mut self, group: impl StringOrInt) -> &mut Self {
        self.group = Some(group.to_obj());
        self
    }

    // Up to 0.7 only strings were allowed (see
    // https://github.com/neovim/neovim/issues/19089).
    #[cfg(not(feature = "nightly"))]
    pub fn patterns(&mut self, patterns: &str) -> &mut Self {
        self.patterns = Some(patterns.into());
        self
    }

    #[cfg(feature = "nightly")]
    pub fn patterns(
        &mut self,
        patterns: impl crate::trait_utils::StringOrListOfStrings,
    ) -> &mut Self {
        self.patterns = Some(patterns.to_obj());
        self
    }

    pub fn build(&mut self) -> ExecAutocmdsOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub(crate) struct KeyDict_exec_autocmds<'a> {
    #[cfg(feature = "nightly")]
    data: NonOwning<'a, Object>,
    group: NonOwning<'a, Object>,
    buffer: Object,
    pattern: NonOwning<'a, Object>,
    modeline: Object,
}

impl<'a> From<&'a ExecAutocmdsOpts> for KeyDict_exec_autocmds<'a> {
    fn from(opts: &'a ExecAutocmdsOpts) -> KeyDict_exec_autocmds<'a> {
        Self {
            #[cfg(feature = "nightly")]
            data: opts.data.non_owning(),
            group: opts.group.non_owning(),
            buffer: opts.buffer.into(),
            pattern: opts.patterns.non_owning(),
            modeline: opts.modeline.into(),
        }
    }
}
