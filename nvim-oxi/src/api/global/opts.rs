use nvim_types::Object;

// IDEA:
// implement `ToObject` for all the basic types
// try to `derive(ToObject)` for structs to automatically convert arbitrary
// structs to Objects.

pub struct UserCommandOpts {}

// #[derive(Clone, Debug, Default, ToObject)]
// pub struct UserCommandOpts {
//     addr: Option<CommandAddr>,
//     bang: Option<bool>,
//     bar: Option<bool>,
//     complete: Option<Object>, // string or function
//     count: Option<u32>,       // bool or int
//     desc: Option<String>,
//     force: Option<bool>,
//     keepscript: Option<bool>,
//     nargs: Option<CommandNArgs>,
//     range: Option<CommandRange>,
//     register: Option<bool>,
// }

// /// See `:h command-addr` for details.
// #[non_exhaustive]
// #[derive(Copy, Clone, Debug)]
// pub enum CommandAddr {
//     Lines,
//     Arguments,
//     Buffers,
//     LoadedBuffers,
//     Windows,
//     Tabs,
//     Quickfix,
//     Other,
// }

// impl ToObject for CommandAddr {
//     fn into(&self) -> Object {
//         use CommandAddr::*;
//         match self {
//             Lines => "lines",
//             Arguments => "arguments",
//             Buffers => "buffers",
//             LoadedBuffers => "loaded_buffers",
//             Windows => "windows",
//             Tabs => "tabs",
//             Quickfixm => "quickfix",
//             Other => "other",
//             _ => return ().into(),
//         }
//         .into()
//     }
// }

// /// See `:h command-complete` for details.
// #[non_exhaustive]
// #[derive(Clone)]
// pub enum CommandComplete<F: FunctionMut<(String, String, usize), Vec<String>>>
// {
//     Arglist,
//     Augroup,
//     Buffer,
//     Behave,
//     Color,
//     Command,
//     Compiler,
//     Cscope,
//     Dir,
//     Environment,
//     Event,
//     Expression,
//     File,
//     FileInPath,
//     Filetype,
//     Function,
//     Help,
//     Highlight,
//     History,
//     Locale,
//     Lua,
//     Mapclear,
//     Mapping,
//     Menu,
//     Messages,
//     Option,
//     Packadd,
//     Shellcmd,
//     Sign,
//     Syntax,
//     Syntime,
//     Tag,
//     TagListFiles,
//     User,
//     Var,
//     Custom(F),
// }

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

// /// Number of arguments accepted by the command. See `:h command-nargs` for
// /// details.
// #[non_exhaustive]
// #[derive(Copy, Clone, Debug)]
// pub enum CommandNArgs {
//     Zero,      // 0 (default)
//     One,       // 1
//     Any,       // "*"
//     ZeroOrOne, // "?"
//     OneOrMore, // "+"
// }

// impl ToObject for CommandNArgs {
//     fn into(&self) -> Object {
//         use CommandNArgs::*;
//         match self {
//             Zero => 0.into(),
//             One => 1.into(),
//             Any => "*".into(),
//             ZeroOrOne => "?".into(),
//             OneOrMore => "+".into(),
//         }
//     }
// }

// /// See `:h command-range` for details.
// #[non_exhaustive]
// #[derive(Copy, Clone, Debug)]
// pub enum CommandRange {
//     CurrentLine,
//     WholeFile,
//     Count(u32),
// }

// impl ToObject for CommandRange {
//     fn into(&self) -> Object {
//         use CommandRange::*;
//         match self {
//             CurrentLine => true.into(),
//             WholeFile => "%".into(),
//             Count(n) => n.into(),
//             _ => ().into(),
//         }
//     }
// }
