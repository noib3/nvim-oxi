use nvim_types::Object;
use serde::Deserialize;

use super::Mode;
use crate::object::{self, FromObject};
use crate::Result;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct GotMode {
    pub blocking: bool,
    pub mode: Mode,
}

impl FromObject for GotMode {
    fn from_obj(obj: Object) -> Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}
