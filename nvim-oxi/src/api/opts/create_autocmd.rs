use derive_builder::Builder;
use nvim_types::{self as nvim, Array, NonOwning, Object};

use crate::api::types::AutocmdCallbackArgs;
use crate::api::Buffer;
use crate::lua::Function;
use crate::trait_utils::StringOrInt;
use crate::Result;

pub type ShouldDeleteAutocmd = bool;

/// Options passed to `crate::api::create_autocmd`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct CreateAutocmdOpts {
    /// A specific `Buffer` for buffer-local autocommands.
    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,

    /// Description of the autocommand.
    #[builder(setter(custom))]
    desc: Object,

    /// Callback to execute when the autocommand is triggered. Cannot be used
    /// together with `command`.
    #[builder(setter(custom))]
    callback: Object,

    /// Vim command to execute when the autocommand is triggered. Cannot be
    /// used together with `callback`.
    #[builder(setter(custom))]
    command: Object,

    /// The autocommand group name or id to match against.
    #[builder(setter(custom))]
    group: Object,

    /// Run nested autocommands.
    #[builder(setter(strip_option))]
    nested: Option<bool>,

    /// Only run the autocommand once.
    #[builder(setter(strip_option))]
    once: Option<bool>,

    /// Patterns to match against.
    #[builder(setter(custom))]
    patterns: Object,
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
        self.callback = Some(Function::from_fn_mut(callback).into());
        self
    }

    pub fn command(&mut self, command: impl Into<nvim::String>) -> &mut Self {
        self.command = Some(command.into().into());
        self
    }

    pub fn desc(&mut self, desc: impl Into<nvim::String>) -> &mut Self {
        self.desc = Some(desc.into().into());
        self
    }

    pub fn group<Grp>(&mut self, group: Grp) -> &mut Self
    where
        Grp: StringOrInt,
    {
        self.group = Some(group.to_obj());
        self
    }

    pub fn patterns<'a, I>(&mut self, patterns: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.patterns = Some(Array::from_iter(patterns).into());
        self
    }

    pub fn build(&mut self) -> CreateAutocmdOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_create_autocmd<'a> {
    desc: NonOwning<'a, Object>,
    once: Object,
    group: NonOwning<'a, Object>,
    buffer: Object,
    nested: Object,
    command: NonOwning<'a, Object>,
    pattern: NonOwning<'a, Object>,
    callback: NonOwning<'a, Object>,
}

impl<'a> From<&'a CreateAutocmdOpts> for KeyDict_create_autocmd<'a> {
    fn from(opts: &'a CreateAutocmdOpts) -> Self {
        Self {
            desc: opts.desc.non_owning(),
            once: opts.once.into(),
            group: opts.group.non_owning(),
            buffer: opts.buffer.as_ref().into(),
            nested: opts.nested.into(),
            command: opts.command.non_owning(),
            pattern: opts.patterns.non_owning(),
            callback: opts.callback.non_owning(),
        }
    }
}
