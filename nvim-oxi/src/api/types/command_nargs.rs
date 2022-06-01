use serde::{Deserialize, Serialize};

/// Number of arguments accepted by a command. See `:h command-nargs` for
/// details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CommandNArgs {
    Zero,      // 0
    One,       // 1
    Any,       // "*"
    ZeroOrOne, // "?"
    OneOrMore, // "+"
}
