use nvim_types::Object;
use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum WindowTitlePosition {
    Left,
    Center,
    Right,
}

impl From<&WindowTitlePosition> for Object {
    fn from(pos: &WindowTitlePosition) -> Self {
        Self::from(match pos {
            WindowTitlePosition::Left => "left",
            WindowTitlePosition::Center => "center",
            WindowTitlePosition::Right => "right",
        })
    }
}
