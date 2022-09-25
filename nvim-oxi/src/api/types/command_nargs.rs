use nvim_types::{Serializer, ToObject, ToObjectResult};
use serde::{Deserialize, Serialize};

/// Number of arguments accepted by a command.
#[non_exhaustive]
#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
pub enum CommandNArgs {
    #[default]
    #[serde(rename = "0")]
    Zero,

    #[serde(rename = "1")]
    One,

    #[serde(rename = "?")]
    ZeroOrOne,

    #[serde(rename = "+")]
    OneOrMore,

    #[serde(rename = "*")]
    Any,
}

impl ToObject for CommandNArgs {
    fn to_obj(self) -> ToObjectResult {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}
