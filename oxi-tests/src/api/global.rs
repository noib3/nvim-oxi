use all_asserts::*;
use nvim_oxi::{
    api::{self, Buffer},
    opts::*,
    types::*,
    LuaFun,
};

pub fn chan_send_fail() {
    let res = api::chan_send(42, "hello there");
    assert!(res.is_err());
}

pub fn create_del_user_command() {
    let opts = CreateCommandOpts::builder().build();
    let res = api::create_user_command("Foo", ":lua print('foo')", &opts);
    assert_eq!(Ok(()), res);
    // TODO: `api::nvim_command("Foo")`

    let cb = LuaFun::from_fn(|_args: CommandArgs| Ok(()));
    let res = api::create_user_command("Bar", cb, &opts);
    assert_eq!(Ok(()), res);
    // TODO: `api::nvim_command("Bar")`

    let opts = GetCommandsOpts::builder().build();
    assert_eq!(2, api::get_commands(opts).unwrap().collect::<Vec<_>>().len());

    assert_eq!(Ok(()), api::del_user_command("Foo"));
    assert_eq!(Ok(()), api::del_user_command("Bar"));
}

pub fn eval_statusline() {
    let opts = EvalStatuslineOpts::builder().highlights(true).build();
    let res = api::eval_statusline("foo", opts);
    assert_eq!(Ok("foo".into()), res.map(|infos| infos.str));
}

pub fn get_chan_info() {
    let res = api::get_chan_info(0);
    assert!(res.is_err());
}

pub fn get_colors() {
    let colors = api::get_color_map().collect::<Vec<_>>();
    assert_lt!(0, colors.len());

    let (name, color) = colors.into_iter().next().unwrap();
    assert_eq!(color, api::get_color_by_name(&name));
}

pub fn get_context() {
    let opts = GetContextOpts::builder().build();
    let res = api::get_context(opts);
    assert!(res.is_ok());
}

pub fn get_highlights() {
    let (name, _) = api::get_color_map().next().unwrap();
    let id = api::get_hl_id_by_name(&*name).unwrap();
    assert_eq!(api::get_hl_by_id(id, true), api::get_hl_by_name(&name, true));
}

pub fn get_mode() {
    let got_mode = api::get_mode().unwrap();
    assert_eq!(Mode::Normal, got_mode.mode);
    assert!(!got_mode.blocking);
}

pub fn get_options() {
    let res = api::get_all_options_info();
    assert_lt!(0, res.unwrap().collect::<Vec<_>>().len());
}

pub fn set_get_del_current_line() {
    let res = api::set_current_line("foo");
    assert_eq!(Ok(()), res);

    let res = api::get_current_line();
    assert_eq!(Ok("foo".into()), res);

    let res = api::del_current_line();
    assert_eq!(Ok(()), res);
}

pub fn set_get_del_keymap() {
    let opts = SetKeymapOpts::builder()
        .callback(|_| Ok(()))
        .desc("does nothing")
        .expr(true)
        .build();

    let res = api::set_keymap(Mode::Insert, "a", None, &opts);
    assert_eq!(Ok(()), res);

    let keymaps = api::get_keymap(Mode::Insert).collect::<Vec<_>>();
    assert_le!(1, keymaps.len());

    let res = api::del_keymap(Mode::Insert, "a");
    assert_eq!(Ok(()), res);
}

pub fn set_get_del_mark() {
    let mut buf = Buffer::current();

    let res = buf.set_mark('A', 1, 0);
    assert_eq!(Ok(true), res);

    let opts = GetMarkOpts::builder().build();
    assert_eq!((1, 0, buf, "".into()), api::get_mark('A', opts).unwrap());

    let res = api::del_mark('A');
    assert_eq!(Ok(true), res);
}

pub fn set_get_del_var() {
    api::set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), api::get_var("foo"));
    assert_eq!(Ok(()), api::del_var("foo"));
}

pub fn set_get_option() {
    api::set_option("modified", true).unwrap();
    assert!(api::get_option::<_, bool>("modified").unwrap());

    api::set_option("modified", false).unwrap();
    assert!(!api::get_option::<_, bool>("modified").unwrap());
}

pub fn strwidth() {
    assert_eq!(Ok(2), api::strwidth("｜"));
}
