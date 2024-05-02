use nvim_oxi::{
    self as nvim,
    api::{self, TabPage},
};

#[nvim::test]
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

#[nvim::test]
fn tabpage_get_number() {
    assert_eq!(Ok(1), TabPage::current().get_number())
}

#[nvim::test]
fn is_valid() {
    assert!(TabPage::current().is_valid());
}

#[nvim::test]
fn tabpage_set_get_del_var() {
    let mut tab = TabPage::current();
    tab.set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), tab.get_var("foo"));
    assert_eq!(Ok(()), tab.del_var("foo"));
}

#[nvim::test]
#[cfg(feature = "neovim-nightly")]
fn tabpage_set_get_win() {
    let config = api::types::WindowConfig::builder()
        .relative(api::types::WindowRelativeTo::Editor)
        .height(10)
        .width(5)
        .row(1.5)
        .col(1.5)
        .build();

    let window =
        api::open_win(&api::Buffer::current(), true, &config).unwrap();

    let mut tab = TabPage::current();

    tab.set_win(&window).unwrap();

    assert_eq!(tab.get_win().unwrap(), window);
}
