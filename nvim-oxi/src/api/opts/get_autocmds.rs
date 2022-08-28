use derive_builder::Builder;
use nvim_types::{Array, NonOwning, Object};

use crate::api::Buffer;

/// Options passed to [`nvim_oxi::api::get_autocmds`](crate::api::get_autocmds).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetAutocmdsOpts {
    /// Get the autocommands local to a specific `Buffer`. Cannot be used
    /// together with `patterns`.
    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,

    /// Get all the autocommands triggered by one or more of the specified
    /// events.
    #[builder(setter(custom))]
    events: Object,

    /// Only get the autocommands belonging to a specific augroup. The
    /// augroup can be specified by both id and name.
    #[builder(setter(into))]
    group: Object,

    /// Only get the autocommands that match specific patterns. For example, if
    /// you have `"*.py"` as a pattern for a particular autocommand, you must
    /// pass that exact pattern to clear it. Cannot be used together with
    /// `buffer`.
    #[builder(setter(custom))]
    patterns: Object,
}

impl GetAutocmdsOpts {
    #[inline(always)]
    pub fn builder() -> GetAutocmdsOptsBuilder {
        GetAutocmdsOptsBuilder::default()
    }
}

macro_rules! string_or_table {
    ($fn_name:ident) => {
        pub fn $fn_name<'a, I>(&mut self, iter: I) -> &mut Self
        where
            I: IntoIterator<Item = &'a str>,
        {
            self.$fn_name = Some(Array::from_iter(iter).into());
            self
        }
    };
}

impl GetAutocmdsOptsBuilder {
    string_or_table!(events);
    string_or_table!(patterns);

    pub fn build(&mut self) -> GetAutocmdsOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_get_autocmds<'a> {
    event: NonOwning<'a, Object>,
    group: NonOwning<'a, Object>,
    buffer: Object,
    pattern: NonOwning<'a, Object>,
}

impl<'a> From<&'a GetAutocmdsOpts> for KeyDict_get_autocmds<'a> {
    fn from(opts: &'a GetAutocmdsOpts) -> Self {
        Self {
            event: opts.events.non_owning(),
            group: opts.group.non_owning(),
            buffer: opts.buffer.as_ref().into(),
            pattern: opts.patterns.non_owning(),
        }
    }
}
