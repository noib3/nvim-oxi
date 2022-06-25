use nvim_oxi::api::opts::*;
use nvim_oxi::api::{self, Buffer};
// use nvim_oxi::opts::*;

pub fn clear_autocmds_current_buf() {
    let opts = ClearAutocmdsOpts::builder().buffer(0).build();
    assert_eq!(Ok(()), api::clear_autocmds(opts));

    let opts = ClearAutocmdsOpts::builder().buffer(Buffer::current()).build();
    assert_eq!(Ok(()), api::clear_autocmds(opts));
}

pub fn clear_autocmds_events() {
    let opts = ClearAutocmdsOpts::builder()
        .events(["BufFilePre", "BufFilePost"])
        .build();

    assert_eq!(Ok(()), api::clear_autocmds(opts));

    let opts = ClearAutocmdsOpts::builder()
        .events(vec![
            String::from("CompleteDone").as_ref(),
            String::from("CursorHold").as_ref(),
        ])
        .build();

    assert_eq!(Ok(()), api::clear_autocmds(opts));
}

pub fn clear_autocmds_buffer_n_patterns() {
    let opts = ClearAutocmdsOpts::builder()
        .buffer(0)
        .patterns(["*.py", "*.ts"])
        .build();

    assert!(
        api::clear_autocmds(opts).is_err(),
        "specifying both `buffer` and `patterns` shouldn't be allowed"
    );
}

pub fn create_augroup() {
    let opts = CreateAugroupOpts::builder().build();
    let id = api::create_augroup("Foo", opts).expect("create_augroup failed");

    let opts = CreateAugroupOpts::builder().clear(false).build();
    let got = api::create_augroup("Foo", opts);

    assert_eq!(Ok(id), got);
}

pub fn create_autocmd() {
    let opts = CreateAutocmdOpts::builder()
        .buffer(0)
        .desc("Does nothing, in the current buffer")
        .callback(|_args| Ok(false))
        .build();

    let id = api::create_autocmd(["VimEnter"], opts);
    assert!(id.is_ok(), "{id:?}");
}

pub fn create_autocmd_buffer_n_patterns() {
    let opts = CreateAutocmdOpts::builder()
        .command("echo 'hi there'")
        .buffer(0)
        .patterns(["*.py", "*.ts"])
        .build();

    let id = api::create_autocmd(["VimEnter"], opts);
    assert!(id.is_err(), "{id:?}");
}

pub fn set_del_augroup_by_id() {
    let opts = CreateAugroupOpts::builder().build();
    let id = api::create_augroup("Foo", opts).expect("create_augroup failed");
    assert_eq!(Ok(()), api::del_augroup_by_id(id));
}

pub fn set_del_augroup_by_name() {
    let opts = CreateAugroupOpts::builder().build();
    let _ = api::create_augroup("Foo", opts).expect("create_augroup failed");
    assert_eq!(Ok(()), api::del_augroup_by_name("Foo"));
}

pub fn set_exec_del_autocmd() {
    let opts =
        CreateAutocmdOpts::builder().callback(|_args| Ok(false)).build();

    let id = api::create_autocmd(["BufAdd, BufDelete"], opts)
        .expect("create_autocmd failed");

    // TODO: exec autocmd

    assert_eq!(Ok(()), api::del_autocmd(id));
}
