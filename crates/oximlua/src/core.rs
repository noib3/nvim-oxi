use std::cell::OnceCell;

use mlua::Lua;

thread_local! {
    static LUA: OnceCell<Lua> = const { OnceCell::new() };
    static LUA_REGISTRY: OnceCell<mlua::Table> = const { OnceCell::new() };
}

/// Sets a global
/// [`mlua::Lua`](https://docs.rs/mlua/latest/mlua/struct.Lua.html)
/// instance which can be then be retrieved with `get_lua()`.
/// This must be called exactly once at the
/// [mlua module entry point](https://github.com/mlua-rs/mlua#module-mode).
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi::lua::olua;
///
/// mod submodule;
///
/// #[mlua::lua_module]
/// fn plugin_entry_point(lua: &Lua) -> mlua::Result<mlua::Table> {
///     olua::set_lua(lua);
///
///     let lua = olua::get_lua();
///     let plugin_entry_point = lua.create_table()?;
///     plugin_entry_point.set("submodule", submodule::submodule_entry_point()?)?;
///
///     Ok(plugin_entry_point)
/// }
/// ```
pub fn set_lua(lua: &Lua) {
    LUA.try_with(|global_lua| {
        global_lua.set(lua.clone()).expect(
            "`LUA` should only be initialized once at mlua entry point",
        );
    })
    .expect("`LUA` should not be dropped");
    set_registry(lua);
}

fn set_registry(lua: &Lua) {
    LUA_REGISTRY
        .try_with(|registry| {
            registry
                .set(
                    lua.load("debug.getregistry()")
                        .eval()
                        .expect("`LUA_REGISTRY` should be available"),
                )
                .expect(
                    "`LUA_REGISTRY` should only be initialized once at mlua \
                     entry point",
                );
        })
        .expect("`LUA_REGISTRY` should not be dropped");
}

/// Returns a
/// [`mlua::Value`](https://docs.rs/mlua/latest/mlua/enum.Value.html)
/// from the NVIM Lua registry by it's registry id.
pub fn get_registry_value(registry_id: i32) -> mlua::Result<mlua::Value> {
    LUA_REGISTRY
        .try_with(|registry| {
            registry
                .get()
                .expect(
                    "`LUA_REGISTRY` should be initialized at mlua entry point",
                )
                .clone()
        })
        .expect("`LUA_REGISTRY` should not be dropped")
        .get(registry_id)
}

/// Returns a
/// [`mlua::Lua`](https://docs.rs/mlua/latest/mlua/struct.Lua.html)
/// instance which can be used to interact with Lua plugins.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi::lua::olua;
///
/// fn hello_oximlua() -> mlua::Result<()> {
///     nvim::print!("Hello from nvim-oxi..");
///
///     let lua = olua::get_lua();
///     let print = lua.globals().get::<_, mlua::Function>("print")?;
///     print.call("..and goodbye from mlua!")?;
///
///     Ok(())
/// }
/// ```
pub fn get_lua() -> Lua {
    LUA.try_with(|global_lua| {
        global_lua
            .get()
            .expect("`LUA` should be initialized at mlua entry point")
            .clone()
    })
    .expect("`LUA` should not be dropped")
}
