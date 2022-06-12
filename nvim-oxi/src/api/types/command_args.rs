use nvim_types::Object;
use serde::Deserialize;

use super::CommandModifiers;
use crate::object::{self, de::utils, FromObject};

/// Arguments passed to callbacks registered with
/// `crate::api::nvim_create_user_command`. See `:h nvim_create_user_command`
/// for details.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct CommandArgs {
    #[serde(deserialize_with = "utils::empty_string_is_none")]
    pub args: Option<String>,

    pub bang: bool,
    pub count: i32,
    pub fargs: Vec<String>,
    pub line1: usize,
    pub line2: usize,
    pub range: usize,

    #[serde(rename = "reg", deserialize_with = "utils::empty_string_is_none")]
    pub register: Option<String>,

    #[serde(deserialize_with = "utils::empty_string_is_none")]
    pub mods: Option<String>,

    pub smods: CommandModifiers,
}

impl FromObject for CommandArgs {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
