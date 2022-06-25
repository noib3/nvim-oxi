use derive_builder::Builder;
use nvim_types::{Array, Object};

use crate::api::Buffer;

/// Options passed to `crate::api::clear_autocmds`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct ClearAutocmdsOpts {
    /// Only clear the autocommands local to a specific `Buffer`. Cannot be
    /// used together with `patterns`.
    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,

    /// Clear all the autocommands triggered by one or more of the specified
    /// events.
    #[builder(setter(custom))]
    events: Option<Object>,

    /// Only clear the autocommands belonging to a specific augroup. The
    /// augroup can be specified by both id and name.
    #[builder(setter(into, strip_option))]
    group: Option<Object>,

    /// Only clear the autocommands that match a specific pattern(s). For
    /// example, if you have `"*.py"` as a pattern for a particular
    /// autocommand, you must pass that exact pattern to clear it. Cannot be
    /// used together with `buffer`.
    #[builder(setter(custom))]
    patterns: Option<Object>,
}

impl ClearAutocmdsOpts {
    #[inline(always)]
    pub fn builder() -> ClearAutocmdsOptsBuilder {
        ClearAutocmdsOptsBuilder::default()
    }
}

macro_rules! string_or_table {
    ($fn_name:ident) => {
        pub fn $fn_name<'a, I>(&mut self, iter: I) -> &mut Self
        where
            I: IntoIterator<Item = &'a str>,
        {
            self.$fn_name = Some(Some(
                iter.into_iter().map(Object::from).collect::<Array>().into(),
            ));
            self
        }
    };
}

impl ClearAutocmdsOptsBuilder {
    string_or_table!(events);
    string_or_table!(patterns);

    pub fn build(&mut self) -> ClearAutocmdsOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_clear_autocmds {
    event: Object,
    group: Object,
    buffer: Object,
    pattern: Object,
}

impl From<ClearAutocmdsOpts> for KeyDict_clear_autocmds {
    fn from(opts: ClearAutocmdsOpts) -> Self {
        Self {
            event: opts.events.into(),
            group: opts.group.into(),
            buffer: opts.buffer.map(|buf| buf.0).into(),
            pattern: opts.patterns.into(),
        }
    }
}
