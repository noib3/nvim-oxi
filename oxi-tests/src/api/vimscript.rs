use nvim_oxi::api;

// pub fn call_function() {
//     let res = api::call_function::<_, usize>("strwidth", ("foo bar"));
//     assert_eq!(Ok(7), res);
// }

pub fn command() {
    let res = api::command(":lua vim.api.nvim_buf_set_var(0, 'foo', 'bar')");
    assert_eq!(Ok(()), res);

    assert_eq!(
        Ok("bar".into()),
        api::get_current_buf().get_var::<String>("foo")
    );
}

pub fn eval() {
    let res = api::eval::<u8>("41 + 1");
    assert_eq!(Ok(42), res);

    let res = api::eval::<u8>(&format!("{} * 2 - 15", res.unwrap()));
    assert_eq!(Ok(69), res); // nice
}

pub fn exec() {
    let no_op = api::exec(":", true);
    assert_eq!(Ok(None), no_op);

    let add = api::exec(":echo 1 + 1", true);
    assert_eq!(Ok(Some("2".into())), add);
}
