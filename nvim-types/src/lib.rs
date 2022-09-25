use std::ffi::{c_double, c_int};

mod array;
mod collection;
mod dictionary;
mod error;
mod from_object;
mod function;
mod non_owning;
mod object;
mod string;
mod to_object;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub(crate) mod serde;

pub use array::{Array, ArrayIterator};
pub(crate) use collection::*;
pub use dictionary::{DictIterator, Dictionary};
pub use error::Error;
pub use from_object::{
    Error as FromObjectError,
    FromObject,
    Result as FromObjectResult,
};
pub use function::Function;
#[doc(hidden)]
pub use non_owning::NonOwning;
pub use object::{Object, ObjectKind};
pub use string::String;
pub use to_object::{Error as ToObjectError, ToObject, ToObjectResult};

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use crate::serde::{Deserializer, Serializer};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L67
#[doc(hidden)]
pub type Boolean = bool;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L68
#[doc(hidden)]
pub type Integer = i64;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L69
#[doc(hidden)]
pub type Float = c_double;

// https://github.com/neovim/neovim/blob/master/src/nvim/types.h#L23
#[doc(hidden)]
pub type LuaRef = c_int;

// https://github.com/neovim/neovim/blob/master/src/nvim/types.h#L18
#[allow(non_camel_case_types)]
type handle_T = c_int;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L82
#[doc(hidden)]
pub type BufHandle = handle_T;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L83
#[doc(hidden)]
pub type WinHandle = handle_T;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L84
#[doc(hidden)]
pub type TabHandle = handle_T;
