use nvim_types::object::Object;
use serde::ser;

use crate::Result;

/// A struct for serializing Rust values into Neovim `Object`s.
#[derive(Debug)]
pub(super) struct Serializer;

// impl ser::Serializer for Serializer {
//     type Error = crate::Error;
//     type Ok = Object;
//     type SerializeMap = ();
//     type SerializeSeq = ();
//     type SerializeStruct = ();
//     type SerializeStructVariant = ();
//     type SerializeTuple = ();
//     type SerializeTupleStruct = ();
//     type SerializeTupleVariant = ();

//     #[inline]
//     fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
//         Ok(value.into())
//     }
// }
