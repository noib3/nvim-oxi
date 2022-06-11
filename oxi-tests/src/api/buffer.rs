use nvim_oxi::{
    self as nvim,
    api::{self, opts::*, Buffer},
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
    assert!(res.is_ok());
}

pub fn create_user_command() {
    let buf = Buffer::current();
    let opts = CreateCommandOpts::builder().build();

    let res = buf.create_user_command("Foo", ":lua print('foo')", &opts);
    assert!(res.is_ok());

    let cb = Box::new(|()| {
        nvim::print!("bar!");
        Ok(())
    }) as Box<dyn Fn(()) -> nvim::Result<()>>;
    let res = buf.create_user_command("Bar", cb, &opts);
    assert!(res.is_ok());
}

pub fn get_changedtick() {
    let buf = Buffer::current();
    assert!(buf.get_changedtick().is_ok());
}

pub fn set_lines() {
    let mut buf = api::create_buf(true, false).unwrap();
    assert!(buf.set_lines(0, 0, false, ["foo", "bar", "baz"]).is_ok());
    let opts = BufDeleteOpts::builder().force(true).unload(true).build();
    assert!(buf.delete(opts).is_ok());
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
