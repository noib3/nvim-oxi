mod de;
mod from_object;
mod to_object;

pub use de::Diocan;
pub use from_object::{FromObject, FromObjectError};
pub use nvim_types::object::{Object, ObjectData, ObjectType};
pub use to_object::{ToObject, ToObjectError};
