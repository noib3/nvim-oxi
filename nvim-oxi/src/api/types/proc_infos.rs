use nvim_types::Object;
use serde::Deserialize;

use crate::object::{self, FromObject};

/// Object returned from a call to `crate::api::get_proc`.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct ProcInfos {
    pub name: Option<String>,
    pub pid: Option<u32>,
    pub ppid: Option<u32>,
}

impl FromObject for ProcInfos {
    fn from_obj(obj: Object) -> crate::Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
