use nvim_oxi::api::{opts::*, Buffer};

// #[test]
// fn buf_attach() {
//     let buf = Buffer::current();

//     let opts = BufAttachOpts::builder()
//         .on_lines(|_args| Ok(false))
//         .on_bytes(|_args| Ok(false))
//         .on_detach(|_args| Ok(false))
//         .on_reload(|_args| Ok(false))
//         .on_changedtick(|_args| Ok(false))
//         .build();

//     let has_attached = buf.attach(false, opts).expect("attach failed");

//     assert!(has_attached);
// }

#[test]
fn get_changedtick() {
    let out = ::std::process::Command::new("nvim")
        .args(["-u", "NONE", "--headless"])
        .args(["-c", "set rtp+=/Users/noib3/Dropbox/projects/nvim-oxi"])
        .args(["-c", "lua require(\"liboxi_tests\")"])
        .args(["+quit"])
        .output()
        .expect("Couldn't find `nvim` binary in $PATH!");

    let stderr = String::from_utf8_lossy(&out.stderr);

    assert_eq!(stderr, String::new());
}

#[no_mangle]
unsafe extern "C" fn luaopen_liboxi_tests(
    state: *mut ::nvim_oxi::lua::lua_State,
) -> ::std::os::raw::c_int {
    ::nvim_oxi::lua::init_state(state);

    let result = ::std::panic::catch_unwind(|| {
        let buf = Buffer::current();
        assert!(buf.get_changedtick().is_err());
    });

    ::std::process::exit(match result {
        Ok(_) => 0,

        Err(err) => {
            eprintln!("{err:?}");
            1
        },
    })
}
