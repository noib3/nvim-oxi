use crate::Object;

pub trait FromObject: Sized {
    fn from_obj(obj: Object) -> Self;
}
