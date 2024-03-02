use mlua::prelude::LuaFunction;
use nvim_oxi::{mlua::lua, print, Result};

#[nvim_oxi::plugin]
fn mlua() -> Result<()> {
    print!("Hello from nvim-oxi..");
    let lua = lua();
    let print = lua.globals().get::<_, LuaFunction>("print")?;
    print.call("..and goodbye from mlua!")?;
    Ok(())
}
