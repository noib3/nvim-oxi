use std::cell::Cell;
use std::ops;
use std::rc::Rc;

use all_asserts::*;
use nvim_oxi::api::{self, Buffer, opts::*, types::*};

#[nvim_oxi::test]
fn buf_attach() {
    let buf = Buffer::current();

    let opts = BufAttachOpts::builder()
        .on_lines(|_args| false)
        .on_bytes(|_args| false)
        .on_detach(|_args| false)
        .on_reload(|_args| false)
        .on_changedtick(|_args| false)
        .build();

    let res = buf.attach(false, &opts);
    assert_eq!(Ok(()), res);

    let bytes_written = api::input("ifoo<Esc>");
    assert!(bytes_written.is_ok(), "{bytes_written:?}");
}

#[nvim_oxi::test]
fn buf_attach_on_bytes() -> Result<(), api::Error> {
    let mut buffer = api::create_buf(true, false)?;

    let count = Rc::new(Cell::new(0));

    let opts = {
        let count = count.clone();

        BufAttachOpts::builder()
            .on_lines(move |_args| {
                count.set(count.get() + 1);
                false
            })
            .build()
    };

    buffer.attach(false, &opts)?;

    api::Window::current().set_buf(&buffer)?;

    buffer.set_text(0..0, 0, 0, [" "])?;
    buffer.set_text(0..0, 0, 0, [" "])?;
    buffer.set_text(0..0, 0, 0, [" "])?;

    assert_eq!(count.get(), 3);

    Ok(())
}

#[nvim_oxi::test]
fn buf_call_nil() {
    let buf = Buffer::current();
    let res = buf.call(|_| ());
    assert_eq!(Ok(()), res);
}

#[nvim_oxi::test]
fn buf_call_int() {
    let buf = Buffer::current();
    let res = buf.call(|_| 42);
    assert_eq!(Ok(42), res);
}

#[nvim_oxi::test]
fn buf_create_del_user_command() {
    let mut buf = Buffer::current();

    let res = buf.create_user_command("Foo", ":", &Default::default());
    assert_eq!(Ok(()), res);
    api::command("Foo").unwrap();

    let res = buf.create_user_command("Bar", |_args| (), &Default::default());
    assert_eq!(Ok(()), res);
    api::command("Bar").unwrap();

    assert_eq!(
        2,
        buf.get_commands(&Default::default())
            .unwrap()
            .collect::<Vec<_>>()
            .len()
    );

    assert_eq!(Ok(()), buf.del_user_command("Foo"));
    assert_eq!(Ok(()), buf.del_user_command("Bar"));
}

#[nvim_oxi::test]
fn buf_get_changedtick() {
    let buf = Buffer::current();
    assert!(buf.get_changedtick().is_ok());
}

#[nvim_oxi::test]
fn buf_get_lines_range_bounds() {
    let mut buf = api::create_buf(true, false).unwrap();

    // `get_lines` returns a single empty line even if the buffer is empty.
    let lines = buf.get_lines(.., true).unwrap().collect::<Vec<_>>();
    assert_eq!(lines, [""]);

    buf.set_text(0..0, 0, 0, ["Hello", "world"]).unwrap();

    for bound in [Range::new(..1), Range::new(0..1)] {
        let lines = buf.get_lines(bound, true).unwrap().collect::<Vec<_>>();
        assert_eq!(lines, ["Hello"]);
    }

    for bound in [Range::new(1..), Range::new(1..2)] {
        let lines = buf.get_lines(bound, true).unwrap().collect::<Vec<_>>();
        assert_eq!(lines, ["world"]);
    }

    for bound in [
        Range::new(..),
        Range::new(..2),
        Range::new(0..2),
        Range::new(..=1),
        Range::new(0..=1),
    ] {
        let lines = buf.get_lines(bound, true).unwrap().collect::<Vec<_>>();
        assert_eq!(lines, ["Hello", "world"]);
    }
}

#[nvim_oxi::test]
fn buf_get_name_invalid_buf() {
    let buf = Buffer::from(42);
    let err = buf.get_name().unwrap_err();
    assert!(matches!(err, api::Error::Nvim(_)));
}

