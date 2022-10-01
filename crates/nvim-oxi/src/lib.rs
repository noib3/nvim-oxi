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

#[doc(hidden)]
pub mod entrypoint;
mod error;
mod toplevel;

pub mod api {
    #[doc(inline)]
    pub use nvim_api::*;
}

#[doc(hidden)]
pub mod lua {
    #[doc(inline)]
    pub use luajit_bindings::*;
}

#[cfg(feature = "libuv")]
#[cfg_attr(docsrs, doc(cfg(feature = "libuv")))]
pub mod libuv {
    #[doc(inline)]
    pub use libuv_bindings::*;
}

#[cfg(feature = "mlua")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlua")))]
pub mod mlua {
    /// Returns a static reference to a
    /// [`mlua::Lua`](https://docs.rs/mlua/latest/mlua/struct.Lua.html) object
    /// to be able to interact with other Lua plugins.
    #[doc(inline)]
    pub fn lua() -> &'static mlua::Lua {
        unsafe {
            crate::lua::with_state(|lstate| {
                mlua::Lua::init_from_ptr(lstate as *mut _).into_static()
            })
        }
    }
}

pub use error::{Error, Result};
pub use luajit_bindings::print;
#[doc(inline)]
pub use nvim_types::*;
#[doc(inline)]
pub use oxi_module::oxi_module as module;
#[cfg(feature = "test")]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
#[doc(inline)]
pub use oxi_test::oxi_test as test;
#[doc(inline)]
pub use toplevel::*;
