//! (De)Serialization support for Neovim [`Object`](crate::Object)s using
//! [Serde].
//!
//! [Serde]: https://serde.rs/

mod de;
mod error;
mod ser;

pub use de::Deserializer;
pub use error::{DeserializeError, SerializeError};
pub use ser::Serializer;