#[nvim_oxi::test]
fn buf_loaded_n_valid() {
    let buf = Buffer::current();
    assert!(buf.is_loaded());
    assert!(buf.is_valid());
}

#[nvim_oxi::test]
fn buf_new_delete() {
    let buf = api::create_buf(true, false).unwrap();
    assert_eq!(Ok(()), buf.delete(&Default::default()));
}

#[nvim_oxi::test]
fn buf_set_get_del_keymap() {
    let mut buf = Buffer::current();

    let opts = SetKeymapOpts::builder()
        .callback(|_| ())
        .desc("does nothing")
        .expr(true)
        .build();

    let res = buf.set_keymap(Mode::Insert, "a", "", &opts);
    assert_eq!(Ok(()), res);

    let keymaps = buf.get_keymap(Mode::Insert).unwrap().collect::<Vec<_>>();
    assert_eq!(1, keymaps.len());
    assert!(keymaps.iter().all(|keymap| keymap.buffer == Some(buf.clone())));

    let res = buf.del_keymap(Mode::Insert, "a");
    assert_eq!(Ok(()), res);
}

/// Regression test for https://github.com/noib3/nvim-oxi/issues/226
#[nvim_oxi::test]
fn buf_set_get_keymap_with_bufnr_more_than_one() {
    let mut buf = api::create_buf(true, false).unwrap();

    assert!(buf.handle() > 1);

    let opts = SetKeymapOpts::builder()
        .callback(|_| ())
        .desc("does nothing")
        .expr(true)
        .build();

    let res = buf.set_keymap(Mode::Insert, "a", "", &opts);
    assert_eq!(Ok(()), res);

    let keymaps = buf.get_keymap(Mode::Insert).unwrap().collect::<Vec<_>>();
    assert_eq!(1, keymaps.len());
    assert!(keymaps.iter().all(|keymap| keymap.buffer == Some(buf.clone())));
}

#[nvim_oxi::test]
fn buf_set_get_del_nvo_keymap() {
    let mut buf = Buffer::current();

    let res = buf.set_keymap(
        Mode::NormalVisualOperator,
        "a",
        "b",
        &Default::default(),
    );
    assert_eq!(Ok(()), res);

    let keymaps = buf
        .get_keymap(Mode::NormalVisualOperator)
        .unwrap()
        .collect::<Vec<_>>();
    assert_le!(1, keymaps.len());
    assert!(keymaps.iter().all(|keymap| keymap.buffer == Some(buf.clone())));

    let res = buf.del_keymap(Mode::NormalVisualOperator, "a");
    assert_eq!(Ok(()), res);
}

#[nvim_oxi::test]
fn buf_set_get_del_lines() {
    let mut buf = Buffer::current();

    assert_eq!(Ok(()), buf.set_lines(.., true, ["foo", "bar", "baz"]));
    assert_eq!(
        vec!["foo", "bar", "baz"],
        buf.get_lines(.., true)
            .unwrap()
            .map(|s| s.to_string_lossy().into())
            .collect::<Vec<String>>()
    );
    assert_eq!(Ok(3), buf.line_count());

    assert_eq!(Ok(()), buf.set_lines::<&str, _, _>(.., true, []));
    assert_eq!(Ok(1), buf.line_count());
}

#[nvim_oxi::test]
fn buf_set_get_del_mark() {
    let mut buf = Buffer::current();
    let opts = SetMarkOpts::default();

    let res = buf.set_mark('a', 1, 0, &opts);
    assert_eq!(Ok(()), res);

    assert_eq!((1, 0), buf.get_mark('a').unwrap());

    let res = buf.del_mark('a');
    assert_eq!(Ok(()), res);
}

#[nvim_oxi::test]
fn buf_set_get_del_text() {
    let mut buf = Buffer::current();

    assert_eq!(Ok(()), buf.set_text(.., 0, 0, ["foo", "bar", "baz"]));
    assert_eq!(
        vec!["foo", "bar", "baz"],
        buf.get_text(.., 0, 3, &Default::default())
            .unwrap()
            .map(|s| s.to_string_lossy().into())
            .collect::<Vec<String>>()
    );
    assert_eq!(Ok(3), buf.line_count());

    assert_eq!(
        vec!["oo", "ba"],
        buf.get_text(..1, 1, 2, &Default::default())
            .unwrap()
            .map(|s| s.to_string_lossy().into())
            .collect::<Vec<String>>()
    );

    assert_eq!(Ok(()), buf.set_text::<&str, _, _>(.., 0, 3, []));

    assert_eq!(
        1,
        buf.get_text(.., 0, 1, &Default::default()).unwrap().count()
    );

    assert_eq!(Ok(1), buf.line_count());
}

