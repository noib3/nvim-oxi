//! `cargo test` will test and run all examples binaries in debug mode
//! `RELEASE=1 cargo test` will test but run all examples binaries in release mode
//! `cargo insta test --review` will update all the snapshots (except ones tagged with `#[ignore]`)
//! `cargo insta test --review -- test_fn_name` will update `test_fn_name` only

use insta::assert_snapshot as save;

// Empty vars
const EMPTY: &[(&str, &str)] = &[];

#[test]
fn calc() {
    let [_out, err] = run("./calc/run.sh", EMPTY).unwrap();
    save!(&err, @r###"
    Result: 
     add(-1, 128): 127 
     multiply(-1, 128): -128 
     compute(calc.multiply, 0, 128): 0 
    "###);
}

#[test]
fn mechanic() {
    let [_out, err] = run("./mechanic/run.sh", EMPTY).unwrap();
    save!(&err, @r###"
    Hands on the wheel!!
    {
      manufacturer = "Tesla",
      miles = 69420,
      works = true
    }
    "###);
}

// This test is failed.
#[test]
#[ignore]
fn api() {
    let [_out, err] = run("./api/run.sh", EMPTY).unwrap();
    save!(&err, @r###"
    thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: NvimError("replace_keycodes is not a boolean")', /rust/github/nvim-oxi/nvim-oxi/src/lua/lua.rs:45:12
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    fatal runtime error: Rust panics must be rethrown
    ../run.sh: line 14: 1621945 Aborted                 nvim -u NONE --headless +"$set_rtp" +":lua $lua" +quit
    "###);
}

// This test is failed and nvim will run forever.
#[test]
#[ignore]
fn mlua() {
    // The project name is `lua` which is not from the directory.
    let [(n1, ref n2), (l1, ref l2)] = var_name("lua");
    let [_out, err] = run("./mlua/run.sh", &[(n1, n2), (l1, l2)]).unwrap();
    save!(&err, @"");
}

// Run bash scripts to test in nvim
fn run(p: &str, envs: &[(&str, &str)]) -> std::io::Result<[String; 2]> {
    let path: &std::path::Path = p.as_ref();
    std::process::Command::new("/bin/bash")
        .envs(envs.iter().copied())
        .current_dir(path.parent().unwrap())
        .arg(path.file_name().unwrap())
        .output()
        .map(|o| {
            [
                String::from_utf8(o.stdout).unwrap(),
                String::from_utf8(o.stderr).unwrap(),
            ]
        })
}

// Get `$name` & `$name_lib` on specific platform
fn var_name(name: &str) -> [(&'static str, String); 2] {
    #[cfg(all(unix, not(target_os = "macos")))]
    let [name, name_lib] = [format!("{name}.so"), format!("lib{name}.so")];

    #[cfg(target_os = "macos")]
    let [name, name_lib] = [format!("{name}.so"), format!("lib{name}.dylib")];

    #[cfg(target_os = "windows")]
    compile_error!("Not supported on Windows");

    [("name", name), ("name_lib", name_lib)]
}
