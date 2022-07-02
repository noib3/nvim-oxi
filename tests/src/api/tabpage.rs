use nvim_oxi::{self as oxi, api::TabPage};

#[oxi::test]
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

#[oxi::test]
fn tabpage_get_number() {
    assert_eq!(Ok(1), TabPage::current().get_number())
}

#[oxi::test]
fn is_valid() {
    assert!(TabPage::current().is_valid());
}

#[oxi::test]
fn tabpage_set_get_del_var() {
    let mut tab = TabPage::current();
    tab.set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), tab.get_var("foo"));
    assert_eq!(Ok(()), tab.del_var("foo"));
}
