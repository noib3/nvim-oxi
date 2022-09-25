use nvim_types::{Deserializer, FromObject, FromObjectResult, Object};
use serde::Deserialize;

use super::Mode;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct GotMode {
    pub blocking: bool,
    pub mode: Mode,
}

impl FromObject for GotMode {
    fn from_obj(obj: Object) -> FromObjectResult<Self> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
