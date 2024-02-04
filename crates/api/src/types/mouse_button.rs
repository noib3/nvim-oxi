use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Wheel,
}

impl From<MouseButton> for types::String {
    fn from(action: MouseButton) -> Self {
        use MouseButton::*;
        Self::from(match action {
            Left => "left",
            Middle => "middle",
            Right => "right",
            Wheel => "wheel",
        })
    }
}
