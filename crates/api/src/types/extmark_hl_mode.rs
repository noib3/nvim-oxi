use serde::Deserialize;

/// Controls how the highlights of a virtual text associated to an extmark are
/// combined with the highlights of the text.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
#[non_exhaustive]
pub enum ExtmarkHlMode {
    /// Only show the virtual text's highlight (default).
    Replace,

    /// Combine with background text's highlight.
    Combine,

    /// Blend with background text's highlight.
    Blend,
}

impl From<ExtmarkHlMode> for types::String {
    #[inline(always)]
    fn from(mode: ExtmarkHlMode) -> Self {
        use ExtmarkHlMode::*;

        Self::from(match mode {
            Replace => "replace",
            Combine => "combine",
            Blend => "blend",
        })
    }
}
