use nvim_types::{Deserializer, FromObject, FromObjectResult, Object};
use serde::Deserialize;

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
    fn from_obj(obj: Object) -> FromObjectResult<Self> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
