use all_asserts::*;
use nvim_oxi::{
    self as oxi,
    api::{self, Buffer},
    opts::*,
    types::*,
};

#[oxi::test]
fn chan_send_fail() {
    let res = api::chan_send(42, "hello there");
    assert!(res.is_err());
}

#[oxi::test]
fn create_del_user_command() {
    let res = api::create_user_command("Foo", ":", None);
    assert_eq!(Ok(()), res);
    api::command("Foo").unwrap();

    let res = api::create_user_command("Bar", |_args| Ok(()), None);
    assert_eq!(Ok(()), res);
    api::command("Bar").unwrap();

    assert_eq!(2, api::get_commands(None).unwrap().collect::<Vec<_>>().len());

    assert_eq!(Ok(()), api::del_user_command("Foo"));
    assert_eq!(Ok(()), api::del_user_command("Bar"));
}

#[oxi::test]
fn user_command_with_count() {
    let opts = CreateCommandOpts::builder().count(32).build();
    api::create_user_command("Foo", "echo 'foo'", Some(&opts)).unwrap();

    let res = api::get_commands(None).map(|cmds| cmds.collect::<Vec<_>>());
    assert!(res.is_ok(), "{res:?}");
}

#[oxi::test]
fn eval_statusline() {
    let opts = EvalStatuslineOpts::builder().highlights(true).build();
    let res = api::eval_statusline("foo", Some(&opts));
    assert_eq!(Ok("foo".into()), res.map(|infos| infos.str));
}

#[oxi::test]
fn get_chan_info() {
    let res = api::get_chan_info(0);
    assert!(res.is_err());
}

#[oxi::test]
fn get_colors() {
    let colors = api::get_color_map().collect::<Vec<_>>();
    assert_lt!(0, colors.len());

    let (name, color) = colors.into_iter().next().unwrap();
    assert_eq!(color, api::get_color_by_name(&name).unwrap());
}

#[oxi::test]
fn get_context() {
    let res = api::get_context(None);
    assert!(res.is_ok());
}

#[oxi::test]
fn get_highlights() {
    let (name, _) = api::get_color_map().next().unwrap();
    let id = api::get_hl_id_by_name(&*name).unwrap();
    assert_eq!(api::get_hl_by_id(id, true), api::get_hl_by_name(&name, true));
}

#[oxi::test]
fn get_mode() {
    let got_mode = api::get_mode().unwrap();
    assert_eq!(Mode::Normal, got_mode.mode);
    assert!(!got_mode.blocking);
}

#[oxi::test]
fn get_options() {
    let res = api::get_all_options_info();
    assert_lt!(0, res.unwrap().collect::<Vec<_>>().len());
}

#[oxi::test]
fn get_option_info() {
    api::set_option("number", true).unwrap();
    assert!(api::get_option_info("number").is_ok());
}

#[oxi::test]
fn hl_underline() {
    let opts = SetHighlightOpts::builder().underline(true).build();
    api::set_hl(0, "MatchParen", Some(&opts)).unwrap();

    let infos = api::get_hl_by_name("MatchParen", true).unwrap();
    assert_eq!(Some(true), infos.underline);
}

#[oxi::test]
fn set_get_del_current_line() {
    let res = api::set_current_line("foo");
    assert_eq!(Ok(()), res);

    let res = api::get_current_line();
    assert_eq!(Ok("foo".into()), res);

    let res = api::del_current_line();
    assert_eq!(Ok(()), res);
}

#[oxi::test]
fn set_get_del_keymap() {
    let opts = SetKeymapOpts::builder()
        .callback(|_| Ok(()))
        .desc("does nothing")
        .expr(true)
        .build();

    let res = api::set_keymap(Mode::Insert, "a", "", Some(&opts));
    assert_eq!(Ok(()), res);

    let keymaps = api::get_keymap(Mode::Insert).collect::<Vec<_>>();
    assert_le!(1, keymaps.len());

    let res = api::del_keymap(Mode::Insert, "a");
    assert_eq!(Ok(()), res);
}

#[oxi::test]
fn set_get_del_mark() {
    let mut buf = Buffer::current();

    let res = buf.set_mark('A', 1, 0);
    assert_eq!(Ok(()), res);

    assert_eq!((1, 0, buf, "".into()), api::get_mark('A', None).unwrap());

    let res = api::del_mark('A');
    assert_eq!(Ok(()), res);
}

#[oxi::test]
fn set_get_del_var() {
    api::set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), api::get_var("foo"));
    assert_eq!(Ok(()), api::del_var("foo"));
}

#[oxi::test]
fn set_get_option() {
    api::set_option("modified", true).unwrap();
    assert!(api::get_option::<bool>("modified").unwrap());

    api::set_option("modified", false).unwrap();
    assert!(!api::get_option::<bool>("modified").unwrap());
}

#[oxi::test]
fn strwidth() {
    assert_eq!(Ok(2), api::strwidth("｜"));
}
