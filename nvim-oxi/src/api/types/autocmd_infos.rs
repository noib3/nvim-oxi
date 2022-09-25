use nvim_types::{Deserializer, FromObject, FromObjectResult, Object};
use serde::Deserialize;

use crate::api::Buffer;

/// Informations related to an autocommand.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct AutocmdInfos {
    /// The `Buffer` associated to the autocommand. Only present if `buflocal`
    /// is `true`.
    pub buffer: Option<Buffer>,

    /// Whether the autocommand is a buffer-local one.
    pub buflocal: bool,

    /// The command executed by the autocommand.
    pub command: String,

    /// The autocommand's description.
    #[serde(default)]
    pub desc: Option<String>,

    /// The event triggering the autocommand.
    pub event: String,

    /// The autocommand group's id. Only present if the autocommand belongs to
    /// an autocommand group.
    #[serde(default)]
    pub group: Option<u32>,

    /// The autocommand group's name. Only present if the autocommand belongs
    /// to an autocommand group.
    #[serde(default)]
    pub group_name: Option<String>,

    /// The autocommand id.
    #[serde(default)]
    pub id: Option<u32>,

    /// Whether the autocommand is only run once.
    pub once: bool,

    /// The autocommand's pattern.
    pub pattern: String,
}

impl FromObject for AutocmdInfos {
    fn from_obj(obj: Object) -> FromObjectResult<Self> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
