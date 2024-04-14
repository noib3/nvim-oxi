use serde::Deserialize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowTitlePosition {
    Left,
    Center,
    Right,
}

impl From<WindowTitlePosition> for types::String {
    #[inline(always)]
    fn from(pos: WindowTitlePosition) -> Self {
        let pos = match pos {
            WindowTitlePosition::Left => "left",
            WindowTitlePosition::Center => "center",
            WindowTitlePosition::Right => "right",
        };

        pos.into()
    }
}

impl From<&WindowTitlePosition> for types::Object {
    #[inline(always)]
    fn from(pos: &WindowTitlePosition) -> Self {
        types::String::from(*pos).into()
    }
}
