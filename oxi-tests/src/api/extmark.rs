use nvim_oxi::{
    api::{self, Buffer},
    opts::*,
    types::*,
};

pub fn add_highlight() {
    let mut buf = Buffer::current();
    let id = api::create_namespace("Foo");
    let res = buf.add_highlight(id, "Normal", 0, 0, 1);
    assert!(res.is_ok(), "{res:?}");
}

pub fn clear_namespace() {
    let mut buf = Buffer::current();
    let id = api::create_namespace("Foo");
    let res = buf.clear_namespace(id, 0, -1);
    assert_eq!(Ok(()), res);
}

pub fn get_namespaces() {
    let id = api::create_namespace("Foo");

    let out = api::get_namespaces()
        .find_map(|(name, id)| (name == "Foo").then(|| id))
        .unwrap();

    assert_eq!(id, out);
}

pub fn set_decoration_provider() {
    use nvim_oxi::print;

    let id = api::create_namespace("Foo");

    let opts = DecorationProviderOpts::builder()
        .on_start(|args| {
            print!("{args:?}");
            Ok(true)
        })
        .on_buf(|args| {
            print!("{args:?}");
            Ok(())
        })
        .on_win(|args| {
            print!("{args:?}");
            Ok(true)
        })
        .on_line(|args| {
            print!("{args:?}");
            Ok(())
        })
        .on_end(|args| {
            print!("{args:?}");
            Ok(())
        })
        .build();

    let res = api::set_decoration_provider(id, opts);
    assert_eq!(Ok(()), res);

    // TODO: I don't think the callbacks are getting triggered. If they were
    // `print!`'s output would be written to stdout, causing `test_all` to
    // fail.

    let bytes_written = api::input("ifoo<Esc>");
    assert!(bytes_written.is_ok(), "{bytes_written:?}");
}

pub fn set_extmark() {
    let mut buf = Buffer::current();
    let id = api::create_namespace("Foo");

    let opts = SetExtmarkOpts::builder()
        .conceal(Some('a'))
        .cursorline_hl_group("Foo")
        .end_right_gravity(true)
        .end_row(0)
        .hl_group("Bar")
        .hl_mode(ExtmarkHlMode::Combine)
        // TODO
        // both `opts = {virt_text={"foo", "Foo"}}`
        // and `opts = {virt_text={"foo", {"Foo", "Bar"}}}`
        // cause problems. Open issue upstream
        // .virt_lines([("foo", "Foo"), ("bar", "Bar")])
        .virt_text([("foo", ["Foo", "Bar"])])
        .virt_text_pos(ExtmarkVirtTextPosition::Overlay)
        .build();

    let res = buf.set_extmark(id, 0, 0, &opts);

    assert!(res.is_ok(), "{res:?}");
}
