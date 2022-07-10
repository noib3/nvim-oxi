use nvim_types::Object;
use serde::Deserialize;

use super::StatuslineHighlightInfos;
use crate::object::{self, FromObject};

/// Statusline informations returned by
/// [`api::eval_statusline`](crate::api::eval_statusline).
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct StatuslineInfos {
    /// Vector of highlight informations for the statusline populated if the
    /// [`highlights`](crate::api::opts::EvalStatuslineOptsBuilder::highlights)
    /// field of  was set to `true`.
    #[serde(default)]
    pub highlights: Vec<StatuslineHighlightInfos>,

    /// Characters displayed in the statusline.
    pub str: String,

    /// Display width of the statusline.
    pub width: u32,
}

impl FromObject for StatuslineInfos {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
