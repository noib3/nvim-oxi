//! This module contains the bindings to the [Neovim API](https://neovim.io/doc/user/api.html), exposed in Lua through the `vim.api` table.
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
mod extmark;
mod ffi;
mod global;
pub(crate) mod opts;
mod tabpage;
pub(crate) mod types;
mod vimscript;
mod win_config;
mod window;

pub use autocmd::*;
pub use buffer::*;
pub use extmark::*;
pub use global::*;
pub use tabpage::*;
pub use vimscript::*;
pub use win_config::*;
pub use window::*;
