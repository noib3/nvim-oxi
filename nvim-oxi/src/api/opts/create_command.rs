use derive_builder::Builder;
use nvim_types::{self as nvim, Integer, NonOwning, Object};

use crate::api::types::{
    CommandAddr,
    CommandComplete,
    CommandNArgs,
    CommandRange,
};
use crate::object::ToObject;

/// Options passed to `Buffer::create_user_command`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct CreateCommandOpts {
    #[builder(setter(custom))]
    addr: Object,

    #[builder(setter(strip_option))]
    bang: Option<bool>,

    #[builder(setter(strip_option))]
    bar: Option<bool>,

    #[builder(setter(custom))]
    complete: Object,

    #[builder(setter(into, strip_option))]
    count: Option<Integer>,

    #[builder(setter(custom))]
    desc: Object,

    #[builder(setter(strip_option))]
    /// Whether to override any previous definitions. Defaults to `true`.
    force: Option<bool>,

    #[builder(setter(strip_option))]
    keepscript: Option<bool>,

    #[builder(setter(custom))]
    nargs: Object,

    #[cfg(feature = "nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
    #[builder(setter(custom))]
    preview: Object,

    #[builder(setter(custom))]
    range: Object,

    #[builder(setter(strip_option))]
    register: Option<bool>,
}

impl CreateCommandOpts {
    #[inline(always)]
    /// Creates a new [`CreateCommandOptsBuilder`].
    pub fn builder() -> CreateCommandOptsBuilder {
        CreateCommandOptsBuilder::default()
    }
}

macro_rules! object_setter {
    ($name:ident, $args:ident) => {
        pub fn $name(&mut self, $name: $args) -> &mut Self {
            self.$name = Some($name.to_obj().unwrap());
            self
        }
    };
}

impl CreateCommandOptsBuilder {
    object_setter!(addr, CommandAddr);
    object_setter!(complete, CommandComplete);
    object_setter!(nargs, CommandNArgs);
    object_setter!(range, CommandRange);

    /// Description for the command.
    pub fn desc(&mut self, desc: impl Into<nvim::String>) -> &mut Self {
        self.desc = Some(desc.into().into());
        self
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
    pub fn preview<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(
                (
                    crate::api::types::CommandArgs,
                    Option<u32>,
                    Option<crate::api::Buffer>,
                ),
            ) -> crate::Result<u8>
            + 'static,
    {
        self.preview = Some(crate::lua::Function::from_fn_mut(f).into());
        self
    }

    pub fn build(&mut self) -> CreateCommandOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

// To see the generated key dicts you need to build Neovim and look in
// `/build/src/nvim/auto/keysets_defs.generated.h`.
#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_user_command<'a> {
    bar: Object,
    addr: NonOwning<'a, Object>,
    bang: Object,
    desc: NonOwning<'a, Object>,
    count: Object,
    force: Object,
    nargs: NonOwning<'a, Object>,
    range: NonOwning<'a, Object>,
    #[cfg(feature = "nightly")]
    preview: NonOwning<'a, Object>,
    complete: NonOwning<'a, Object>,
    register_: Object,
    keepscript: Object,
}

impl<'a> From<&'a CreateCommandOpts> for KeyDict_user_command<'a> {
    fn from(opts: &'a CreateCommandOpts) -> Self {
        Self {
            bar: opts.bar.into(),
            addr: opts.addr.non_owning(),
            bang: opts.bang.into(),
            desc: opts.desc.non_owning(),
            count: opts.count.into(),
            force: opts.force.into(),
            nargs: opts.nargs.non_owning(),
            range: opts.range.non_owning(),
            #[cfg(feature = "nightly")]
            preview: opts.preview.non_owning(),
            complete: opts.complete.non_owning(),
            register_: opts.register.into(),
            keepscript: opts.keepscript.into(),
        }
    }
}
