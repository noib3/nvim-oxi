use mlua::prelude::LuaFunction;
use nvim_oxi::{mlua::lua, print, Result};

#[nvim_oxi::plugin]
fn mlua() -> Result<()> {
    print!("Hello from nvim-oxi..");
    let lua = lua();
    let print: LuaFunction = lua.globals().get("print")?;
    print.call("..and goodbye from mlua!")?;
    Ok(())
}
