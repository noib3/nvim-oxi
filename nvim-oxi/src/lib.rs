//! # First-class bindings to Neovim internals
//!
//! The `nvim-oxi` crate ...

#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../../README.md")]

pub mod api;
mod error;
#[doc(hidden)]
pub mod lua;
mod macros;
pub mod object;
pub mod opts {
    pub use crate::api::opts::*;
}
mod toplevel;
pub mod types {
    pub use crate::api::types::*;
}

pub use error::{Error, Result};
pub use lua::LuaFun;
pub use nvim_types::{Object, String};
pub use oxi_module::oxi_module as module;
#[cfg(feature = "test")]
pub use oxi_test::oxi_test as test;
pub use toplevel::{nprint as print, *};
