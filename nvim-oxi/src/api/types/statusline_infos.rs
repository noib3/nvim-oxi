use nvim_types::Object;
use serde::Deserialize;

use super::StatuslineHighlightInfos;
use crate::object::{self, FromObject};

/// Object returned from a call to `crate::api::eval_statusline`.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct StatuslineInfos {
    /// Vector of highlight informations for the statusline. Only included if
    /// the `highlights` field of `crate::api::opts::EvalStatuslineOpts` was
    /// set to `true`.
    pub highlights: Option<Vec<StatuslineHighlightInfos>>,

    /// Characters that will be displayed in the statusline.
    pub str: String,

    /// Display width of the statusline.
    pub width: usize,
}

impl FromObject for StatuslineInfos {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
