use nvim_types::String as NvimString;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Controls how the highlights of a virtual text associated to an extmark are
/// combined with the highlights of the text.
pub enum ExtmarkHlMode {
    /// Only show the virtual text's highlight (default).
    Replace,

    /// Combine with background text's highlight.
    Combine,

    /// Blend with background text's highlight.
    Blend,
}

impl From<ExtmarkHlMode> for NvimString {
    fn from(mode: ExtmarkHlMode) -> Self {
        use ExtmarkHlMode::*;

        Self::from(match mode {
            Replace => "replace",
            Combine => "combine",
            Blend => "blend",
        })
    }
}
