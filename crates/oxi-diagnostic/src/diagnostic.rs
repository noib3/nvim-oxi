use std::error::Error;

use oxi_api::Buffer;
use oxi_luajit::{self as lua, ffi::*, macros::cstr, Pushable};

/// Binding to [`vim.diagnostic.enable()`][1].
///
/// [1]: https://neovim.io/doc/user/diagnostic.html#vim.diagnostic.enable()
pub fn enable(
    buffer: &Buffer,
    namespace: Option<u32>,
) -> Result<(), Box<dyn Error> /* TODO: actual error */> {
    let res = unsafe {
        lua::with_state(|lua_state| {
            lua_getglobal(lua_state, cstr!("vim"));
            lua_getfield(lua_state, -1, cstr!("diagnostic"));
            lua_getfield(lua_state, -1, cstr!("enable"));
            let _ = buffer.clone().push(lua_state);
            let _ = namespace.push(lua_state);
            lua_pcall(lua_state, 2, 0, 0)
        })
    };

    match res {
        LUA_OK => Ok(()),

        _err_code => {
            // TODO: error handling
            todo!()
        },
    }
}
