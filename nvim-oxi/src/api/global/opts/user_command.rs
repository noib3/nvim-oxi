use derive_builder::Builder;
use nvim_types::{
    dictionary::Dictionary,
    object::Object,
    string::String as NvimString,
    Integer,
};

use crate::api::types::{CommandAddr, CommandNArgs, CommandRange};
use crate::lua::LuaFn;
use crate::object::ToObject;

#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct CreateCommandOpts {
    #[builder(setter(custom))]
    addr: Option<Object>,

    bang: bool,

    bar: bool,

    #[builder(setter(custom))]
    complete: Option<Object>, // string or function

    #[builder(setter(into, strip_option))]
    count: Option<Integer>,

    #[builder(setter(into, strip_option))]
    desc: Option<NvimString>,

    #[builder(default = "true")]
    force: bool,

    keepscript: bool,

    #[builder(setter(custom))]
    nargs: Option<Object>,

    #[builder(setter(custom))]
    range: Option<Object>,

    register: bool,
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
    /// See `:h command-completion-custom` for details.
    Custom(LuaFn<(String, String, usize), Vec<String>>),
}

impl From<CreateCommandOpts> for Dictionary {
    fn from(opts: CreateCommandOpts) -> Self {
        Self::from_iter([
            ("addr", Object::from(opts.addr)),
            ("nargs", opts.nargs.into()),
            ("range", opts.range.into()),
            ("complete", opts.complete.into()),
            ("count", opts.count.into()),
            ("desc", opts.desc.into()),
            ("force", opts.force.into()),
            ("bang", opts.bang.into()),
            ("bar", opts.bar.into()),
            ("keepscript", opts.keepscript.into()),
            ("register", opts.register.into()),
        ])
    }
}

impl<'a> From<&'a CreateCommandOpts> for Dictionary {
    fn from(opts: &CreateCommandOpts) -> Self {
        opts.clone().into()
    }
}
