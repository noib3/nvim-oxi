use nvim_types::Object;
use serde::Deserialize;

use super::Mode;
use crate::lua::LuaFun;
use crate::object::{self, de::utils, FromObject};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct KeymapInfos {
    #[serde(deserialize_with = "utils::bool_from_int")]
    buffer: bool,

    callback: Option<LuaFun<(), ()>>,

    #[serde(deserialize_with = "utils::bool_from_int")]
    expr: bool,

    lhs: String,

    #[serde(deserialize_with = "utils::zero_is_none")]
    lnum: Option<u32>,

    mode: Mode,

    #[serde(deserialize_with = "utils::bool_from_int")]
    noremap: bool,

    #[serde(deserialize_with = "utils::bool_from_int")]
    nowait: bool,

    rhs: Option<String>,

    #[serde(deserialize_with = "utils::bool_from_int")]
    script: bool,

    sid: i32,

    #[serde(deserialize_with = "utils::bool_from_int")]
    silent: bool,
}

impl FromObject for KeymapInfos {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
