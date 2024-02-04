use types::Integer;

use super::OneOrMore;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExtmarkVirtTextChunk {
    pub text: String,
    pub hl_groups: Vec<StringOrInt>,
}

impl<'de> serde::de::Deserialize<'de> for ExtmarkVirtTextChunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};

        struct ExtmarkVirtTextChunkVisitor;

        impl<'de> Visitor<'de> for ExtmarkVirtTextChunkVisitor {
            type Value = ExtmarkVirtTextChunk;

            fn expecting(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                f.write_str("a (text, hl_group) tuple")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let text = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let hl_groups = seq
                    .next_element::<OneOrMore<StringOrInt>>()?
                    .map(|groups| match groups {
                        OneOrMore::One(group) => vec![group],
                        OneOrMore::List(groups) => groups,
                    })
                    .unwrap_or_default();

                Ok(ExtmarkVirtTextChunk { text, hl_groups })
            }
        }

        deserializer.deserialize_seq(ExtmarkVirtTextChunkVisitor)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Deserialize)]
#[serde(untagged)]
pub enum StringOrInt {
    String(String),
    Int(Integer),
}

impl From<String> for StringOrInt {
    #[inline]
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for StringOrInt {
    #[inline]
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl From<Integer> for StringOrInt {
    #[inline]
    fn from(i: Integer) -> Self {
        Self::Int(i)
    }
}
