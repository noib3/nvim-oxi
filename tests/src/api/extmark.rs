use nvim_oxi::api::{self, opts::*, types::*, Buffer};

#[nvim_oxi::test]
fn add_highlight() {
    let mut buf = Buffer::current();
    let id = api::create_namespace("Foo");
    let res = buf.add_highlight(id, "Normal", 0, ..);
    assert!(res.is_ok(), "{res:?}");
}

#[nvim_oxi::test]
fn clear_namespace() {
    let mut buf = Buffer::current();
    let id = api::create_namespace("Foo");
    let res = buf.clear_namespace(id, ..);
    assert_eq!(Ok(()), res);
}

#[nvim_oxi::test]
fn get_extmarks() {
    let mut buf = Buffer::current();
    let ns_id = api::create_namespace("Foo");

    let opts = SetExtmarkOpts::builder()
        .conceal(Some('a'))
        .cursorline_hl_group("Foo")
        .end_right_gravity(true)
        .end_row(0)
        .hl_group("Bar")
        .hl_mode(ExtmarkHlMode::Combine)
        .virt_lines([[("foo", "Foo"), ("bar", "Bar")]])
        .virt_text([("foo", vec!["Foo", "Bar"])])
        .virt_text_pos(ExtmarkVirtTextPosition::Overlay)
        .build();

    let extmark_id = buf.set_extmark(ns_id, 0, 0, &opts).unwrap();

    let start = ExtmarkPosition::ById(extmark_id);
    let end = ExtmarkPosition::ById(extmark_id);
    let opts = GetExtmarksOpts::builder().details(true).build();

    let res = buf
        .get_extmarks(ns_id, start, end, &opts)
        .map(|iter| iter.collect::<Vec<_>>());

    assert!(res.is_ok(), "{res:?}");

    let extmarks = res.unwrap();
    assert_eq!(1, extmarks.len());

    let (id, row, col, infos) = extmarks.into_iter().next().unwrap();

    assert!(infos.is_some(), "no informations were returned");

    let infos = infos.unwrap();

    assert_eq!(extmark_id, id);
    assert_eq!((row, col), (0, 0));
    assert_eq!(Some(true), infos.end_right_gravity);
    assert_eq!(Some(0), infos.end_row);
    assert_eq!(infos.hl_group.unwrap(), String::from("Bar"));
    assert_eq!(Some(ExtmarkHlMode::Combine), infos.hl_mode);

    let virt_text = vec![ExtmarkVirtTextChunk {
        text: "foo".to_owned(),
        hl_groups: vec!["Foo".into(), "Bar".into()],
    }];

    assert_eq!(infos.virt_text, virt_text);

    assert_eq!(Some(ExtmarkVirtTextPosition::Overlay), infos.virt_text_pos);
}

#[nvim_oxi::test]
fn get_namespaces() {
    let id = api::create_namespace("Foo");

    let out = api::get_namespaces()
        .find_map(|(name, id)| (name == "Foo").then_some(id))
        .unwrap();

    assert_eq!(id, out);
}

#[nvim_oxi::test]
fn set_decoration_provider() {
    let id = api::create_namespace("Foo");

    let opts = DecorationProviderOpts::builder()
        .on_start(|args| {
            nvim_oxi::print!("{args:?}");
            true
        })
        .on_buf(|args| {
            nvim_oxi::print!("{args:?}");
        })
        .on_win(|args| {
            nvim_oxi::print!("{args:?}");
            true
        })
        .on_line(|args| {
            nvim_oxi::print!("{args:?}");
        })
        .on_end(|args| {
            nvim_oxi::print!("{args:?}");
        })
        .build();

    let res = api::set_decoration_provider(id, &opts);
    assert_eq!(Ok(()), res);

    // TODO: I don't think the callbacks are getting triggered. If they were
    // `print!`'s output would be written to stdout, causing `test_all` to
    // fail.

    let bytes_written = api::input("ifoo<Esc>");
    assert!(bytes_written.is_ok(), "{bytes_written:?}");
}

#[nvim_oxi::test]
fn set_get_del_extmark() {
    let mut buf = Buffer::current();
    let ns_id = api::create_namespace("Foo");

    let opts = SetExtmarkOpts::builder()
        .conceal(Some('a'))
        .cursorline_hl_group("Foo")
        .end_right_gravity(true)
        .end_row(0)
        .hl_group("Bar")
        .hl_mode(ExtmarkHlMode::Combine)
        .virt_lines([[("foo", "Foo"), ("bar", "Bar")]])
        .virt_text([("foo", vec!["Foo"]), ("bar", vec!["Bar", "Baz"])])
        .virt_text_pos(ExtmarkVirtTextPosition::Overlay)
        .build();

    let res = buf.set_extmark(ns_id, 0, 0, &opts);
    assert!(res.is_ok(), "{res:?}");

    let extmark_id = res.unwrap();

    let opts = GetExtmarkByIdOpts::builder().details(true).build();
    let got = buf.get_extmark_by_id(ns_id, extmark_id, &opts);
    assert!(got.is_ok(), "{got:?}");

    let (row, col, infos) = got.unwrap();
    assert_eq!((row, col), (0, 0));

    assert!(infos.is_some(), "no informations were returned");

    let infos = infos.unwrap();
    assert_eq!(Some(true), infos.end_right_gravity);
    assert_eq!(Some(0), infos.end_row);
    assert_eq!(infos.hl_group.unwrap(), String::from("Bar"));
    assert_eq!(Some(ExtmarkHlMode::Combine), infos.hl_mode);

    let virt_text = vec![
        ExtmarkVirtTextChunk {
            text: "foo".to_owned(),
            hl_groups: vec!["Foo".into()],
        },
        ExtmarkVirtTextChunk {
            text: "bar".to_owned(),
            hl_groups: vec!["Bar".into(), "Baz".into()],
        },
    ];

    assert_eq!(infos.virt_text, virt_text);

    assert_eq!(Some(ExtmarkVirtTextPosition::Overlay), infos.virt_text_pos);

    let res = buf.del_extmark(ns_id, extmark_id);
    assert_eq!(Ok(()), res);
}

#[nvim_oxi::test]
fn virt_text_pos_inline() {
    let mut buf = Buffer::current();

    let ns_id = api::create_namespace("test");

    let opts = SetExtmarkOpts::builder()
        .virt_text([("", "")])
        .virt_text_pos(ExtmarkVirtTextPosition::Inline)
        .build();

    let extmark_id = buf.set_extmark(ns_id, 0, 0, &opts).unwrap();

    let opts = GetExtmarkByIdOpts::builder().details(true).build();

    let Ok((_, _, Some(infos))) =
        buf.get_extmark_by_id(ns_id, extmark_id, &opts)
    else {
        unreachable!()
    };

    assert_eq!(infos.virt_text_pos, Some(ExtmarkVirtTextPosition::Inline));
}
