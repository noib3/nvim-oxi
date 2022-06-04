use nvim_oxi::api::{opts::*, Buffer};

/// Tests ...
pub fn attach() {
    let buf = Buffer::current();

    let opts = BufAttachOpts::builder()
        .on_lines(|_args| Ok(false))
        .on_bytes(|_args| Ok(false))
        .on_detach(|_args| Ok(false))
        .on_reload(|_args| Ok(false))
        .on_changedtick(|_args| Ok(false))
        .build()
        .unwrap();

    let has_attached = buf.attach(false, opts).expect("attach failed");

    assert!(has_attached);
}

/// Tests ...
pub fn get_changedtick() {
    let buf = Buffer::current();

    assert!(buf.get_changedtick().is_ok());
}
