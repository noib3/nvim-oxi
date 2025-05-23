use mlua::prelude::LuaFunction;
use nvim_oxi::{Result, mlua, print};

#[nvim_oxi::plugin]
fn mlua() -> Result<()> {
    print!("Hello from nvim-oxi..");
    let lua = mlua::lua();
    let print: LuaFunction = lua.globals().get("print")?;
    print.call::<()>("..and goodbye from mlua!")?;
    Ok(())
}
