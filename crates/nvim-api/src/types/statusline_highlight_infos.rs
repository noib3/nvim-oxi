use serde::Deserialize;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct StatuslineHighlightInfos {
    /// Name of the highlight group.
    pub group: String,

    /// Byte index (0-based) of the first character that uses the highlight.
    pub start: usize,
}
