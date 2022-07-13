use all_asserts::*;
use nvim_oxi::{
    self as oxi,
    api::{self, Buffer},
    opts::*,
};

#[oxi::test]
fn clear_autocmds_current_buf() {
    let opts = ClearAutocmdsOpts::builder().buffer(0).build();
    assert_eq!(Ok(()), api::clear_autocmds(&opts));

    let opts = ClearAutocmdsOpts::builder().buffer(Buffer::current()).build();
    assert_eq!(Ok(()), api::clear_autocmds(&opts));
}

#[oxi::test]
fn clear_autocmds_events() {
    let opts = ClearAutocmdsOpts::builder()
        .events(["BufFilePre", "BufFilePost"])
        .build();

    assert_eq!(Ok(()), api::clear_autocmds(&opts));

    let opts = ClearAutocmdsOpts::builder()
        .events(vec![
            String::from("CompleteDone").as_ref(),
            String::from("CursorHold").as_ref(),
        ])
        .build();

    assert_eq!(Ok(()), api::clear_autocmds(&opts));
}

#[oxi::test]
fn clear_autocmds_buffer_n_patterns() {
    let opts = ClearAutocmdsOpts::builder()
        .buffer(0)
        .patterns(["*.py", "*.ts"])
        .build();

    assert!(
        api::clear_autocmds(&opts).is_err(),
        "specifying both `buffer` and `patterns` shouldn't be allowed"
    );
}

#[oxi::test]
fn create_augroup() {
    let opts = CreateAugroupOpts::builder().build();
    let id = api::create_augroup("Foo", Some(&opts))
        .expect("create_augroup failed");

    let opts = CreateAugroupOpts::builder().clear(false).build();
    let got = api::create_augroup("Foo", Some(&opts));

    assert_eq!(Ok(id), got);
}

#[oxi::test]
fn create_autocmd() {
    let opts = CreateAutocmdOpts::builder()
        .buffer(0)
        .desc("Does nothing, in the current buffer")
        .callback(|_args| Ok(false))
        .build();

    let id = api::create_autocmd(["VimEnter"], &opts);
    assert!(id.is_ok(), "{id:?}");
}

#[oxi::test]
fn create_autocmd_buffer_n_patterns() {
    let opts = CreateAutocmdOpts::builder()
        .command("echo 'hi there'")
        .buffer(0)
        .patterns(["*.py", "*.ts"])
        .build();

    let id = api::create_autocmd(["VimEnter"], &opts);
    assert!(id.is_err(), "{id:?}");
}

#[oxi::test]
fn exec_autocmds() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let i = Rc::new(RefCell::new(0));

    let cloned = Rc::clone(&i);

    let opts = CreateAutocmdOpts::builder()
        .callback(move |_args| {
            let mut i = cloned.borrow_mut();
            *i += 1;
            Ok(false)
        })
        .buffer(0)
        .once(true)
        .build();

    let id = api::create_autocmd(["BufAdd"], &opts);
    assert!(id.is_ok(), "{id:?}");

    let opts = ExecAutocmdsOpts::builder().buffer(0).build();

    let res = api::exec_autocmds(["BufAdd"], &opts);
    assert_eq!(Ok(()), res);
    assert_eq!(1, *i.try_borrow().unwrap());

    let res = api::exec_autocmds(["BufAdd"], &opts);
    assert_eq!(Ok(()), res);
    // `i` should still be equal to 1 since `once` was set to `true`.
    assert_eq!(1, *i.try_borrow().unwrap());
}

#[oxi::test]
fn get_autocmds() {
    let autocmds = api::get_autocmds(None).expect("couldn't get autocmds");
    assert_lt!(0, autocmds.collect::<Vec<_>>().len());
}

#[oxi::test]
fn set_del_augroup_by_id() {
    let id = api::create_augroup("Foo", None).expect("create_augroup failed");
    assert_eq!(Ok(()), api::del_augroup_by_id(id));
}

#[oxi::test]
fn set_del_augroup_by_name() {
    let _ = api::create_augroup("Foo", None).expect("create_augroup failed");
    assert_eq!(Ok(()), api::del_augroup_by_name("Foo"));
}

#[oxi::test]
fn set_exec_del_autocmd() {
    let opts =
        CreateAutocmdOpts::builder().callback(|_args| Ok(false)).build();

    let id = api::create_autocmd(["BufAdd, BufDelete"], &opts)
        .expect("create_autocmd failed");

    let opts = ExecAutocmdsOpts::builder().build();
    assert_eq!(Ok(()), api::exec_autocmds(["BufAdd"], &opts));

    assert_eq!(Ok(()), api::del_autocmd(id));
}
