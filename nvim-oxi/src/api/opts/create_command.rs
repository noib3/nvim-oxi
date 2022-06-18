use derive_builder::Builder;
use nvim_types::{Integer, Object, String as NvimString};

use crate::api::types::{
    CommandAddr,
    CommandArgs,
    CommandComplete,
    CommandNArgs,
    CommandRange,
};
use crate::api::Buffer;
use crate::lua::LuaFun;
use crate::object::ToObject;

/// Options passed to `Buffer::create_user_command`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct CreateCommandOpts {
    #[builder(setter(custom))]
    addr: Option<Object>,

    #[builder(setter(strip_option))]
    bang: Option<bool>,

    #[builder(setter(strip_option))]
    bar: Option<bool>,

    #[builder(setter(custom))]
    complete: Option<Object>,

    #[builder(setter(into, strip_option))]
    count: Option<Integer>,

    #[builder(setter(into, strip_option))]
    desc: Option<NvimString>,

    #[builder(setter(strip_option))]
    force: Option<bool>,

    #[builder(setter(strip_option))]
    keepscript: Option<bool>,

    #[builder(setter(custom))]
    nargs: Option<Object>,

    #[builder(setter(custom))]
    preview: Option<Object>,

    #[builder(setter(custom))]
    range: Option<Object>,

    #[builder(setter(strip_option))]
    register: Option<bool>,
}

impl CreateCommandOpts {
    #[inline(always)]
    pub fn builder() -> CreateCommandOptsBuilder {
        CreateCommandOptsBuilder::default()
    }
}

macro_rules! object_setter {
    ($name:ident, $args:ident) => {
        pub fn $name(&mut self, $name: $args) -> &mut Self {
            self.$name = Some(Some($name.to_obj().unwrap()));
            self
        }
    };
}

impl CreateCommandOptsBuilder {
    object_setter!(addr, CommandAddr);
    object_setter!(nargs, CommandNArgs);
    object_setter!(range, CommandRange);
    object_setter!(complete, CommandComplete);

    pub fn preview<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(
                (CommandArgs, Option<u32>, Option<Buffer>),
            ) -> crate::Result<u8>
            + 'static,
    {
        self.preview = Some(Some(LuaFun::from_fn_mut(f).into()));
        self
    }

    pub fn build(&mut self) -> CreateCommandOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

// To see the generated key dicts you need to build Neovim and look in
// `/build/src/nvim/auto/keysets_defs.generated.h`.
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_user_command {
    bar: Object,
    addr: Object,
    bang: Object,
    desc: Object,
    count: Object,
    force: Object,
    nargs: Object,
    range: Object,
    preview: Object,
    complete: Object,
    register_: Object,
    keepscript: Object,
}

impl<'a> From<&'a CreateCommandOpts> for KeyDict_user_command {
    fn from(opts: &CreateCommandOpts) -> Self {
        Self {
            bar: opts.bar.into(),
            addr: opts.addr.clone().into(),
            bang: opts.bang.into(),
            desc: opts.desc.clone().into(),
            count: opts.count.into(),
            force: opts.force.into(),
            nargs: opts.nargs.clone().into(),
            range: opts.range.clone().into(),
            preview: opts.preview.clone().into(),
            complete: opts.complete.clone().into(),
            register_: opts.register.into(),
            keepscript: opts.keepscript.into(),
        }
    }
}
