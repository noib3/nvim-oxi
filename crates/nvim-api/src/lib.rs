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

#![cfg_attr(docsrs, feature(doc_cfg))]

mod autocmd;
mod buffer;
mod error;
mod extmark;
mod ffi;
mod global;
pub mod opts;
pub(crate) mod serde_utils;
mod tabpage;
mod trait_utils;
pub mod types;
pub(crate) mod utils;
mod vimscript;
mod win_config;
mod window;

pub use autocmd::*;
pub use buffer::*;
pub use error::Error;
use error::Result;
pub use extmark::*;
pub use global::*;
pub use tabpage::*;
pub use trait_utils::*;
pub use vimscript::*;
pub use win_config::*;
pub use window::*;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L43
const INTERNAL_CALL_MASK: u64 = 1u64 << (std::mem::size_of::<u64>() * 8 - 1);

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L46
const VIML_INTERNAL_CALL: u64 = INTERNAL_CALL_MASK;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L49
const LUA_INTERNAL_CALL: u64 = VIML_INTERNAL_CALL + 1;

macro_rules! choose {
    ($err:expr, ()) => {
        if $err.is_err() {
            Err($err.into())
        } else {
            Ok(())
        }
    };

    ($err:expr, $other:expr) => {
        if $err.is_err() {
            Err($err.into())
        } else {
            $other
        }
    };
}

pub(crate) use choose;
