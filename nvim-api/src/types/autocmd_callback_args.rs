use std::path::PathBuf;

use nvim_types::{Deserializer, FromObject, FromObjectResult, Object};
use serde::Deserialize;

use crate::Buffer;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AutocmdCallbackArgs {
    /// The `Buffer` specified by `<abuf>`.
    #[serde(rename = "buf")]
    pub buffer: Buffer,

    /// Arbitrary data passed to
    /// [`nvim_oxi::api::exec_autocmds`](crate::exec_autocmds).
    #[serde(default)]
    pub data: Object,

    /// The name of the event that triggered the autocommand.
    pub event: String,

    /// The expanded value of `<afile>`.
    pub file: PathBuf,

    /// The `id` of the autocommand group that the autocommand belongs to, if
    /// any.
    #[serde(default)]
    pub group: Option<u32>,

    /// The `id` of the autocommand.
    pub id: u32,

    /// The expanded value of `<amatch>`.
    pub r#match: String,
}

impl FromObject for AutocmdCallbackArgs {
    fn from_obj(obj: Object) -> FromObjectResult<Self> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl lua_bindings::LuaPoppable for AutocmdCallbackArgs {
    const N: std::ffi::c_int = 1;

    unsafe fn pop(
        lstate: *mut lua_bindings::ffi::lua_State,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Object::pop(lstate).and_then(|obj| Ok(Self::from_obj(obj)?))
    }
}
