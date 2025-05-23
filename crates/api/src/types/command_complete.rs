use serde::Serialize;
use types::{
    Function,
    Object,
    conversion::{self, ToObject},
    serde::Serializer,
};

/// See `:h command-complete` for details.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
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

    /// See `:h command-completion-customlist` for details.
    CustomList(Function<(String, String, usize), Vec<String>>),
}

impl ToObject for CommandComplete {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}
