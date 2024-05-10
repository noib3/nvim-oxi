use serde::{Deserialize, Serialize};
use types::String as NvimString;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SplitDirection {
    Above,
    Below,
    Left,
    Right,
}

impl From<SplitDirection> for NvimString {
    fn from(direction: SplitDirection) -> Self {
        match direction {
            SplitDirection::Above => "above",
            SplitDirection::Below => "below",
            SplitDirection::Left => "left",
            SplitDirection::Right => "right",
        }
        .into()
    }
}
