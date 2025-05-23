use serde::de::Deserialize;
use types::Dictionary;
use types::conversion::Error;
use types::serde::Deserializer;

/// Infos returned by [`Window::text_height()`][crate::Window::text_height].
#[derive(Debug, serde::Deserialize)]
pub struct WinTextHeightInfos {
    /// The total number of screen lines occupied by the range.
    pub all: u32,

    /// The number of diff filler or virtual lines among them.
    pub fill: u32,
}

impl TryFrom<types::Dictionary> for WinTextHeightInfos {
    type Error = Error;

    #[inline]
    fn try_from(value: Dictionary) -> Result<Self, Self::Error> {
        Self::deserialize(Deserializer::new(value.into())).map_err(Into::into)
    }
}
