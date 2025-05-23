use serde::{Deserialize, Serialize};
use types::{
    Object,
    conversion::{self, ToObject},
    serde::Serializer,
};

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

impl CommandAddr {
    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::Lines => "lines",
            Self::Arguments => "arguments",
            Self::Buffers => "buffers",
            Self::LoadedBuffers => "loaded_buffers",
            Self::Windows => "windows",
            Self::Tabs => "tabs",
            Self::Quickfix => "quickfix",
            Self::Other => "other",
        }
    }
}

impl ToObject for CommandAddr {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}
