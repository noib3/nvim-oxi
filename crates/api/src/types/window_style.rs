use serde::Deserialize;
use types::{Object, String as NvimString};

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowStyle {
    Minimal,
}

impl From<WindowStyle> for NvimString {
    fn from(style: WindowStyle) -> Self {
        match style {
            WindowStyle::Minimal => "minimal",
        }
        .into()
    }
}

impl From<WindowStyle> for Object {
    fn from(style: WindowStyle) -> Self {
        NvimString::from(style).into()
    }
}
