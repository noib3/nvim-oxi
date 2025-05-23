use nvim_oxi::api::TabPage;

#[nvim_oxi::test]
fn get_list_wins() {
    let tab = TabPage::current();

    let win = tab.get_win();
    assert!(win.is_ok(), "{win:?}");
    let win = win.unwrap();

    let all_wins = tab.list_wins().map(|wins| wins.collect::<Vec<_>>());
    assert!(all_wins.is_ok(), "{all_wins:?}");
    let all_wins = all_wins.unwrap();

    assert_eq!(1, all_wins.len());
    assert_eq!(win, all_wins.into_iter().next().unwrap());
}

#[nvim_oxi::test]
fn tabpage_get_number() {
    assert_eq!(Ok(1), TabPage::current().get_number())
}

#[nvim_oxi::test]
fn is_valid() {
    assert!(TabPage::current().is_valid());
}

#[nvim_oxi::test]
fn tabpage_set_get_del_var() {
    let mut tab = TabPage::current();
    tab.set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), tab.get_var("foo"));
    assert_eq!(Ok(()), tab.del_var("foo"));
}

#[nvim_oxi::test]
fn tabpage_set_get_win() {
    let config = nvim_oxi::api::types::WindowConfig::builder()
        .relative(nvim_oxi::api::types::WindowRelativeTo::Editor)
        .height(10)
        .width(5)
        .row(1.5)
        .col(1.5)
        .build();

    let window = nvim_oxi::api::open_win(
        &nvim_oxi::api::Buffer::current(),
        true,
        &config,
    )
    .unwrap();

    let mut tab = TabPage::current();

    tab.set_win(&window).unwrap();

    assert_eq!(tab.get_win().unwrap(), window);
}
