use nvim_types::Object;
use serde::{Deserialize, Serialize};

use crate::object::{self, ToObject};

/// See `:h command-addr` for details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandAddr {
    Lines,
    Arguments,
    Buffers,
    LoadedBuffers,
    Windows,
    Tabs,
    Quickfix,
    Other,
}

impl ToObject for CommandAddr {
    fn to_obj(self) -> crate::Result<Object> {
        self.serialize(object::Serializer)
    }
}
