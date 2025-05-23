use serde::Deserialize;
use types::{
    Object,
    conversion::{self, FromObject},
    serde::Deserializer,
};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct ProcInfos {
    pub name: Option<String>,
    pub pid: Option<u32>,
    pub ppid: Option<u32>,
}

impl FromObject for ProcInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
