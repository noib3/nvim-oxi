use oxi_types as nvim;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MouseAction {
    Down,
    Drag,
    Left,
    Press,
    Release,
    Right,
    Up,
}

impl From<MouseAction> for nvim::String {
    fn from(action: MouseAction) -> nvim::String {
        use MouseAction::*;
        nvim::String::from(match action {
            Down => "down",
            Drag => "drag",
            Left => "left",
            Press => "press",
            Release => "release",
            Right => "right",
            Up => "up",
        })
    }
}
