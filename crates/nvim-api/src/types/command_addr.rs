use nvim_types::{
    conversion::{self, ToObject},
    serde::Serializer,
    Object,
};
use serde::{Deserialize, Serialize};

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
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}
