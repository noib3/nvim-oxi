use serde::Deserialize;
use types::{Object, String as NvimString};

use crate::Window;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Specifies what a floating window is positioned relative to.
pub enum WindowRelativeTo {
    /// Positions the window relative to the global Neovim editor grid.
    Editor,

    /// Positions the window relative to the status line if present, or last row.
    Laststatus,

    /// Positions the window relative to the current cursor position.
    Cursor,

    /// Positions the window relative to the current mouse cursor position..
    Mouse,

    /// Positions the window relative to the tabline if present, or first row.
    Tabline,

    /// Positions the window relative to another window.
    #[serde(untagged)]
    Window(Window),
}

impl From<WindowRelativeTo> for NvimString {
    fn from(pos: WindowRelativeTo) -> Self {
        match pos {
            WindowRelativeTo::Editor => "editor",
            WindowRelativeTo::Window(_) => "win",
            WindowRelativeTo::Laststatus => "laststatus",
            WindowRelativeTo::Cursor => "cursor",
            WindowRelativeTo::Mouse => "mouse",
            WindowRelativeTo::Tabline => "tabline",
        }
        .into()
    }
}

impl From<&WindowRelativeTo> for Object {
    fn from(pos: &WindowRelativeTo) -> Self {
        NvimString::from(pos.clone()).into()
    }
}
