use nvim_types::Object;
use serde::{Deserialize, Serialize};

use crate::object::{self, ToObject};

/// Number of arguments accepted by a command. See `:h command-nargs` for
/// details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CommandNArgs {
    #[serde(rename = "0")]
    Zero,

    #[serde(rename = "1")]
    One,

    #[serde(rename = "*")]
    Any,

    #[serde(rename = "?")]
    ZeroOrOne,

    #[serde(rename = "+")]
    OneOrMore,
}

impl ToObject for CommandNArgs {
    fn to_obj(self) -> crate::Result<Object> {
        self.serialize(object::Serializer)
    }
}
