use nvim_types as nvim;
use serde::Deserialize;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
/// Controls the positioning of the virtual text associated to an extmark.
#[serde(rename_all(deserialize = "snake_case"))]
pub enum ExtmarkVirtTextPosition {
    /// Right after the EOL character (default).
    Eol,

    /// Display over the specified column, without shifting the underlying
    /// text.
    Overlay,

    /// Display right aligned in the window.
    RightAlign,
}

impl From<ExtmarkVirtTextPosition> for nvim::String {
    fn from(pos: ExtmarkVirtTextPosition) -> Self {
        use ExtmarkVirtTextPosition::*;

        Self::from(match pos {
            Eol => "eol",
            Overlay => "overlay",
            RightAlign => "right_align",
        })
    }
}
