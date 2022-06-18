use nvim_oxi::{
    api::{self, opts::*, types::*, Buffer},
    LuaFun,
};

pub fn attach() {
    let buf = Buffer::current();

    let opts = BufAttachOpts::builder()
        .on_lines(|_args| Ok(false))
        .on_bytes(|_args| Ok(false))
        .on_detach(|_args| Ok(false))
        .on_reload(|_args| Ok(false))
        .on_changedtick(|_args| Ok(false))
        .build();

    let has_attached = buf.attach(false, opts).expect("attach failed");

    assert!(has_attached);
}

pub fn call() {
    let buf = Buffer::current();
    let res = buf.call(|_| Ok(()));
    assert_eq!(Ok(()), res);
}

pub fn create_user_command() {
    let buf = Buffer::current();
    let opts = CreateCommandOpts::builder().build();

    let res = buf.create_user_command("Foo", ":lua print('foo')", &opts);
    assert_eq!(Ok(()), res);
    // TODO: `api::nvim_command("Foo")`

    let cb = LuaFun::from_fn(|_args: CommandArgs| Ok(()));
    let res = buf.create_user_command("Bar", cb, &opts);
    assert_eq!(Ok(()), res);
    // TODO: `api::nvim_command("Foo")`
}

pub fn set_get_del_keymap() {
    let mut buf = Buffer::current();

    let opts = SetKeymapOpts::builder()
        .callback(|_| Ok(()))
        .desc("does nothing")
        .expr(true)
        .build();

    let res = buf.set_keymap(Mode::Insert, "a", None, &opts);
    assert_eq!(Ok(()), res);

    let keymaps = buf.get_keymap(Mode::Insert).unwrap().collect::<Vec<_>>();
    assert_eq!(1, keymaps.len());

    let res = buf.del_keymap(Mode::Insert, "a");
    assert_eq!(Ok(()), res);
}

pub fn set_get_del_mark() {
    let mut buf = Buffer::current();

    let res = buf.set_mark('a', 1, 0);
    assert_eq!(Ok(true), res);

    assert_eq!((1, 0), buf.get_mark('a').unwrap());

    let res = buf.del_mark('a');
    assert_eq!(Ok(true), res);
}

pub fn get_name() {
    let buf = Buffer::current();
    assert_eq!("", buf.get_name().unwrap().display().to_string());
}

pub fn get_changedtick() {
    let buf = Buffer::current();
    assert!(buf.get_changedtick().is_ok());
}

pub fn set_lines() {
    let mut buf = api::create_buf(true, false).unwrap();
    assert!(buf.set_lines(0, 0, false, ["foo", "bar", "baz"]).is_ok());
    let opts = BufDeleteOpts::builder().force(true).unload(true).build();
    assert_eq!(Ok(()), buf.delete(opts));
}

pub fn set_option() {
    let mut buf = Buffer::current();

    buf.set_option("modified", true).unwrap();
    assert!(buf.get_option::<bool>("modified").unwrap());

    buf.set_option("modified", false).unwrap();
    assert!(!buf.get_option::<bool>("modified").unwrap());
}

pub fn set_var() {
    let mut buf = Buffer::current();
    buf.set_var("foo", 42).unwrap();
    assert_eq!(42, buf.get_var("foo").unwrap());
}
