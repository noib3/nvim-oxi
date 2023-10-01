use oxi_types::{Object, String as NvimString};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum WindowTitlePosition {
    Left,
    Center,
    Right,
}

impl From<WindowTitlePosition> for NvimString {
    fn from(pos: WindowTitlePosition) -> Self {
        match pos {
            WindowTitlePosition::Left => "left",
            WindowTitlePosition::Center => "center",
            WindowTitlePosition::Right => "right",
        }
        .into()
    }
}

impl From<&WindowTitlePosition> for Object {
    fn from(pos: &WindowTitlePosition) -> Self {
        NvimString::from(pos.clone()).into()
    }
}
