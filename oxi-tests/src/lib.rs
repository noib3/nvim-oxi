#[cfg(test)]
use std::path::{Path, PathBuf};
#[cfg(test)]
use std::process::Command;
use std::{panic, process};

use nvim_oxi as nvim;

mod api;

#[cfg(test)]
#[cfg(all(unix, not(target_os = "macos")))]
const LIB_EXTENSION: &str = "so";

#[cfg(test)]
#[cfg(target_os = "macos")]
const LIB_EXTENSION: &str = "dylib";

#[cfg(test)]
#[cfg(target_os = "windows")]
const LIB_EXTENSION: &str = "dll";

#[cfg(test)]
fn setup(root: &Path) {
    let lib_name = PathBuf::from("liboxi_tests").with_extension(LIB_EXTENSION);
    let lib_path = root.join("target").join("debug").join(&lib_name);

    if !lib_path.exists() {
        panic!(
            "Missing library at '{}', run `cargo build` first",
            lib_path.display()
        );
    }

    let lua_dir = root.join("lua");

    std::fs::create_dir_all(&lua_dir)
        .expect(&format!("Couldn't create '{}'", lua_dir.display()));

    let from = lib_path;
    let to = lua_dir.join(lib_name).with_extension("so");
    std::fs::copy(&from, &to).expect(&format!(
        "Couldn't copy {} to {}",
        from.display(),
        to.display(),
    ));
}

#[test]
fn test_all() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    setup(&root);

    let out = Command::new("nvim")
        .args(["-u", "NONE", "--headless"])
        .args(["-c", &format!("set rtp+={}", root.display())])
        .args(["-c", "lua require(\"liboxi_tests\")"])
        .args(["+quit"])
        .output()
        .expect("Couldn't find `nvim` binary in $PATH!");

    let stderr = String::from_utf8_lossy(&out.stderr);

    assert_eq!(stderr, String::new());
}

#[nvim::module]
fn liboxi_tests() -> nvim::Result<()> {
    let result = panic::catch_unwind(|| {
        api::buffer::attach();
        api::buffer::call();
        api::buffer::get_changedtick();
        api::buffer::set_lines();
        api::buffer::set_option();
        api::buffer::set_var();
    });

    process::exit(match result {
        Ok(_) => 0,

        Err(err) => {
            eprintln!("{err:?}");
            1
        },
    })
}
