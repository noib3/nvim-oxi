use derive_builder::Builder;
use nvim_types::{
    dictionary::Dictionary,
    object::Object,
    string::String as NvimString,
    Integer,
};

use crate::api::types::{CommandAddr, CommandNArgs, CommandRange};
use crate::lua::LuaFun;
use crate::object::ToObject;

#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct CreateCommandOpts {
    #[builder(setter(custom))]
    addr: Option<Object>,

    #[builder(setter(custom))]
    nargs: Option<Object>,

    #[builder(setter(custom))]
    range: Option<Object>,

    #[builder(setter(custom))]
    complete: Option<Object>, // string or function

    #[builder(setter(into, strip_option))]
    count: Option<Integer>,

    #[builder(setter(into, strip_option))]
    desc: Option<NvimString>,

    #[builder(default = "true")]
    force: bool,

    bang: bool,
    bar: bool,
    keepscript: bool,
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
            self.$name = Some(Some($name.to_obj()));
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

/// See `:h command-completion-custom` for details.
type CompleteFun = Box<
    dyn FnMut((String, String, usize)) -> crate::Result<Vec<String>> + 'static,
>;

/// See `:h command-complete` for details.
#[non_exhaustive]
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
    Custom(CompleteFun),
}

impl ToObject for CommandComplete {
    fn to_obj(self) -> Object {
        use CommandComplete::*;
        match self {
            Arglist => "arglist",
            Augroup => "augroup",
            Buffer => "buffer",
            Behave => "behave",
            Color => "color",
            Command => "command",
            Compiler => "compiler",
            Cscope => "cscope",
            Dir => "dir",
            Environment => "environment",
            Event => "event",
            Expression => "expression",
            File => "file",
            FileInPath => "file_in_path",
            Filetype => "filetype",
            Function => "function",
            Help => "help",
            Highlight => "highlight",
            History => "history",
            Locale => "locale",
            Lua => "lua",
            Mapclear => "mapclear",
            Mapping => "mapping",
            Menu => "menu",
            Messages => "messages",
            Option => "option",
            Packadd => "packadd",
            Shellcmd => "shellcmd",
            Sign => "sign",
            Syntax => "syntax",
            Syntime => "syntime",
            Tag => "tag",
            TagListfiles => "tag_listfiles",
            User => "user",
            Var => "var",
            Custom(f) => return LuaFun::from_fn_mut(f).to_obj(),
        }
        .to_obj()
    }
}

impl From<CreateCommandOpts> for Dictionary {
    fn from(opts: CreateCommandOpts) -> Self {
        Self::from_iter([
            ("addr", opts.addr.to_obj()),
            ("nargs", opts.nargs.to_obj()),
            ("range", opts.range.to_obj()),
            ("complete", opts.complete.to_obj()),
            ("count", opts.count.to_obj()),
            ("desc", opts.desc.to_obj()),
            ("force", opts.force.to_obj()),
            ("bang", opts.bang.to_obj()),
            ("bar", opts.bar.to_obj()),
            ("keepscript", opts.keepscript.to_obj()),
            ("register", opts.register.to_obj()),
        ])
    }
}

impl<'a> From<&'a CreateCommandOpts> for Dictionary {
    fn from(opts: &CreateCommandOpts) -> Self {
        opts.clone().into()
    }
}
