use std::sync::Arc;

use all_asserts::*;
use nvim_oxi::api::{self, opts::*, types::*, Buffer, Window};
use nvim_oxi::mlua::{Error as LuaError, IntoLuaMulti, Lua, Table};
use nvim_oxi::{Dictionary, Object};

#[nvim_oxi::test]
fn chan_send_fail() {
    let res = api::chan_send(42, "hello there");
    assert!(res.is_err());
}

#[nvim_oxi::test]
fn create_del_user_command() {
    let res = api::create_user_command("Foo", ":", &Default::default());
    assert_eq!(Ok(()), res);
    api::command("Foo").unwrap();

    let res = api::create_user_command("Bar", |_args| (), &Default::default());
    assert_eq!(Ok(()), res);
    api::command("Bar").unwrap();

    let commands =
        api::get_commands(&Default::default()).unwrap().collect::<Vec<_>>();

    assert!(commands.iter().any(|cmd| cmd.name == "Foo"));
    assert!(commands.iter().any(|cmd| cmd.name == "Bar"));

    assert_eq!(Ok(()), api::del_user_command("Foo"));
    assert_eq!(Ok(()), api::del_user_command("Bar"));
}

#[nvim_oxi::test]
fn echo() {
    api::echo(
        [("Hello ", None), ("World", Some("WarningMsg"))],
        true,
        &Default::default(),
    )
    .unwrap();
}

#[nvim_oxi::test]
fn eval_statusline() {
    let opts = EvalStatuslineOpts::builder().highlights(true).build();
    let res = api::eval_statusline("foo", &opts);
    assert_eq!(Ok("foo".into()), res.map(|infos| infos.str));
}

#[nvim_oxi::test]
fn feedkeys() {
    let keys = "iHllo<Esc>bi<Right>e";
    let keys = api::replace_termcodes(keys, true, false, true);
    api::feedkeys(&keys, c"x", false);

    let lines = api::Buffer::current()
        .get_lines(0..1, true)
        .unwrap()
        .collect::<Vec<_>>();

    assert_eq!(lines, ["Hello"]);
}

#[nvim_oxi::test]
fn get_chan_info() {
    let res = api::get_chan_info(0);
    assert!(res.is_err());
}

#[nvim_oxi::test]
fn get_colors() {
    let colors = api::get_color_map().collect::<Vec<_>>();
    assert_lt!(0, colors.len());

    let (name, color) = colors.into_iter().next().unwrap();
    assert_eq!(color, api::get_color_by_name(&name).unwrap());
}

#[nvim_oxi::test]
fn get_context() {
    let res = api::get_context(&Default::default());
    assert!(res.is_ok());
}

#[nvim_oxi::test]
fn get_highlights() {
    let (name, _) = api::get_color_map().next().unwrap();
    let id = api::get_hl_id_by_name(&name).unwrap();
    assert_eq!(api::get_hl_by_id(id, true), api::get_hl_by_name(&name, true));
}

#[nvim_oxi::test]
fn get_hl() {
    let infos = api::get_hl(0, &Default::default()).unwrap();
    let GetHlInfos::Map(map_iter) = infos else { panic!("expected a map") };
    assert!(!map_iter.collect::<Vec<_>>().is_empty());

    let opts = GetHighlightOpts::builder().name("Normal").build();
    let infos = api::get_hl(0, &opts).unwrap();
    let GetHlInfos::Single(_) = infos else { panic!("expected a single") };
}

#[nvim_oxi::test]
fn get_mode() {
    let GotMode { blocking, mode, .. } = api::get_mode().unwrap();
    assert_eq!(mode, "n");
    assert!(!blocking);
}

#[nvim_oxi::test]
fn get_options() {
    let res = api::get_all_options_info();
    assert_lt!(0, res.unwrap().collect::<Vec<_>>().len());
}

#[nvim_oxi::test]
fn get_option_info() {
    let opts =
        OptionOpts::builder().scope(api::opts::OptionScope::Global).build();
    api::set_option_value("number", true, &opts).unwrap();
    assert!(api::get_option_info("number").is_ok());
}

#[nvim_oxi::test]
fn get_runtime_file() {
    assert!(api::get_runtime_file("*", true).unwrap().next().is_some());
}

#[nvim_oxi::test]
fn hl_foreground() {
    let foreground = "#FF0000";
    let opts = SetHighlightOpts::builder()
        .foreground(foreground)
        .strikethrough(true)
        .bold(true)
        .build();
    api::set_hl(0, "Header", &opts).unwrap();

    let infos = api::get_hl_by_name("Header", true).unwrap();
    assert_eq!(infos.foreground, Some(hex_to_dec(foreground)));
}

#[nvim_oxi::test]
fn hl_link() {
    let base_fg = "#579dd6";
    let base_opts = SetHighlightOpts::builder().foreground(base_fg).build();
    nvim_oxi::api::set_hl(0, "Base", &base_opts).unwrap();

    let linked_opts = SetHighlightOpts::builder().link("Base").build();
    nvim_oxi::api::set_hl(0, "Linked", &linked_opts).unwrap();

    let infos = api::get_hl_by_name("Linked", true).unwrap();
    assert_eq!(infos.foreground, Some(hex_to_dec(base_fg)));
}

#[nvim_oxi::test]
fn hl_underline() {
    let opts = SetHighlightOpts::builder().underline(true).build();
    api::set_hl(0, "MatchParen", &opts).unwrap();

    let infos = api::get_hl_by_name("MatchParen", true).unwrap();
    assert_eq!(Some(true), infos.underline);
}

