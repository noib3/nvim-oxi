//! This module contains the bindings to the [Neovim
//! API](https://neovim.io/doc/user/api.html), exposed in Lua through the
//! `vim.api` table.
//!
//! # Naming convention
//!
//! All the functions have been renamed by dropping the leading `nvim_` prefix,
//! e.g. `nvim_get_current_buf` simply becomes [`get_current_buf`].
//!
//! Also, the functions starting with `nvim_buf_*`, `nvim_win_*` and
//! `nvim_tabpage_*` are implemented as methods on the [`Buffer`], [`Window`]
//! and [`TabPage`] objects respectively.

mod autocmd;
mod buffer;
mod error;
mod extmark;
mod ffi;
mod global;
pub(crate) mod iterator;
pub mod opts;
mod tabpage;
mod trait_utils;
pub mod types;
mod vimscript;
mod win_config;
mod window;

pub use autocmd::*;
pub use buffer::*;
pub use error::{Error, Result};
pub use extmark::*;
pub use global::*;
pub use tabpage::*;
pub use vimscript::*;
pub use win_config::*;
pub use window::*;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L41
const INTERNAL_CALL_MASK: u64 = 1u64 << (std::mem::size_of::<u64>() * 8 - 1);

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L44
const VIML_INTERNAL_CALL: u64 = INTERNAL_CALL_MASK;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L47
const LUA_INTERNAL_CALL: u64 = VIML_INTERNAL_CALL + 1;
