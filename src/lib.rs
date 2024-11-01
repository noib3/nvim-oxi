//! # Rust bindings to all things Neovim
//!
//! This library provides safe bindings to the API exposed by the [Neovim] text
//! editor.
//!
//! [Neovim]: https://neovim.io

#![doc(html_root_url = "https://docs.rs/nvim_oxi/latest")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]

#[doc(hidden)]
pub mod entrypoint;
mod error;
mod toplevel;

pub mod api {
    //! Bindings to the [Neovim C API][api].
    //!
    //! [api]: https://neovim.io/doc/user/api.html
    #[doc(inline)]
    pub use api::*;
}

#[cfg(feature = "libuv")]
#[cfg_attr(docsrs, doc(cfg(feature = "libuv")))]
pub mod libuv {
    //! Bindings to the [Neovim event loop][loop] powered by [libuv].
    //!
    //! [loop]: https://neovim.io/doc/user/lua.html#vim.loop
    //! [libuv]: https://libuv.org/
    #[doc(inline)]
    pub use libuv::*;
}

pub mod lua {
    //! Low-level Rust bindings to [LuaJIT], the Lua version used by Neovim.
    //!
    //! [LuaJIT]: https://luajit.org/
    #[doc(inline)]
    pub use luajit::*;
}

#[cfg(feature = "mlua")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlua")))]
pub mod mlua {
    //! Integrations with the [mlua] Rust crate providing safe Lua bindings.
    //!
    //! [mlua]: https://github.com/khvzak/mlua

    pub use mlua::*;

    /// Returns a
    /// [`mlua::Lua`](https://docs.rs/mlua/latest/mlua/struct.Lua.html)
    /// instance which can be used to interact with Lua plugins.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use mlua::prelude::LuaFunction;
    /// use nvim_oxi as nvim;
    ///
    /// #[nvim::plugin]
    /// fn mlua() -> nvim::Result<()> {
    ///     nvim::print!("Hello from nvim-oxi..");
    ///
    ///     let lua = nvim::mlua::lua();
    ///     let print = lua.globals().get::<_, LuaFunction>("print")?;
    ///     print.call("..and goodbye from mlua!")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn lua() -> mlua::Lua {
        unsafe {
            luajit::with_state(|lua_state| {
                mlua::Lua::init_from_ptr(lua_state as *mut _)
            })
        }
    }
}

pub use error::{Error, Result};
pub use luajit::{dbg, print, IntoResult};
pub use macros::plugin;
#[cfg(feature = "test")]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub use macros::test;
pub use types::*;
#[cfg(feature = "test")]
#[doc(hidden)]
pub mod tests;
#[cfg(feature = "test-terminator")]
#[cfg_attr(docsrs, doc(cfg(feature = "test-terminator")))]
pub use tests::{TestFailure, TestTerminator};
pub use toplevel::*;
pub use types::string;