#[nvim_oxi::test]
fn list_bufs() {
    let _ = api::create_buf(true, false);
    let _ = api::create_buf(true, false);

    let bufs = api::list_bufs().collect::<Vec<_>>();

    assert_eq!(3, bufs.len());
    assert_eq!(vec![Buffer::from(1), Buffer::from(2), Buffer::from(3)], bufs);
}

#[nvim_oxi::test]
fn list_runtime_paths() {
    assert!(api::list_runtime_paths().unwrap().next().is_some());
}

#[nvim_oxi::test]
fn list_wins() {
    api::command("vsp").unwrap();
    api::command("vsp").unwrap();

    let wins = api::list_wins().collect::<Vec<_>>();

    assert_eq!(3, wins.len());
    assert_eq!(
        vec![Window::from(1002), Window::from(1001), Window::from(1000)],
        wins
    );
}

#[nvim_oxi::test]
fn notify() {
    let opts = Dictionary::new();
    let ret = api::notify("", LogLevel::Error, &opts).unwrap();
    assert_eq!(ret, Object::nil());
}

#[nvim_oxi::test]
fn notify_custom() {
    let message = "Notifier was called!";

    // Set up a custom notification provider.
    set_notification_provider(move |lua, _msg, _level, _opts| {
        lua.create_string(message)
    });

    let opts = Dictionary::new();
    let ret = api::notify("", LogLevel::Error, &opts).unwrap();
    assert_eq!(ret, message.into());
}

#[nvim_oxi::test]
fn notify_custom_err() {
    #[derive(Debug, thiserror::Error)]
    #[error("")]
    struct CustomError;

    // Set up a custom notification provider.
    set_notification_provider(move |_lua, _msg, _level, _opts| {
        Err::<(), _>(LuaError::ExternalError(Arc::new(CustomError)))
    });

    let opts = Dictionary::new();
    let _err = api::notify("", LogLevel::Error, &opts).unwrap_err();
}

#[nvim_oxi::test]
fn set_get_del_current_line() {
    let res = api::set_current_line("foo");
    assert_eq!(Ok(()), res);

    let res = api::get_current_line();
    assert_eq!(Ok("foo".into()), res);

    let res = api::del_current_line();
    assert_eq!(Ok(()), res);
}

#[nvim_oxi::test]
fn set_get_del_keymap() {
    let opts = SetKeymapOpts::builder()
        .callback(|_| ())
        .desc("does nothing")
        .expr(true)
        .build();

    let res = api::set_keymap(Mode::Insert, "a", "", &opts);
    assert_eq!(Ok(()), res);

    let keymaps = api::get_keymap(Mode::Insert).collect::<Vec<_>>();
    assert_le!(1, keymaps.len());

    let res = api::del_keymap(Mode::Insert, "a");
    assert_eq!(Ok(()), res);
}

#[nvim_oxi::test]
fn set_get_del_mark() {
    let mut buf = api::create_buf(true, false).unwrap();

    let opts = SetMarkOpts::default();

    let res = buf.set_mark('A', 1, 0, &opts);
    assert_eq!(Ok(()), res);

    assert_eq!(
        (1, 0, buf, "".into()),
        api::get_mark('A', &Default::default()).unwrap()
    );

    let res = api::del_mark('A');
    assert_eq!(Ok(()), res);
}

#[nvim_oxi::test]
fn set_get_del_var() {
    api::set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), api::get_var("foo"));
    assert_eq!(Ok(()), api::del_var("foo"));
}

// `api::{get,set}_option()` were deprecated on 0.11, so only test on 0.10.
#[cfg(not(feature = "neovim-nightly"))]
#[nvim_oxi::test]
fn set_get_option() {
    api::set_option("modified", true).unwrap();
    assert!(api::get_option::<bool>("modified").unwrap());

    api::set_option("modified", false).unwrap();
    assert!(!api::get_option::<bool>("modified").unwrap());
}

#[nvim_oxi::test]
fn set_get_option_value() {
    let opts =
        OptionOpts::builder().scope(api::opts::OptionScope::Global).build();
    api::set_option_value("modified", true, &opts).unwrap();
    assert!(api::get_option_value::<bool>("modified", &opts).unwrap());
}

#[nvim_oxi::test]
fn strwidth() {
    assert_eq!(Ok(2), api::strwidth("｜"));
}

#[nvim_oxi::test]
fn user_command_with_count() {
    let opts = CreateCommandOpts::builder().count(32).build();
    api::create_user_command("Foo", "echo 'foo'", &opts).unwrap();

    let res = api::get_commands(&Default::default())
        .map(|cmds| cmds.collect::<Vec<_>>());

    assert!(res.is_ok(), "{res:?}");
}

fn hex_to_dec(hex_color: &str) -> u32 {
    assert!(hex_color.starts_with('#'));
    assert!(hex_color[1..].chars().all(|c| c.is_ascii_digit()
        || ('a'..='f').contains(&c.to_ascii_lowercase())));
    u32::from_str_radix(&hex_color[1..], 16).unwrap()
}

fn set_notification_provider<P, R>(mut provider: P)
where
    P: FnMut(&Lua, String, u32, Table) -> Result<R, LuaError> + 'static,
    R: IntoLuaMulti,
{
    let lua = nvim_oxi::mlua::lua();
    let vim = lua.globals().get::<Table>("vim").unwrap();
    let notify = lua
        .create_function_mut(move |lua, (msg, level, opts)| {
            provider(lua, msg, level, opts)
        })
        .unwrap();
    vim.set("notify", notify).unwrap();
}
