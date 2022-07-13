use derive_builder::Builder;
use nvim_types::{Array, NonOwning, Object};

use crate::api::Buffer;
use crate::trait_utils::StringOrInt;

/// Options passed to [`api::clear_autocmds`](crate::api::clear_autocmds).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct ClearAutocmdsOpts {
    /// Only clear the autocommands local to a specific `Buffer`. Cannot be
    /// used together with [`patterns`](ClearAutocmdsOptsBuilder::patterns).
    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,

    #[builder(setter(custom))]
    events: Object,

    #[builder(setter(custom))]
    group: Object,

    #[builder(setter(custom))]
    patterns: Object,
}

impl ClearAutocmdsOpts {
    /// Creates a new [`ClearAutocmdsOptsBuilder`].
    #[inline(always)]
    pub fn builder() -> ClearAutocmdsOptsBuilder {
        ClearAutocmdsOptsBuilder::default()
    }
}

impl ClearAutocmdsOptsBuilder {
    /// Clear all the autocommands triggered by one or more of the specified
    /// events.
    pub fn events<'a, I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.events = Some(Array::from_iter(iter).into());
        self
    }

    /// Only clear the autocommands matching specific patterns. For example, if
    /// you have `"*.py"` as a pattern for a particular autocommand, you must
    /// pass that exact pattern to clear it. Cannot be used together with
    /// [`buffer`](ClearAutocmdsOptsBuilder::buffer).
    pub fn patterns<'a, I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.patterns = Some(Array::from_iter(iter).into());
        self
    }

    /// Only clear the autocommands belonging to a specific augroup. The
    /// augroup can be specified by both id and name.
    pub fn group<Grp>(&mut self, group: Grp) -> &mut Self
    where
        Grp: StringOrInt,
    {
        self.group = Some(group.to_obj());
        self
    }

    pub fn build(&mut self) -> ClearAutocmdsOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_clear_autocmds<'a> {
    event: NonOwning<'a, Object>,
    group: NonOwning<'a, Object>,
    buffer: Object,
    pattern: NonOwning<'a, Object>,
}

impl<'a> From<&'a ClearAutocmdsOpts> for KeyDict_clear_autocmds<'a> {
    fn from(opts: &'a ClearAutocmdsOpts) -> Self {
        Self {
            event: opts.events.non_owning(),
            group: opts.group.non_owning(),
            buffer: opts.buffer.as_ref().into(),
            pattern: opts.patterns.non_owning(),
        }
    }
}
