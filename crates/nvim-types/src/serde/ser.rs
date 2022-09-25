//! [`Serialize`](serde::Serialize) implementations for various types defined
//! in this crate.

use serde::ser::{Serialize, Serializer};

use crate::Function;

impl<A, R> Serialize for Function<A, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.lua_ref as f32)
    }
}
