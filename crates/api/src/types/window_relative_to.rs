use std::fmt;

use serde::de;
use types::{Object, String as NvimString};

use crate::Window;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
/// Specifies what a floating window is positioned relative to.
pub enum WindowRelativeTo {
    /// Positions the window relative to the global Neovim editor grid.
    Editor,

    /// Positions the window relative to another window.
    Window(Window),

    /// Positions the window relative to the current cursor position.
    Cursor,

    /// Positions the window relative to the current mouse cursor position..
    Mouse,
}

impl From<WindowRelativeTo> for NvimString {
    fn from(pos: WindowRelativeTo) -> Self {
        match pos {
            WindowRelativeTo::Editor => "editor",
            WindowRelativeTo::Window(_) => "win",
            WindowRelativeTo::Cursor => "cursor",
            WindowRelativeTo::Mouse => "mouse",
        }
        .into()
    }
}

impl From<&WindowRelativeTo> for Object {
    fn from(pos: &WindowRelativeTo) -> Self {
        NvimString::from(pos.clone()).into()
    }
}

// https://github.com/serde-rs/serde/issues/1402
impl<'de> de::Deserialize<'de> for WindowRelativeTo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct WindowRelativeToVisitor;

        impl<'de> de::Visitor<'de> for WindowRelativeToVisitor {
            type Value = WindowRelativeTo;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("\"editor\", \"cursor\" or \"mouse\"")
            }

            fn visit_i64<E>(self, n: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let handle = i32::try_from(n).unwrap();
                Ok(WindowRelativeTo::Window(handle.into()))
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match s {
                    "editor" => Ok(WindowRelativeTo::Editor),
                    "cursor" => Ok(WindowRelativeTo::Cursor),
                    "mouse" => Ok(WindowRelativeTo::Mouse),
                    _ => Err(E::invalid_value(
                        de::Unexpected::Str(s),
                        &"\"editor\", \"cursor\" or \"mouse\"",
                    )),
                }
            }
        }

        deserializer.deserialize_str(WindowRelativeToVisitor)
    }
}
