use nvim_types::object::Object;
use serde::{Deserialize, Serialize};

use crate::object::ToObject;

/// See `:h command-addr` for details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
    fn to_obj(self) -> Object {
        use CommandAddr::*;
        match self {
            Lines => "lines",
            Arguments => "arguments",
            Buffers => "buffers",
            LoadedBuffers => "loaded_buffers",
            Windows => "windows",
            Tabs => "tabs",
            Quickfix => "quickfix",
            Other => "other",
        }
        .to_obj()
    }
}
