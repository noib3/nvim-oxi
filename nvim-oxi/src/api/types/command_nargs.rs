use nvim_types::object::Object;
use serde::{Deserialize, Serialize};

use crate::object::ToObject;

/// Number of arguments accepted by a command. See `:h command-nargs` for
/// details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

impl ToObject for CommandNArgs {
    fn to_obj(self) -> Object {
        use CommandNArgs::*;
        match self {
            Zero => 0.to_obj(),
            One => 1.to_obj(),
            Any => "*".to_obj(),
            ZeroOrOne => "?".to_obj(),
            OneOrMore => "+".to_obj(),
        }
    }
}
