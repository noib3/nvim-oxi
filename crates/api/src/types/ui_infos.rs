use serde::Deserialize;
use types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Object,
};

use crate::serde_utils as utils;

/// Informations about an attached UI.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct UiInfos {
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

    /// Requested height of the UI.
    pub height: usize,

    pub r#override: bool,

    /// `true` if the UI uses RGB colors.
    pub rgb: bool,

    /// Requested height of the UI.
    pub width: usize,
}

impl FromObject for UiInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
