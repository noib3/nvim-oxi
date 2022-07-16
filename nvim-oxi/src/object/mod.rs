//! Handles the conversion between Rust types and Neovim
//! [`Object`s](crate::Object).
pub(crate) mod de;
mod from_object;
mod ser;
mod to_object;

pub use de::Deserializer;
pub use from_object::FromObject;
pub use ser::Serializer;
pub use to_object::ToObject;
