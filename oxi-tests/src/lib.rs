mod api;

#[no_mangle]
unsafe extern "C" fn luaopen_liboxi_tests(
    state: *mut ::nvim_oxi::lua::lua_State,
) -> std::os::raw::c_int {
    ::nvim_oxi::lua::init_state(state);

    let result = ::std::panic::catch_unwind(|| {
        api::buffer::attach();
        api::buffer::get_changedtick();
    });

    std::process::exit(match result {
        Ok(_) => 0,

        Err(err) => {
            eprintln!("{err:?}");
            1
        },
    })
}

// #[test]
// fn get_changedtick() {
//     let out = ::std::process::Command::new("nvim")
//         .args(["-u", "NONE", "--headless"])
//         .args(["-c", "set rtp+=/Users/noib3/Dropbox/projects/nvim-oxi"])
//         .args(["-c", "lua require(\"liboxi_tests\")"])
//         .args(["+quit"])
//         .output()
//         .expect("Couldn't find `nvim` binary in $PATH!");

//     let stderr = String::from_utf8_lossy(&out.stderr);

//     assert_eq!(stderr, String::new());
// }

// #[nvim::module]
// fn buffer() -> nvim::Result<()> {
//     let result = ::std::panic::catch_unwind(|| {
//         attach();
//         get_changedtick();
//     });

//     std::process::exit(match result {
//         Ok(_) => 0,

//         Err(err) => {
//             eprintln!("{err:?}");
//             1
//         },
//     })
// }
