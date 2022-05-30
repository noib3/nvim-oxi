use derive_builder::Builder;
use nvim_types::{
    dictionary::Dictionary,
    object::Object,
    string::String as NvimString,
    Integer,
};

use crate::object::ToObject;

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct UserCommandOpts {
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

impl UserCommandOpts {
    #[inline(always)]
    pub fn builder() -> UserCommandOptsBuilder {
        UserCommandOptsBuilder::default()
    }
}

macro_rules! object_setter {
    ($name:ident, $args:ident) => {
        pub fn $name(&mut self, $name: $args) -> &mut Self {
            self.$name = Some(Some($name.into()));
            self
        }
    };
}

impl UserCommandOptsBuilder {
    object_setter!(addr, CommandAddr);

    object_setter!(nargs, CommandNArgs);

    object_setter!(range, CommandRange);

    // object_setter!(complete, CommandComplete);
}

/// See `:h command-addr` for details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum CommandAddr {
    Lines,
    Arguments,
    Buffers,
    LoadedBuffers,
    Windows,
    Tabs,
    Quickfix,
    Other,
}

impl From<CommandAddr> for Object {
    fn from(addr: CommandAddr) -> Self {
        use CommandAddr::*;
        match addr {
            Lines => "lines",
            Arguments => "arguments",
            Buffers => "buffers",
            LoadedBuffers => "loaded_buffers",
            Windows => "windows",
            Tabs => "tabs",
            Quickfix => "quickfix",
            Other => "other",
        }
        .to_obj()
    }
}

/// Number of arguments accepted by the command. See `:h command-nargs` for
/// details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum CommandNArgs {
    // #[object("0")]
    Zero,

    // #[object("1")]
    One,

    // #[object("\"*\"")]
    Any,

    // #[object("\"?\"")]
    ZeroOrOne,

    // #[object("\"+\"")]
    OneOrMore,
}

impl From<CommandNArgs> for Object {
    fn from(nargs: CommandNArgs) -> Self {
        use CommandNArgs::*;
        match nargs {
            Zero => 0.to_obj(),
            One => 1.to_obj(),
            Any => "*".to_obj(),
            ZeroOrOne => "?".to_obj(),
            OneOrMore => "+".to_obj(),
        }
    }
}

/// See `:h command-range` for details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum CommandRange {
    // #[object("true")]
    CurrentLine,

    // #[object("\"%\"")]
    WholeFile,

    Count(u32),
}

impl From<CommandRange> for Object {
    fn from(range: CommandRange) -> Self {
        use CommandRange::*;
        match range {
            CurrentLine => true.to_obj(),
            WholeFile => "%".to_obj(),
            Count(n) => n.to_obj(),
        }
    }
}

/// See `:h command-complete` for details.
#[non_exhaustive]
// #[derive(ToObject)]
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
    // Custom(LuaFnMut<(String, String, usize), Vec<String>>),
}

// impl<F: FunctionMut<(String, String, usize), Vec<String>>> ToObject
//     for CommandComplete
// {
//     fn into(&self) -> Object {
//         use CommandComplete::*;
//         match self {
//             Arglist => "arglist",
//             Augroup => "augroup",
//             Buffer => "buffer",
//             Behave => "behave",
//             Color => "color",
//             Command => "command",
//             Compiler => "compiler",
//             Cscope => "cscope",
//             Dir => "dir",
//             Environment => "environment",
//             Event => "event",
//             Expression => "expression",
//             File => "file",
//             FileInPath => "file_in_path",
//             Filetype => "filetype",
//             Function => "function",
//             Help => "help",
//             Highlight => "highlight",
//             History => "history",
//             Locale => "locale",
//             Lua => "lua",
//             Mapclear => "mapclear",
//             Mapping => "mapping",
//             Menu => "menu",
//             Messages => "messages",
//             Option => "option",
//             Packadd => "packadd",
//             Shellcmd => "shellcmd",
//             Sign => "sign",
//             Syntax => "syntax",
//             Syntime => "syntime",
//             Tag => "tag",
//             TagListFiles => "tag_listfiles",
//             User => "user",
//             Var => "var",
//             Custom(f) => return f.into(),
//         }
//         .into()
//     }
// }

impl From<UserCommandOpts> for Dictionary {
    fn from(opts: UserCommandOpts) -> Self {
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
