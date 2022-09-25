use nvim_types::{Deserializer, FromObject, FromObjectResult, Object};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct ProcInfos {
    pub name: Option<String>,
    pub pid: Option<u32>,
    pub ppid: Option<u32>,
}

impl FromObject for ProcInfos {
    fn from_obj(obj: Object) -> FromObjectResult<Self> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
