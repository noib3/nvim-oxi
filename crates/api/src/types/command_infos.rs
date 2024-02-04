use serde::{
    de::{self, Error},
    Deserialize,
};
use types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Function,
    Object,
};

use super::{CommandAddr, CommandArgs, CommandNArgs, CommandRange};

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

    /// TODO: docs
    #[serde(deserialize_with = "parse_count")]
    pub count: Option<u32>,

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

fn parse_count<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|count| {
            count.parse().map_err(|err: std::num::ParseIntError| {
                D::Error::custom(err.to_string())
            })
        })
        .transpose()
}

impl FromObject for CommandInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
