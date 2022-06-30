use nvim_types::Object;
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

impl From<WindowAnchor> for Object {
    fn from(anchor: WindowAnchor) -> Self {
        use WindowAnchor::*;
        Self::from(match anchor {
            NorthWest => "NW",
            NorthEast => "NE",
            SouthWest => "SW",
            SouthEast => "SE",
        })
    }
}
