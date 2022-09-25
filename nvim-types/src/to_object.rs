use crate::{Function, Object};

pub trait ToObject {
    fn to_obj(self) -> Object;
}

impl<A, R> ToObject for Function<A, R> {
    fn to_obj(self) -> Object {
        self.into()
    }
}
