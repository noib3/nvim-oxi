use nvim_types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Object,
};
use serde::Deserialize;

use crate::serde_utils as utils;

/// Arguments passed to functions executed by commands. See
/// [`Buffer::create_user_command`](crate::api::Buffer::create_user_command) to
/// create a buffer-local command or
/// [`create_user_command`](crate::api::create_user_command) to create a global
/// one.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct CommandArgs {
    /// The arguments passed to the command, if any.
    #[serde(deserialize_with = "utils::empty_string_is_none")]
    pub args: Option<String>,

    /// Whether the command was executed with a `!` modifier.
    pub bang: bool,

    /// The count supplied by `<count>`, if any.
    #[serde(deserialize_with = "utils::minus_one_is_none")]
    pub count: Option<u32>,

    /// The arguments passed to the command split by unescaped whitespace.
    pub fargs: Vec<String>,

    /// The starting line of the command range.
    pub line1: usize,

    /// The final line of the command range.
    pub line2: usize,

    /// Command modifiers, if any.
    #[serde(deserialize_with = "utils::empty_string_is_none")]
    pub mods: Option<String>,

    /// The number of items in the command range.
    pub range: u8,

    /// The optional register, if specified.
    #[serde(rename = "reg", deserialize_with = "utils::empty_string_is_none")]
    pub register: Option<String>,

    /// Command modifiers in a more structured format.
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-8", feature = "neovim-nightly")))
    )]
    pub smods: super::CommandModifiers,
}

impl FromObject for CommandArgs {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl luajit_bindings::Poppable for CommandArgs {
    unsafe fn pop(
        lstate: *mut luajit_bindings::ffi::lua_State,
    ) -> Result<Self, luajit_bindings::Error> {
        let obj = Object::pop(lstate)?;

        Self::from_object(obj)
            .map_err(luajit_bindings::Error::pop_error_from_err::<Self, _>)
    }
}
