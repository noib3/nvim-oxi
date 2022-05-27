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
pub use handles::*;
pub use object::{Object, ObjectType};
pub use string::NvimString;

// https://github.com/neovim/neovim/blob/master/src/nvim/types.h#L23
pub type LuaRef = libc::c_int;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L67
pub type Boolean = bool;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L68
pub type Integer = i64;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L69
pub type Float = libc::c_double;
