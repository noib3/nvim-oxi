//! # Rust bindings to all things Neovim
//!
//! This library provides safe bindings to the API exposed by the [Neovim] text
//! editor.
//!
//! [Neovim]: https://neovim.io

#![doc(html_root_url = "https://docs.rs/nvim_oxi/0.4.1")]
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
    pub use oxi_api::*;
}

#[cfg(feature = "libuv")]
#[cfg_attr(docsrs, doc(cfg(feature = "libuv")))]
pub mod libuv {
    //! Bindings to the [Neovim event loop][loop] powered by [libuv].
    //!
    //! [loop]: https://neovim.io/doc/user/lua.html#vim.loop
    //! [libuv]: https://libuv.org/
    #[doc(inline)]
    pub use oxi_libuv::*;
}

pub mod lua {
    //! Low-level Rust bindings to [LuaJIT], the Lua version used by Neovim.
    //!
    //! [LuaJIT]: https://luajit.org/
    #[doc(inline)]
    pub use oxi_luajit::*;
}

#[cfg(feature = "mlua")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlua")))]
pub mod mlua {
    //! Integrations with the [mlua] Rust crate providing safe Lua bindings.
    //!
    //! [mlua]: https://github.com/khvzak/mlua

    pub use mlua::*;

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
            oxi_luajit::with_state(|lua_state| {
                mlua::Lua::init_from_ptr(lua_state as *mut _).into_static()
            })
        }
    }
}

// #[cfg(feature = "diagnostic")]
// #[cfg_attr(docsrs, doc(cfg(feature = "diagnostic")))]
// pub mod diagnostic {
//     #[doc(inline)]
//     pub use nvim_diagnostic::*;
// }

#[doc(hidden)]
pub use entrypoint::entrypoint;
pub use error::{Error, Result};
pub use oxi_luajit::{dbg, print};
pub use oxi_macros::oxi_module as module;
#[cfg(feature = "test")]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub use oxi_macros::oxi_test as test;
pub use oxi_types::*;
#[cfg(feature = "test")]
#[doc(hidden)]
pub mod __test {
    use std::path::{Path, PathBuf};

    pub fn get_target_dir(manifest_dir: &Path) -> PathBuf {
        use miniserde::json;

        let output = ::std::process::Command::new(
            ::std::env::var("CARGO")
                .ok()
                .unwrap_or_else(|| "cargo".to_string()),
        )
        .arg("metadata")
        .arg("--format-version=1")
        .arg("--no-deps")
        .current_dir(manifest_dir)
        .output()
        .unwrap();

        let object: json::Object =
            json::from_str(&String::from_utf8(output.stdout).unwrap())
                .unwrap();

        let target_dir = match object.get("target_directory").unwrap() {
            json::Value::String(s) => s,
            _ => panic!("Must be string value"),
        };

        target_dir.into()
    }
}

pub use toplevel::*;
