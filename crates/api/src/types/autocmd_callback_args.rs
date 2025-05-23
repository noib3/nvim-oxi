use std::path::PathBuf;

use serde::Deserialize;
use types::{
    Object,
    conversion::{self, FromObject},
    serde::Deserializer,
};

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
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl luajit::Poppable for AutocmdCallbackArgs {
    unsafe fn pop(
        lstate: *mut luajit::ffi::State,
    ) -> Result<Self, luajit::Error> {
        let obj = Object::pop(lstate)?;

        Self::from_object(obj)
            .map_err(luajit::Error::pop_error_from_err::<Self, _>)
    }
}
