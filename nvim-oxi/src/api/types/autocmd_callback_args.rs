use std::path::PathBuf;

use nvim_types::Object;
use serde::Deserialize;

use crate::api::Buffer;
use crate::object::{self, FromObject};
use crate::Result;

pub type ShouldDeleteAutocmd = bool;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct AutocmdCallbackArgs {
    /// The `Buffer` specified by `<abuf>`.
    #[serde(rename = "buf")]
    pub buffer: Buffer,

    /// Arbitrary data passed to `api::exec_autocmds`.
    #[serde(default)]
    // TODO: what to put here?
    pub data: (),

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
    fn from_obj(obj: Object) -> Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
