use std::fmt;

use serde::{de, ser, Serialize};

/// See `:h command-range` for details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum CommandRange {
    #[serde(rename(deserialize = "."), serialize_with = "serialize_as_true")]
    CurrentLine,

    #[serde(rename = "%")]
    WholeFile,

    #[serde(deserialize_with = "parse_string")]
    Count(u32),
}

impl<'de> de::Deserialize<'de> for CommandRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct CommandRangeVisitor;

        impl<'de> de::Visitor<'de> for CommandRangeVisitor {
            type Value = CommandRange;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("\".\", \"%\" or an integer")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v.parse::<u32>() {
                    Ok(n) => Ok(Self::Value::Count(n)),

                    _ => match v {
                        "." => Ok(Self::Value::CurrentLine),
                        "%" => Ok(Self::Value::WholeFile),
                        other => Err(E::invalid_value(
                            de::Unexpected::Str(other),
                            &"\".\", \"%\" or an integer",
                        )),
                    },
                }
            }
        }

        deserializer.deserialize_str(CommandRangeVisitor)
    }
}

fn serialize_as_true<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    serializer.serialize_bool(true)
}
