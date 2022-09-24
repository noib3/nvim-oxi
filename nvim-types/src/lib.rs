use std::ffi::{c_double, c_int};

mod array;
mod collection;
mod dictionary;
mod error;
mod non_owning;
mod object;
mod string;

pub use array::{Array, ArrayIterator};
pub(crate) use collection::*;
pub use dictionary::{DictIterator, Dictionary};
pub use error::Error;
pub use non_owning::NonOwning;
pub use object::{FromObjectError, Object, ObjectKind};
pub use string::String;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L67
pub type Boolean = bool;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L68
pub type Integer = i64;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L69
pub type Float = c_double;

// https://github.com/neovim/neovim/blob/master/src/nvim/types.h#L23
pub type LuaRef = c_int;

// https://github.com/neovim/neovim/blob/master/src/nvim/types.h#L18
#[allow(non_camel_case_types)]
type handle_T = c_int;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L82
pub type BufHandle = handle_T;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L83
pub type WinHandle = handle_T;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L84
pub type TabHandle = handle_T;
