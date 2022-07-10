use nvim_types::Object;
use serde::Deserialize;

use super::{CommandAddr, CommandArgs, CommandNArgs, CommandRange};
use crate::lua::Function;
use crate::object::{self, FromObject};
use crate::Result;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct CommandInfos {
    /// TODO: docs
    pub addr: Option<CommandAddr>,

    /// Whether the command can take a `!` modifier.
    pub bang: bool,

    /// Whether the command can be followed by a `|` and another command.
    pub bar: bool,

    /// Callback triggered by the command.
    pub callback: Option<Function<CommandArgs, ()>>,

    /// Command complletion strategy.
    pub complete: Option<String>,

    /// TODO: docs
    pub complete_arg: Option<String>,

    /// TODO: docs and parse string to `u32`.
    pub count: Option<String>,

    /// TODO: docs
    pub definition: Option<String>,

    /// Whether to use the invocation location as opposed to the definition
    /// location in verbose messages.
    pub keepscript: bool,

    /// The command name.
    pub name: String,

    /// The number of arguments the command can take.
    #[serde(default)]
    pub nargs: CommandNArgs,

    /// TODO: docs
    pub range: Option<CommandRange>,

    /// Whether the firrst argument to the command can be an optional register
    /// name (like `:del`, `:put` or `:yank`).
    pub register: bool,

    /// TODO: docs
    pub script_id: i32,
}

impl FromObject for CommandInfos {
    fn from_obj(obj: Object) -> Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
