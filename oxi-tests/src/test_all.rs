use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(all(unix, not(target_os = "macos")))]
const LIB_COMPILED: &str = "liboxi_tests.so";

#[cfg(target_os = "macos")]
const LIB_COMPILED: &str = "liboxi_tests.dylib";

#[cfg(target_os = "windows")]
const LIB_COMPILED: &str = "oxi_tests.dll";

#[cfg(target_family = "unix")]
const LIB_LUA: &str = "liboxi_tests.so";

#[cfg(target_family = "windows")]
const LIB_LUA: &str = "liboxi_tests.dll";

fn setup(root: &Path) {
    let lib_name = PathBuf::from(LIB_COMPILED);
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
    let to = lua_dir.join(LIB_LUA);
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
