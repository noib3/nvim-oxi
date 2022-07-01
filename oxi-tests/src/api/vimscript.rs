use nvim_oxi::{api, opts::*, types::*};

// pub fn call_function() {
//     let res = api::call_function::<_, usize>("strwidth", ("foo bar"));
//     assert_eq!(Ok(7), res);
// }

// pub fn cmd_basic() {
//     let opts = CmdOpts::builder().output(true).build();
//     let infos = CmdInfos::builder().cmd("echo 'foo'").build();
//     assert_eq!(Ok(Some("foo".into())), api::cmd(&infos, &opts));
// }

pub fn cmd_no_output() {
    let opts = CmdOpts::builder().output(false).build();
    let infos = CmdInfos::builder().cmd("echo 'foo'").build();
    assert_eq!(Ok(None), api::cmd(&infos, &opts));
}

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

pub fn parse_cmd_basic() {
    let opts = ParseCmdOpts::builder().build();

    let res = api::parse_cmd("echo 'foo'", &opts);
    assert!(res.is_ok(), "{res:?}");

    let infos = res.unwrap();

    assert_eq!(None, infos.addr);
    assert_eq!(vec!["'foo'"], infos.args);
    assert_eq!(Some(false), infos.bang);
    assert_eq!(Some("echo".into()), infos.cmd);
    assert_eq!(None, infos.count);

    let magic = infos.magic.unwrap();
    assert_eq!(false, magic.file);
    assert_eq!(false, magic.bar);

    let mods = infos.mods.unwrap();
    assert_eq!(false, mods.browse);
    assert_eq!(false, mods.confirm);
    assert_eq!(false, mods.emsg_silent);
    assert_eq!(false, mods.hide);
    assert_eq!(false, mods.keepalt);
    assert_eq!(false, mods.keepjumps);
    assert_eq!(false, mods.keepmarks);
    assert_eq!(false, mods.keeppatterns);
    assert_eq!(false, mods.lockmarks);
    assert_eq!(false, mods.noautocmd);
    assert_eq!(false, mods.sandbox);
    assert_eq!(false, mods.silent);
    assert_eq!(None, mods.split);
    assert_eq!(0, mods.tab);
    assert_eq!(false, mods.vertical);

    assert_eq!(Some(CommandNArgs::Any), infos.nargs);
    assert_eq!(None, infos.nextcmd);
    assert_eq!(Some(CmdRange::None), infos.range);
}
