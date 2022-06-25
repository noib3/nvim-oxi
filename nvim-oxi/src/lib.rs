//! # First-class bindings to Neovim internals
//!
//! The `nvim-oxi` crate ...

#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]

pub mod api;
mod error;
#[doc(hidden)]
pub mod lua;
mod macros;
pub mod object;
mod toplevel;

pub use error::{Error, Result};
pub use lua::LuaFun;
pub use nvim_types::{Object, String};
pub use oxi_module::oxi_module as module;
pub use toplevel::{nprint as print, *};
