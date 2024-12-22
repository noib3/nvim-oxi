use nvim_oxi::api::{self, opts::*, types::*};

#[nvim_oxi::test]
fn command_nargs_0() {
    let opts = CreateCommandOpts::builder().nargs(CommandNArgs::Zero).build();
    api::create_user_command("Foo", ":", &opts).unwrap();
    assert_eq!(api::command("Foo"), Ok(()));
    let err = api::command("Foo foo");
    assert!(err.is_err(), "expected an error when passing an argument");
}

#[nvim_oxi::test]
fn command_nargs_1() {
    let opts = CreateCommandOpts::builder().nargs(CommandNArgs::One).build();
    api::create_user_command("Foo", ":", &opts).unwrap();
    let err = api::command("Foo");
    assert!(err.is_err(), "expected an error when passing 0 arguments");
    assert_eq!(api::command("Foo foo"), Ok(()));
}

#[nvim_oxi::test]
fn regression_1() {
    let opts = CreateCommandOpts::builder()
        .bang(true)
        .nargs(CommandNArgs::ZeroOrOne)
        .build();
    api::create_user_command("Echo", "", &opts).unwrap();
}
