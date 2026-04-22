use std::cell::OnceCell;

use anyhow::Result;
use mlua::MaybeSend;
use mlua::prelude::*;

thread_local! {
    static LUA: OnceCell<Lua> = const { OnceCell::new() };
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
/// use anyhow::Result;
/// use mlua::prelude::*;
/// use nvim_oxi as nvim;
///
/// mod submodule;
///
/// #[mlua::lua_module]
/// fn plugin_entry_point(lua: &Lua) -> LuaResult<LuaTable> {
///     nvim::oximlua::set_lua(lua);
///
///     let lua = nvim::oximlua::get_lua();
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
}

/// Returns a
/// [`mlua::Lua`](https://docs.rs/mlua/latest/mlua/struct.Lua.html)
/// instance which can be used to interact with Lua plugins.
///
/// # Examples
///
/// ```ignore
/// use anyhow::Result;
/// use mlua::prelude::LuaFunction;
/// use nvim_oxi as nvim;
///
/// fn hello_oximlua() -> Result<()> {
///     nvim::print!("Hello from nvim-oxi..");
///
///     let lua = nvim::oximlua::get_lua();
///     let print = lua.globals().get::<_, LuaFunction>("print")?;
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

/// Wraps a function in a
/// [`mlua::Result`](https://docs.rs/mlua/latest/mlua/type.Result.html).
/// For convenient interoperability between `nvim-oxi`, `mlua`, `std`
/// and other crates the function's return type is expected to be
/// [`anyhow::Result`](https://docs.rs/anyhow/latest/anyhow/type.Result.html)
///
/// # Examples
///
/// ```ignore
/// use anyhow::Result;
/// use mlua::prelude::*;
/// use nvim_oxi as nvim;
///
/// fn to_int(str_num: String) -> Result<i32> {
///    Ok(args.parse::<i32>()?)
/// }
///
/// fn util_module() -> LuaResult<LuaTable> {
///     let lua = nvim::oximlua::get_lua();
///     let util_module = lua.create_table()?;
///     util_module.set("to_int", lua_wrap_fn(to_int)?)?;
///
///     Ok(util_module)
/// }
/// ```
pub fn lua_wrap_fn<F, A, R>(func: F) -> LuaResult<LuaFunction>
where
    F: Fn(A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
{
    get_lua().create_function(move |_lua, args| func(args).into_lua_err())
}
