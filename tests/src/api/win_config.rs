use nvim_oxi as oxi;
use nvim_oxi::api::{self, types::*, Buffer, Window};

#[oxi::test]
fn open_win_empty_config() {
    let buf = Buffer::current();
    let config = WindowConfig::builder().build();
    let res = api::open_win(&buf, false, &config);
    assert!(
        res.is_err(),
        "config is missing required fields `relative`, `height` and `width`"
    );
}

#[oxi::test]
fn open_win_basic_config() {
    let buf = api::create_buf(true, true).unwrap();
    let config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .height(10)
        .width(5)
        .row(1.5)
        .col(1.5)
        .build();

    let res = api::open_win(&buf, false, &config);
    assert!(res.is_ok(), "{res:?}");

    let win = res.unwrap();

    let got = win.get_config();
    assert!(got.is_ok(), "{got:?}");

    let got = got.unwrap();
    assert_eq!(config.relative.unwrap(), got.relative.unwrap());
    assert_eq!(config.height.unwrap(), got.height.unwrap());
    assert_eq!(config.width.unwrap(), got.width.unwrap());
    assert_eq!(config.row.unwrap(), got.row.unwrap());
    assert_eq!(config.col.unwrap(), got.col.unwrap());
}

#[oxi::test]
fn open_win_full_config() {
    let buf = api::create_buf(true, true).unwrap();

    let config = WindowConfig::builder()
        .relative(WindowRelativeTo::Window(Window::current()))
        .anchor(WindowAnchor::SouthWest)
        .height(10)
        .width(5)
        .bufpos(7, 5)
        .row(1.5)
        .col(1.5)
        .focusable(false)
        .external(false)
        .zindex(300u32)
        .style(WindowStyle::Minimal)
        .border(WindowBorder::from((
            None, None, None, '>', None, None, None, '<',
        )))
        .build();

    let res = api::open_win(&buf, false, &config);
    assert!(res.is_ok(), "{res:?}");

    let win = res.unwrap();

    let got = win.get_config();
    assert!(got.is_ok(), "{got:?}");

    let got = got.unwrap();
    assert_eq!(config.relative.unwrap(), got.relative.unwrap());
    assert_eq!(config.height.unwrap(), got.height.unwrap());
    assert_eq!(config.width.unwrap(), got.width.unwrap());
    assert_eq!(config.row.unwrap(), got.row.unwrap());
    assert_eq!(config.col.unwrap(), got.col.unwrap());
    assert_eq!(config.border.unwrap(), got.border.unwrap());
}

#[cfg(feature = "neovim-nightly")]
#[oxi::test]
fn open_split_win() {
    let buf = api::create_buf(true, true).unwrap();
    let old_win = api::get_current_win();

    let config = WindowConfig::builder()
        .vertical(true)
        .split(SplitDirection::Right)
        .build();

    let res = api::open_win(&buf, true, &config);
    assert!(res.is_ok(), "{res:?}");

    let win = res.unwrap();

    let got = win.get_config();
    assert!(got.is_ok(), "{got:?}");

    let got = got.unwrap();
    assert_eq!(config.split.unwrap(), got.split.unwrap());

    let new_win = api::get_current_win();
    assert_ne!(old_win, new_win);
}

#[oxi::test]
fn set_config() {
    let buf = api::create_buf(true, true).unwrap();

    let initial = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .height(10)
        .width(5)
        .row(1.5)
        .col(1.5)
        .build();

    let mut win = api::open_win(&buf, false, &initial).unwrap();

    let config = WindowConfig::builder()
        .relative(WindowRelativeTo::Window(Window::current()))
        .anchor(WindowAnchor::SouthWest)
        .height(10)
        .width(5)
        .bufpos(7, 5)
        .row(1.5)
        .col(1.5)
        .focusable(false)
        .external(false)
        .zindex(300)
        .style(WindowStyle::Minimal)
        .border(WindowBorder::from((
            None, None, None, '>', None, None, None, '<',
        )))
        .build();

    assert_eq!(Ok(()), win.set_config(&config));
}
