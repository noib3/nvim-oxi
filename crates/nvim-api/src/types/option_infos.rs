use nvim_types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Object,
};
use serde::Deserialize;

/// Informations related to an option. Unlike in the Lua API, the `type` field
/// is omitted because it's included in the definition of `default`.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OptionInfos {
    /// TODO: docs
    pub allows_duplicates: bool,

    /// List of comma-separated values.
    pub commalist: bool,

    /// The default value for the option.
    pub default: Object,

    /// TODO: docs
    pub flaglist: bool,

    /// Whether a window or buffer option also has a global value.
    pub global_local: bool,

    /// Channel id where the option was set (`0` for local).
    pub last_set_chan: i64,

    /// The line number where the option was set.
    pub last_set_linenr: usize,

    /// Last set script id (if any).
    pub last_set_sid: i32,

    /// Name of the option (like `"filetype"`).
    pub name: String,

    /// Scope of the option.
    pub scope: OptionScope,

    /// Shortened name of the  option (like `"ft"`).
    pub shortname: String,

    /// Whether the option was set.
    pub was_set: bool,
}

impl FromObject for OptionInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum OptionScope {
    #[serde(rename = "buf")]
    Buffer,

    #[serde(rename = "global")]
    Global,

    #[serde(rename = "win")]
    Window,
}

impl OptionScope {
    #[inline]
    pub const fn is_buffer(&self) -> bool {
        matches!(self, OptionScope::Buffer)
    }

    #[inline]
    pub const fn is_global(&self) -> bool {
        matches!(self, OptionScope::Global)
    }

    #[inline]
    pub const fn is_window(&self) -> bool {
        matches!(self, OptionScope::Window)
    }
}
