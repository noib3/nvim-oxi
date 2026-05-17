use nvim_oxi::{
    api,
    mlua::{self, lua},
};

#[nvim_oxi::test]
fn get_variable() {
    api::set_var("a_varialbe", "Hello!").unwrap();

    let lua = lua();

    let a_variable = lua.load("vim.g.a_varialbe").eval::<String>().unwrap();
    let not_a_variable =
        lua.load("vim.g.not_a_varialbe").eval::<mlua::Value>().unwrap();

    assert!(not_a_variable.is_nil());
    assert_eq!(a_variable, "Hello!");
}

#[nvim_oxi::test]
fn set_variable() {
    lua().load("vim.g.a_varialbe = 'Hello!'").exec().unwrap();

    let var = api::get_var::<String>("a_varialbe").unwrap();
    assert_eq!(var, "Hello!");
}

#[nvim_oxi::test]
fn nvim_api() {
    let hello = "Hello!";

    let hello1 = lua()
        .load(format!(
            "
            buf = vim.api.nvim_create_buf(true, false)
            vim.api.nvim_buf_set_lines(buf, 0, 1, true, {{ '{hello}' }})
            return vim.api.nvim_buf_get_lines(buf, 0, 1, true)[1]
            "
        ))
        .eval::<String>()
        .unwrap();

    assert_eq!(hello1, hello);
}

#[nvim_oxi::test]
fn rust_callback() {
    let hello: &'static str = "Hello!";
    let lua = lua();
    let func = lua
        .create_function(|_, ()| {
            api::set_var("a_varialbe", "Hello!").unwrap();
            Ok(())
        })
        .unwrap();

    lua.globals().set("rust_callback", func).unwrap();
    let hello1 = lua
        .load("rust_callback(); return vim.g.a_varialbe")
        .eval::<String>()
        .unwrap();
    let hello2 = api::get_var::<String>("a_varialbe").unwrap();

    assert_eq!(hello1, hello);
    assert_eq!(hello2, hello);
}
