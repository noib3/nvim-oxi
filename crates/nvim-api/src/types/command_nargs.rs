use nvim_types::{
    conversion::{self, ToObject},
    serde::Serializer,
    Object,
};
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
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}
