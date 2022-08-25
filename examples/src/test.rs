//! `cargo r` will run this testing binary in debug mode but run all examples in debug mode
//! `RELEASE=1 cargo r` will run this testing binary in debug mode but run all examples in release mode

use insta::assert_snapshot as save;

#[test]
fn calc() {
    let [_out, err] = run("./calc/run.sh", &[]).unwrap();
    save!(&err, @r###"
    Result: 
     add(-1, 128): 127 
     multiply(-1, 128): -128 
     compute(calc.multiply, 0, 128): 0 
    "###);
}

#[test]
fn mechanic() {
    let [_out, err] = run("./mechanic/run.sh", &[]).unwrap();
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
    let [_out, err] = run("./api/run.sh", &[]).unwrap();
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
    let [_out, err] = run("./mlua/run.sh", &[]).unwrap();
    save!(&err, @"");
}

// run bash scripts to test in nvim
fn run(p: &str, envs: &[(&str, &str)]) -> std::io::Result<[String; 2]> {
    let path: &std::path::Path = p.as_ref();
    let mut cmd = std::process::Command::new("/bin/bash");

    // Clear env if `envs` are specified.
    if !envs.is_empty() {
        cmd.env_clear().envs(envs.iter().copied());
    }

    cmd.current_dir(path.parent().unwrap())
        .arg(path.file_name().unwrap())
        .output()
        .map(|o| {
            [
                String::from_utf8(o.stdout).unwrap(),
                String::from_utf8(o.stderr).unwrap(),
            ]
        })
}
