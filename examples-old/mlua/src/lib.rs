use mlua::prelude::LuaFunction;
use nvim_oxi::{self as oxi, Dictionary, Function};

fn greetings(_: ()) -> oxi::Result<()> {
    oxi::print!("Hello from Rust..");

    let lua = oxi::mlua::lua();
    let print = lua.globals().get::<_, LuaFunction>("print")?;
    print.call("..and goodbye from Lua!")?;

    Ok(())
}

#[oxi::module]
fn lua() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([("greetings", Function::from_fn(greetings))]))
}
