//! # Rust bindings to all things Neovim
//!
//! The `nvim-oxi` crate provides first-class Rust bindings to the rich API
//! exposed by the [Neovim](https://neovim.io) terminal text editor.
//!
//! The project is mostly intended for plugin authors, although nothing's
//! stopping end users from writing their Neovim configs in Rust.

#![doc(html_root_url = "https://docs.rs/nvim_oxi/0.1.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod api;
mod error;
pub(crate) mod iterator;
#[doc(hidden)]
pub mod lua;
mod macros;
pub mod object;
pub use object::{FromObject, ToObject};
mod trait_utils;
pub mod opts {
    //! Contains all the `*Opts` structs passed to functions as optional
    //! arguments.
    pub use crate::api::opts::*;
}
mod toplevel;
pub mod types {
    //! Contains the Rust type definitions of objects given to and returned by
    //! Neovim functions.
    pub use crate::api::types::*;
}

pub use error::{Error, Result};
pub use lua::Function;
#[doc(hidden)]
pub use lua::{LuaPoppable, LuaPushable};
pub use nvim_types::{Array, Dictionary, Object, ObjectKind, String};
pub use oxi_module::oxi_module as module;
#[cfg(feature = "test")]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub use oxi_test::oxi_test as test;
pub use toplevel::*;
