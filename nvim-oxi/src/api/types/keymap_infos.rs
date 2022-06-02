use nvim_types::LuaRef;
use serde::{de, Deserialize};

use super::Mode;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct KeymapInfos {
    #[serde(deserialize_with = "bool_from_int")]
    buffer: bool,

    callback: Option<LuaRef>,

    #[serde(deserialize_with = "bool_from_int")]
    expr: bool,

    lhs: String,

    #[serde(deserialize_with = "zero_is_none")]
    lnum: Option<u32>,

    mode: Mode,

    #[serde(deserialize_with = "bool_from_int")]
    noremap: bool,

    #[serde(deserialize_with = "bool_from_int")]
    nowait: bool,

    rhs: Option<String>,

    #[serde(deserialize_with = "bool_from_int")]
    script: bool,

    sid: i32,

    #[serde(deserialize_with = "bool_from_int")]
    silent: bool,
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),

        other => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

fn zero_is_none<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: de::Deserializer<'de>,
{
    u32::deserialize(deserializer).map(|lnum| (lnum != 0).then(|| lnum))
}

// fn empty_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
// where
//     D: de::Deserializer<'de>,
// {
//     String::deserialize(deserializer).map(|rhs| (!rhs.is_empty()).then(|| rhs))
// }
