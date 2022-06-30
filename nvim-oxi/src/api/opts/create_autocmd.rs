use derive_builder::Builder;
use nvim_types::{Array, Object};

use crate::api::types::{AutocmdCallbackArgs, ShouldDeleteAutocmd};
use crate::api::Buffer;
use crate::lua::LuaFun;
use crate::Result;

/// Options passed to `crate::api::create_autocmd`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct CreateAutocmdOpts {
    /// A specific `Buffer` for buffer-local autocommands.
    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,

    /// Description of the autocommand.
    #[builder(setter(into, strip_option))]
    desc: Option<String>,

    /// Callback to execute when the autocommand is triggered. Cannot be used
    /// together with `command`.
    #[builder(setter(custom))]
    callback: Option<Object>,

    /// Vim command to execute when the autocommand is triggered. Cannot be
    /// used together with `callback`>
    #[builder(setter(into, strip_option))]
    command: Option<String>,

    /// The autocommand group name or id to match against.
    #[builder(setter(into, strip_option))]
    group: Option<Object>,

    /// Run nested autocommands.
    #[builder(setter(strip_option))]
    nested: Option<bool>,

    /// Only run the autocommand once.
    #[builder(setter(strip_option))]
    once: Option<bool>,

    /// Patterns to match against.
    #[builder(setter(custom))]
    patterns: Option<Object>,
}

impl CreateAutocmdOpts {
    #[inline(always)]
    pub fn builder() -> CreateAutocmdOptsBuilder {
        CreateAutocmdOptsBuilder::default()
    }
}

impl CreateAutocmdOptsBuilder {
    pub fn callback<F>(&mut self, callback: F) -> &mut Self
    where
        F: FnMut(AutocmdCallbackArgs) -> Result<ShouldDeleteAutocmd> + 'static,
    {
        self.callback = Some(Some(LuaFun::from_fn_mut(callback).into()));
        self
    }

    pub fn patterns<'a, I>(&mut self, patterns: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.patterns = Some(Some(Array::from_iter(patterns).into()));
        self
    }

    pub fn build(&mut self) -> CreateAutocmdOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_create_autocmd {
    desc: Object,
    once: Object,
    group: Object,
    buffer: Object,
    nested: Object,
    command: Object,
    pattern: Object,
    callback: Object,
}

impl From<CreateAutocmdOpts> for KeyDict_create_autocmd {
    fn from(opts: CreateAutocmdOpts) -> Self {
        Self {
            desc: opts.desc.into(),
            once: opts.once.into(),
            group: opts.group.into(),
            buffer: opts.buffer.into(),
            nested: opts.nested.into(),
            command: opts.command.into(),
            pattern: opts.patterns.into(),
            callback: opts.callback.into(),
        }
    }
}
