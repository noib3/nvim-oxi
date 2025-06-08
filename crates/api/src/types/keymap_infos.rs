use serde::Deserialize;
use types::{
    Function,
    Object,
    conversion::{self, FromObject},
    serde::Deserializer,
};

use super::Mode;
use crate::{Buffer, serde_utils as utils};

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct KeymapInfos {
    /// When the [`KeymapInfos`] are returned from [`Buffer::get_keymap()`],
    /// this will contain the [`Buffer`] it was called on. `None` when returned
    /// from [`get_keymap()`](crate::get_keymap).
    #[serde(deserialize_with = "utils::zero_is_none")]
    pub buffer: Option<Buffer>,

    /// Optional callback triggered by the keymap.
    pub callback: Option<Function<(), ()>>,

    /// Whether the keymap argument is an expression.
    #[serde(deserialize_with = "utils::bool_from_int")]
    pub expr: bool,

    /// The left-hand side of the mapping.
    pub lhs: String,

    /// The number where a script-local mapping is defined, if known.
    #[serde(deserialize_with = "utils::zero_is_none")]
    pub lnum: Option<u32>,

    /// The modes for which the keymap is enabled.
    pub mode: Mode,

    /// Whether the right-hand side of the mapping is not remappable.
    #[serde(deserialize_with = "utils::bool_from_int")]
    pub noremap: bool,

    /// For buffer-local mappings, whether Neovim should wait for more
    /// characters to be typed if there's a global mapping that could also
    /// match. See `:h map-nowait` for more details.
    #[serde(deserialize_with = "utils::bool_from_int")]
    pub nowait: bool,

    /// The right-hand side of the mapping.
    #[serde(default, deserialize_with = "utils::empty_string_is_none")]
    pub rhs: Option<String>,

    /// Whether the mapping was defined with `<script>`.
    #[serde(deserialize_with = "utils::bool_from_int")]
    pub script: bool,

    /// The script-local ID, used for `<sid>` mappings.
    pub sid: i32,

    /// Whether the keymap is silent.
    #[serde(deserialize_with = "utils::bool_from_int")]
    pub silent: bool,
}

impl FromObject for KeymapInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
