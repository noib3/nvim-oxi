use serde::{Deserialize, Serialize, ser};
use types::{
    Object,
    conversion::{self, ToObject},
    serde::Serializer,
};

/// Number of arguments accepted by a command.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize)]
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

// https://github.com/serde-rs/serde/issues/1773
impl Serialize for CommandNArgs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            CommandNArgs::Zero => serializer.serialize_i32(0),
            CommandNArgs::One => serializer.serialize_i32(1),
            CommandNArgs::ZeroOrOne => serializer.serialize_str("?"),
            CommandNArgs::OneOrMore => serializer.serialize_str("+"),
            CommandNArgs::Any => serializer.serialize_str("*"),
        }
    }
}

impl ToObject for CommandNArgs {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}
