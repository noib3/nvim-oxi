use oxi_types::{Object, String as NvimString};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
/// Specifies which corner of a floating window to place at `(row, col)`.
pub enum WindowAnchor {
    #[serde(rename = "NW")]
    NorthWest,

    #[serde(rename = "NE")]
    NorthEast,

    #[serde(rename = "SW")]
    SouthWest,

    #[serde(rename = "SE")]
    SouthEast,
}

impl From<WindowAnchor> for NvimString {
    fn from(anchor: WindowAnchor) -> Self {
        match anchor {
            WindowAnchor::NorthWest => "NW",
            WindowAnchor::NorthEast => "NE",
            WindowAnchor::SouthWest => "SW",
            WindowAnchor::SouthEast => "SE",
        }
        .into()
    }
}

impl From<WindowAnchor> for Object {
    fn from(anchor: WindowAnchor) -> Self {
        NvimString::from(anchor).into()
    }
}
