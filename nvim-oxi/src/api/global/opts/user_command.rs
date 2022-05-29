use derive_builder::Builder;
use nvim_types::{Dictionary, Integer, NvimString, Object};

use crate::lua::{LuaPoppable, LuaPushable};

// IDEA:
// implement `ToObject` for all the basic types
// try to `derive(ToObject)` for structs to automatically convert arbitrary
// structs to Objects.

pub trait ToObject {
    fn to_object(self) -> Object;
}

type LuaFnMut<A, R> = Box<dyn FnMut(A) -> crate::Result<R> + 'static>;

impl<A, R> ToObject for LuaFnMut<A, R>
where
    A: LuaPoppable + 'static,
    R: LuaPushable + 'static,
{
    fn to_object(self) -> Object {
        crate::lua::mut_to_luaref(self).into()
    }
}

#[derive(Debug, Default, Builder)]
#[builder(default, pattern = "owned")]
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

macro_rules! object_setter {
    ($name:ident, $args:ident) => {
        pub fn $name(mut self, $name: $args) -> Self {
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
        .into()
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
            Zero => 0.into(),
            One => 1.into(),
            Any => "*".into(),
            ZeroOrOne => "?".into(),
            OneOrMore => "+".into(),
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
            CurrentLine => true.into(),
            WholeFile => "%".into(),
            Count(n) => n.into(),
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
    Custom(LuaFnMut<(String, String, usize), Vec<String>>),
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
        Self::from_iter::<[(_, Object); 11]>([
            ("addr", opts.addr.into()),
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
