use serde::Deserialize;
use types::{
    Object,
    conversion::{self, FromObject},
    serde::Deserializer,
};

/// Attributes related to a highlight group.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Default)]
pub struct HighlightInfos {
    pub altfont: Option<bool>,
    #[serde(rename = "bg")]
    pub background: Option<u32>,
    pub bg_indexed: Option<bool>,
    pub blend: Option<u32>,
    pub bold: Option<bool>,
    pub default: Option<bool>,
    pub fallback: Option<bool>,
    pub fg_indexed: Option<bool>,
    pub force: Option<bool>,
    #[serde(rename = "fg")]
    pub foreground: Option<u32>,
    pub italic: Option<bool>,
    pub reverse: Option<bool>,
    #[serde(rename = "sp")]
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
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
