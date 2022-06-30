use serde::Deserialize;

use super::SplitModifier;
use crate::object::de::utils;

/// See `:h command-modifiers` for more infos.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct CommandModifiers {
    pub browse: bool,
    pub confirm: bool,
    pub emsg_silent: bool,
    pub hide: bool,
    pub keepalt: bool,
    pub keepjumps: bool,
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
