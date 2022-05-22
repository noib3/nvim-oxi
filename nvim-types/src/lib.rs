mod array;
mod collection;
mod dictionary;
pub mod error;
mod handles;
mod object;
mod string;

pub use array::Array;
pub use dictionary::Dictionary;
pub use error::{Error, ErrorType};
pub use handles::BufHandle;
pub use object::Object;
pub use string::NvimString;

pub type Integer = i64;
