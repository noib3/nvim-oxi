use nvim_oxi as oxi;
use nvim_oxi::api::{self, types::*, Buffer, TabPage, Window};

#[oxi::test]
fn win_call() {
    let win = Window::current();
    let res = win.call(|_| Ok(()));
    assert_eq!(Ok(()), res);
}

#[oxi::test]
fn close_hide() {
    let config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .height(10)
        .width(5)
        .row(1.5)
        .col(1.5)
        .build();

    let buf = Buffer::current();

    let win = api::open_win(&buf, false, &config).unwrap();
    assert_eq!(Ok(()), win.close(false));

    let win = api::open_win(&buf, false, &config).unwrap();
    assert_eq!(Ok(()), win.hide());
}

#[oxi::test]
fn win_get_number() {
    assert_eq!(Ok(1), Window::current().get_number());
}

#[oxi::test]
fn get_position() {
    assert_eq!(Ok((0, 0)), Window::current().get_position());
}

#[oxi::test]
fn get_set_buf() {
    let mut win = Window::current();

    assert_eq!(Ok(Buffer::current()), win.get_buf());

    let buf = api::create_buf(true, false).unwrap();
    assert_eq!(Ok(()), win.set_buf(&buf));

    let res = win.call(move |_| {
        let win = Window::current();
        assert_eq!(Ok(buf), win.get_buf());
        Ok(())
    });

    assert_eq!(Ok(()), res);
}

#[oxi::test]
fn get_set_height_width() {
    let config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .height(10)
        .width(5)
        .row(1.5)
        .col(1.5)
        .build();

    let buf = Buffer::current();

    let mut win = api::open_win(&buf, false, &config).unwrap();

    assert_eq!(10, win.get_height().unwrap());
    assert_eq!(5, win.get_width().unwrap());

    assert_eq!(Ok(()), win.set_height(5));
    assert_eq!(Ok(()), win.set_width(10));

    assert_eq!(5, win.get_height().unwrap());
    assert_eq!(10, win.get_width().unwrap());
}

#[oxi::test]
fn get_tabpage() {
    assert_eq!(Ok(TabPage::current()), Window::current().get_tabpage())
}

#[oxi::test]
fn set_get_cursor() {
    let mut buf = Buffer::current();
    buf.set_lines(0, 1, true, ["foo"]).unwrap();

    let mut win = Window::current();

    assert_eq!(Ok(()), win.set_cursor(1, 2));
    assert_eq!(Ok((1, 2)), win.get_cursor());

    assert_eq!(Ok(()), win.set_cursor(1, 42));
    assert_eq!(Ok((1, 2)), win.get_cursor());

    buf.set_lines(0, 1, true, [""]).unwrap();

    assert_eq!(Ok((1, 0)), win.get_cursor());
}

#[oxi::test]
fn win_set_get_option() {
    let mut win = Window::current();

    win.set_option("spell", true).unwrap();
    assert!(win.get_option::<bool>("spell").unwrap());

    win.set_option("spell", false).unwrap();
    assert!(!win.get_option::<bool>("spell").unwrap());
}

#[oxi::test]
fn win_set_get_del_var() {
    let mut win = Window::current();
    win.set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), win.get_var("foo"));
    assert_eq!(Ok(()), win.del_var("foo"));
}
