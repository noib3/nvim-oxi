//! # Rust bindings to all things Neovim
//!
//! This library provides safe bindings to the API exposed by the [Neovim] text
//! editor.
//!
//! [Neovim]: https://neovim.io

#![doc(html_root_url = "https://docs.rs/nvim_oxi/0.1.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]

mod entrypoint;
mod error;
mod toplevel;

pub mod api {
    //! Bindings to the [Neovim C API][api].
    //!
    //! [api]: https://neovim.io/doc/user/api.html
    #[doc(inline)]
    pub use nvim_api::*;
}

#[cfg(feature = "libuv")]
#[cfg_attr(docsrs, doc(cfg(feature = "libuv")))]
pub mod libuv {
    //! Bindings to the [Neovim event loop][loop] powered by [libuv].
    //!
    //! [loop]: https://neovim.io/doc/user/lua.html#vim.loop
    //! [libuv]: https://libuv.org/
    #[doc(inline)]
    pub use libuv_bindings::*;
}

pub mod lua {
    //! Low-level Rust bindings to [LuaJIT], the Lua version used by Neovim.
    //!
    //! [LuaJIT]: https://luajit.org/
    #[doc(inline)]
    pub use luajit_bindings::*;
}

#[cfg(feature = "mlua")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlua")))]
pub mod mlua {
    //! Integrations with the [mlua] Rust crate providing safe Lua bindings.
    //!
    //! [mlua]: https://github.com/khvzak/mlua

    /// Returns a static reference to a
    /// [`mlua::Lua`](https://docs.rs/mlua/latest/mlua/struct.Lua.html) object
    /// which can be used to interact with Lua plugins.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use mlua::prelude::LuaFunction;
    /// use nvim_oxi as nvim;
    ///
    /// #[nvim::module]
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
    pub fn lua() -> &'static mlua::Lua {
        unsafe {
            luajit_bindings::with_state(|lua_state| {
                mlua::Lua::init_from_ptr(lua_state as *mut _).into_static()
            })
        }
    }
}

#[doc(hidden)]
pub use entrypoint::entrypoint;
pub use error::{Error, Result};
pub use luajit_bindings::{dbg, print};
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
