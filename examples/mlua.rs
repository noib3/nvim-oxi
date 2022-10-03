use mlua::prelude::LuaFunction;
use nvim_oxi as oxi;

#[oxi::module]
fn mlua() -> oxi::Result<()> {
    oxi::print!("Hello from nvim-oxi..");

    let lua = oxi::mlua::lua();
    let print = lua.globals().get::<_, LuaFunction>("print")?;
    print.call("..and goodbye from mlua!")?;

    Ok(())
}