#[nvim_oxi::test]
fn buf_set_get_del_var() {
    let mut buf = Buffer::current();
    buf.set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), buf.get_var("foo"));
    assert_eq!(Ok(()), buf.del_var("foo"));
}

#[nvim_oxi::test]
fn buf_set_get_name() {
    let mut buf = api::create_buf(true, false).unwrap();

    assert_eq!("", buf.get_name().unwrap().display().to_string());

    assert_eq!(Ok(()), buf.set_name("foo"));

    assert_eq!(
        "foo",
        buf.get_name().unwrap().file_name().unwrap().to_string_lossy()
    );

    assert_eq!(Ok(()), buf.set_name(""));
}

#[nvim_oxi::test]
fn buf_set_get_option() {
    let mut buf = Buffer::current();

    buf.set_option("modified", true).unwrap();
    assert!(buf.get_option::<bool>("modified").unwrap());

    buf.set_option("modified", false).unwrap();
    assert!(!buf.get_option::<bool>("modified").unwrap());
}

#[cfg_attr(target_os = "windows", ignore = "Windows' paths are dumb")]
#[nvim_oxi::test]
fn buf_terminal_name() {
    api::command("term").unwrap();

    let term_name_oxi = Buffer::current().get_name().unwrap();

    let term_name_lua =
        api::exec("lua =vim.api.nvim_buf_get_name(0)", true).unwrap().unwrap();

    assert_eq!(
        term_name_oxi.display().to_string(),
        term_name_lua.trim_matches('"')
    );
}

enum Range<T> {
    /// ..
    Full(ops::RangeFull),

    /// a..
    From(ops::RangeFrom<T>),

    /// ..b
    To(ops::RangeTo<T>),

    /// a..b
    FromTo(ops::Range<T>),

    /// ..=b
    ToInclusive(ops::RangeToInclusive<T>),

    /// a..=b
    Inclusive(ops::RangeInclusive<T>),
}

impl<T> Range<T> {
    fn new(range: impl Into<Self>) -> Self {
        range.into()
    }
}

impl<T> From<ops::RangeFull> for Range<T> {
    fn from(range: ops::RangeFull) -> Self {
        Range::Full(range)
    }
}

impl<T> From<ops::RangeFrom<T>> for Range<T> {
    fn from(range: ops::RangeFrom<T>) -> Self {
        Range::From(range)
    }
}

impl<T> From<ops::RangeTo<T>> for Range<T> {
    fn from(range: ops::RangeTo<T>) -> Self {
        Range::To(range)
    }
}

impl<T> From<ops::Range<T>> for Range<T> {
    fn from(range: ops::Range<T>) -> Self {
        Range::FromTo(range)
    }
}

impl<T> From<ops::RangeInclusive<T>> for Range<T> {
    fn from(range: ops::RangeInclusive<T>) -> Self {
        Range::Inclusive(range)
    }
}

impl<T> From<ops::RangeToInclusive<T>> for Range<T> {
    fn from(range: ops::RangeToInclusive<T>) -> Self {
        Range::ToInclusive(range)
    }
}

impl<T> ops::RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> ops::Bound<&T> {
        match self {
            Range::Full(range) => range.start_bound(),
            Range::From(range) => range.start_bound(),
            Range::To(range) => range.start_bound(),
            Range::FromTo(range) => range.start_bound(),
            Range::ToInclusive(range) => range.start_bound(),
            Range::Inclusive(range) => range.start_bound(),
        }
    }

    fn end_bound(&self) -> ops::Bound<&T> {
        match self {
            Range::Full(range) => range.end_bound(),
            Range::From(range) => range.end_bound(),
            Range::To(range) => range.end_bound(),
            Range::FromTo(range) => range.end_bound(),
            Range::ToInclusive(range) => range.end_bound(),
            Range::Inclusive(range) => range.end_bound(),
        }
    }
}
