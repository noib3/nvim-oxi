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
        .events(vec![String::from("CompleteDone"), String::from("CursorHold")])
        .build();

    assert_eq!(Ok(()), api::clear_autocmds(opts));
}

pub fn clear_autocmds_buffer_n_patterns() {
    let opts = ClearAutocmdsOpts::builder()
        .buffer(0)
        .patterns(["*.py", "*.ts"])
        .build();

    assert!(api::clear_autocmds(opts).is_err());
    // assert_eq!()
}

pub fn set_del_augroup_by_id() {
    // // TODO
    // let id = todo!();
    // assert_eq!(Ok(()), api::del_augroup_by_id(id));
}

pub fn set_del_augroup_by_name() {
    // // TODO
    // let name = todo!();
    // assert_eq!(Ok(()), api::del_augroup_by_name(name));
}

pub fn set_del_autocmd() {
    // // TODO
    // let id = todo!();
    // assert_eq!(Ok(()), api::del_autocmd(id));
}
