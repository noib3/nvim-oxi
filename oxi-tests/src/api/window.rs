use nvim_oxi::api::{Buffer, Window};

pub fn set_get_cursor() {
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

pub fn set_get_del_var() {
    let mut win = Window::current();
    win.set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), win.get_var("foo"));
    assert_eq!(Ok(()), win.del_var("foo"));
}
