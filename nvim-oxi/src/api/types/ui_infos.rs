use nvim_types::Object;
use serde::Deserialize;

use crate::object::{self, de::utils, FromObject};

/// Informations about an attached UI.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct UiInfos {
    /// Requested height of the UI.
    pub height: usize,

    /// Requested height of the UI.
    pub width: usize,

    /// `true` if the UI uses RGB colors.
    pub rgb: bool,

    pub r#override: bool,

    /// Channel id or remote UI (not present for TUI).
    #[serde(rename = "chan", deserialize_with = "utils::zero_is_none")]
    pub chan_id: Option<u32>,

    pub ext_cmdline: bool,
    pub ext_hlstate: bool,
    pub ext_linegrid: bool,
    pub ext_messages: bool,
    pub ext_multigrid: bool,
    pub ext_popupmenu: bool,
    pub ext_tabline: bool,
    pub ext_termcolors: bool,
    pub ext_wildmenu: bool,
}

impl FromObject for UiInfos {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
