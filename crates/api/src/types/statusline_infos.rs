use serde::Deserialize;
use types::{
    Object,
    conversion::{self, FromObject},
    serde::Deserializer,
};

use super::StatuslineHighlightInfos;

/// Statusline informations returned by
/// [`eval_statusline`](crate::eval_statusline).
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct StatuslineInfos {
    /// Vector of highlight informations for the statusline populated if the
    /// [`highlights`](crate::opts::EvalStatuslineOptsBuilder::highlights)
    /// field of  was set to `true`.
    #[serde(default)]
    pub highlights: Vec<StatuslineHighlightInfos>,

    /// Characters displayed in the statusline.
    pub str: String,

    /// Display width of the statusline.
    pub width: u32,
}

impl FromObject for StatuslineInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
