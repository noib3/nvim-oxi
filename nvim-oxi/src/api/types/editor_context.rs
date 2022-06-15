use nvim_types::{Object, String as NvimString};
use serde::Deserialize;

use crate::object::{self, FromObject};

/// Object returned from a call to `crate::api::get_context`.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct EditorContext {
    #[serde(rename = "bufs")]
    bufferlist: Vec<NvimString>,

    #[serde(rename = "gvars")]
    global_vars: Vec<NvimString>,

    #[serde(rename = "funcs")]
    global_and_script_local_funcs: Vec<NvimString>,

    #[serde(rename = "jumps")]
    jumplist: Vec<NvimString>,

    #[serde(rename = "regs")]
    registers: Vec<NvimString>,

    #[serde(rename = "sfuncs")]
    script_local_funcs: Vec<NvimString>,
}

impl FromObject for EditorContext {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
