use nvim_types::object::Object;

use crate::Result;

pub trait Diocan: Sized {
    fn from_obj(obj: Object) -> Result<Self>;
}

impl<'de, S: serde::Deserialize<'de>> Diocan for S {
    fn from_obj(obj: Object) -> Result<Self> {
        todo!()
    }
}
