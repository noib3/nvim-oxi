use derive_builder::Builder;
use nvim_types::{object::Object, string::String as NvimString, Integer};

use crate::api::types::{CommandAddr, CommandNArgs, CommandRange};
use crate::lua::LuaFn;
use crate::object::ToObject;

/// Options passed to `Buffer::create_user_command`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
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
}

/// See `:h command-complete` for details.
#[non_exhaustive]
#[derive(serde::Serialize)]
pub enum CommandComplete {
    Arglist,
    Augroup,
    Buffer,
    Behave,
    Color,
    Command,
    Compiler,
    Cscope,
    Dir,
    Environment,
    Event,
    Expression,
    File,
    FileInPath,
    Filetype,
    Function,
    Help,
    Highlight,
    History,
    Locale,
    Lua,
    Mapclear,
    Mapping,
    Menu,
    Messages,
    Option,
    Packadd,
    Shellcmd,
    Sign,
    Syntax,
    Syntime,
    Tag,
    TagListfiles,
    User,
    Var,

    #[serde(skip)]
    /// See `:h command-completion-customlist` for details.
    CustomList(LuaFn<(String, String, usize), Vec<String>>),
}

// To see the generated key dicts you need to build Neovim and look in
// `/build/src/nvim/auto/keysets_defs.generated.h`.
// TODO: write a proc macro that writes this. Something like a
// ```
// #[derive(ToKeyDict)]
// ```
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct KeyDict_user_command {
    addr: Object,
    bang: Object,
    bar: Object,
    complete: Object,
    count: Object,
    desc: Object,
    force: Object,
    keepscript: Object,
    nargs: Object,
    preview: Object,
    range: Object,
    register_: Object,
}

impl<'a> From<&'a CreateCommandOpts> for KeyDict_user_command {
    fn from(opts: &CreateCommandOpts) -> Self {
        Self {
            addr: opts.addr.clone().into(),
            bang: opts.bang.into(),
            bar: opts.bar.into(),
            complete: opts.complete.clone().into(),
            count: opts.count.into(),
            desc: opts.desc.clone().into(),
            force: opts.force.into(),
            keepscript: opts.keepscript.into(),
            nargs: opts.nargs.clone().into(),
            // TODO: what's `preview`?
            preview: Object::nil(),
            range: opts.range.clone().into(),
            register_: opts.register.into(),
        }
    }
}
