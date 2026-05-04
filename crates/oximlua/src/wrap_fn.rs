use std::cell::RefCell;

use mlua::{ExternalResult, FromLuaMulti, IntoLuaMulti};

use crate::{IntoResult, lua as olua};

/// Wraps a Rust `<Fn>` function in a
/// [`mlua::Function`](https://docs.rs/mlua/latest/mlua/struct.Function.html).
///
/// # Examples
///
/// ```ignore
/// use mlua::prelude::*;
/// use nvim_oxi::lua::olua;
///
/// fn to_int(str_num: String) -> anyhow::Result<i32> {
///    Ok(str_num.parse::<i32>()?)
/// }
///
/// fn util_module() -> mlua::Result<mlua::Table> {
///     let lua = olua::get_lua();
///     let util_module = lua.create_table()?;
///     util_module.set("to_int", olua::wrap_fn(to_int)?)?;
///
///     Ok(util_module)
/// }
/// ```
pub fn wrap_fn<A, F, O, R>(func: F) -> mlua::Result<mlua::Function>
where
    F: Fn(A) -> O + mlua::MaybeSend + 'static,
    A: FromLuaMulti,
    O: IntoResult<R>,
    R: IntoLuaMulti,
    // O::Error: StdError + 'static,
{
    olua::get_lua().create_function(move |_lua, args| {
        // func(args).into_result().into_lua_err()
        func(args).into_result()
    })
}

/// Wraps a Rust `<FnMut>` function in a
/// [`mlua::Function`](https://docs.rs/mlua/latest/mlua/struct.Function.html).
pub fn wrap_fn_mut<A, F, O, R>(func: F) -> mlua::Result<mlua::Function>
where
    F: Fn(A) -> O + mlua::MaybeSend + 'static,
    A: FromLuaMulti,
    O: IntoResult<R>,
    R: IntoLuaMulti,
    // O::Error: StdError + 'static,
{
    olua::get_lua().create_function_mut(move |_lua, args| {
        // func(args).into_result().into_lua_err()
        func(args).into_result()
    })
}

/// Wraps a Rust `<FnOnce>` function in a
/// [`mlua::Function`](https://docs.rs/mlua/latest/mlua/struct.Function.html).
pub fn wrap_fn_once<A, F, O, R>(fun: F) -> mlua::Result<mlua::Function>
where
    F: Fn(A) -> O + mlua::MaybeSend + 'static,
    A: FromLuaMulti,
    O: IntoResult<R>,
    R: IntoLuaMulti,
    // O::Error: StdError + 'static,
{
    let fun = RefCell::new(Some(fun));

    olua::get_lua().create_function_mut(move |_lua, args| {
        let fun = fun
            .try_borrow_mut()
            .map_err(|_| mlua::Error::RecursiveMutCallback)?
            .take()
            .ok_or("Cannot call function twice")
            .into_lua_err()?;

        // fun(args).into_result().into_lua_err()
        fun(args).into_result()
    })
}
