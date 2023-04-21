use oxi_types::Object;
use serde::Deserialize;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowStyle {
    Minimal,
}

impl From<WindowStyle> for Object {
    fn from(style: WindowStyle) -> Self {
        use WindowStyle::*;
        Self::from(match style {
            Minimal => "minimal",
        })
    }
}
