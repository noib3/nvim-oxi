use nvim_types as nvim;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Wheel,
}

impl From<MouseButton> for nvim::String {
    fn from(action: MouseButton) -> nvim::String {
        use MouseButton::*;
        nvim::String::from(match action {
            Left => "left",
            Middle => "middle",
            Right => "right",
            Wheel => "wheel",
        })
    }
}
