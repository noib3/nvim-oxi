use nvim_types::Object;
use serde::Deserialize;

use crate::object::{self, FromObject};

/// Attributes related to a highlight group.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct HighlightInfos {
    pub background: Option<u32>,
    pub bg_indexed: Option<bool>,
    pub blend: Option<u32>,
    pub bold: Option<bool>,
    pub fg_indexed: Option<bool>,
    pub foreground: Option<u32>,
    pub italic: Option<bool>,
    pub reverse: Option<bool>,
    pub special: Option<u32>,
    pub standout: Option<bool>,
    pub strikethrough: Option<bool>,
    pub undercurl: Option<bool>,
    pub underdash: Option<bool>,
    pub underdot: Option<bool>,
    pub underline: Option<bool>,
    pub underlineline: Option<bool>,
}

impl FromObject for HighlightInfos {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
