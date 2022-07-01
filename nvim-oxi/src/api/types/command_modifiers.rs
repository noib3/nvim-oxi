use nvim_types::Object;
use serde::{Deserialize, Serialize};

use super::SplitModifier;
use crate::object::{self, de::utils, ToObject};

/// See `:h command-modifiers` for more infos.
#[non_exhaustive]
#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize, Serialize,
)]
pub struct CommandModifiers {
    pub browse: bool,
    pub confirm: bool,
    pub emsg_silent: bool,
    pub hide: bool,
    pub keepalt: bool,
    pub keepjumps: bool,
    pub keepmarks: bool,
    pub keeppatterns: bool,
    pub lockmarks: bool,
    pub noautocmd: bool,
    pub noswapfile: bool,
    pub sandbox: bool,
    pub silent: bool,
    #[serde(deserialize_with = "utils::empty_string_is_none")]
    pub split: Option<SplitModifier>,
    pub tab: i32,
    pub verbose: i32,
    pub vertical: bool,
}

impl ToObject for CommandModifiers {
    fn to_obj(self) -> crate::Result<Object> {
        self.serialize(object::Serializer)
    }
}
