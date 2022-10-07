#![allow(clippy::missing_safety_doc)]
use std::ffi::{c_double, c_int};

mod array;
mod collection;
pub mod conversion;
mod dictionary;
mod error;
mod function;
mod non_owning;
mod object;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;
mod string;

pub use array::{Array, ArrayIterator};
pub use collection::Collection;
pub use dictionary::{DictIterator, Dictionary};
pub use error::Error;
pub use function::Function;
#[doc(hidden)]
pub use non_owning::NonOwning;
pub use object::{Object, ObjectKind};
pub use string::String;

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
