use nvim_oxi::api::{self, opts::*, types::*};

#[nvim_oxi::test]
fn call_function() {
    let res = api::call_function::<_, usize>("strwidth", ("foo bar",));
    assert_eq!(Ok(7), res);
}

#[nvim_oxi::test(cmd = "set autoread")] // getting `W13` warnings otherwise
fn cmd_basic() {
    let cmd = "checktime";
    let infos = CmdInfos::builder().cmd(cmd).build();
    let opts = CmdOpts::builder().output(true).build();
    assert_eq!(Ok(None), api::cmd(&infos, &opts));
}

#[nvim_oxi::test]
fn cmd_no_output() {
    let cmd = "checktime";
    let infos = CmdInfos::builder().cmd(cmd).build();
    let opts = CmdOpts::builder().output(false).build();
    assert_eq!(Ok(None), api::cmd(&infos, &opts));
}

#[nvim_oxi::test]
fn command() {
    let res = api::command(":lua vim.api.nvim_buf_set_var(0, 'foo', 'bar')");
    assert_eq!(Ok(()), res);

    assert_eq!(
        Ok("bar".into()),
        api::get_current_buf().get_var::<String>("foo")
    );
}

#[nvim_oxi::test]
fn eval() {
    let res = api::eval::<u8>("41 + 1");
    assert_eq!(Ok(42), res);

    let res = api::eval::<u8>(&format!("{} * 2 - 15", res.unwrap()));
    assert_eq!(Ok(69), res); // nice
}

#[nvim_oxi::test]
fn exec() {
    let no_op = api::exec(":", true);
    assert_eq!(Ok(None), no_op);

    let add = api::exec(":echo 1 + 1", true);
    assert_eq!(Ok(Some("2".into())), add);
}

#[nvim_oxi::test]
fn parse_cmd_basic() {
    let res = api::parse_cmd("echo 'foo'", &Default::default());
    assert!(res.is_ok(), "{res:?}");

    let infos = res.unwrap();

    assert_eq!(None, infos.addr);
    assert_eq!(vec!["'foo'"], infos.args);
    assert_eq!(Some(false), infos.bang);
    assert_eq!(Some("echo".into()), infos.cmd);

    assert_eq!(Some(0), infos.count);

    let magic = infos.magic.unwrap();
    assert!(!magic.file);
    assert!(!magic.bar);

    let mods = infos.mods.unwrap();
    assert!(!mods.browse);
    assert!(!mods.confirm);
    assert!(!mods.emsg_silent);
    assert!(!mods.hide);
    assert!(!mods.keepalt);
    assert!(!mods.keepjumps);
    assert!(!mods.keepmarks);
    assert!(!mods.keeppatterns);
    assert!(!mods.lockmarks);
    assert!(!mods.noautocmd);
    assert!(!mods.sandbox);
    assert!(!mods.silent);
    assert_eq!(None, mods.split);
    assert_eq!(-1, mods.tab);
    assert!(!mods.vertical);

    assert_eq!(Some(CommandNArgs::Any), infos.nargs);
    assert_eq!(None, infos.nextcmd);
    assert_eq!(Some(CmdRange::None), infos.range);
}

#[nvim_oxi::test]
fn parse_expression_basic() {
    let res = api::parse_expression("lua print('a')", "", true);
    assert!(res.is_ok(), "{res:?}");

    // `ast` from
    //
    // ```
    // :lua =vim.api.nvim_parse_expression("lua print('a')", "", true)
    // ```
    //
    // is:
    //
    // ```lua
    // ast = {
    //   children = { {
    //     ident = "lua",
    //     len = 3,
    //     scope = 0,
    //     start = { 0, 0 },
    //     type = "PlainIdentifier"
    //   }, {
    //     children = { {
    //         ident = "print",
    //         len = 6,
    //         scope = 0,
    //         start = { 0, 3 },
    //         type = "PlainIdentifier"
    //       }, {
    //         len = 3,
    //         start = { 0, 10 },
    //         svalue = "a",
    //         type = "SingleQuotedString"
    //       } },
    //     len = 1,
    //     start = { 0, 9 },
    //     type = "Call"
    //   } },
    //   len = 0,
    //   start = { 0, 3 },
    //   type = "OpMissing"
    // }
    // ```

    let ParsedVimLExpression { ast, error, highlight, len, .. } = res.unwrap();

    let ast = ast.expect("ast is set");
    assert_eq!(2, ast.children.len());
    assert_eq!((0, 3), ast.start);
    assert_eq!(0, ast.len);
    assert_eq!(VimLAstNode::OpMissing, ast.ty);

    let mut iter = ast.children.into_iter();
    let leaf1 = iter.next().unwrap();
    let node = iter.next().unwrap();

    assert!(
        leaf1.children.is_empty(),
        "tree has {} elements",
        leaf1.children.len()
    );
    assert_eq!((0, 0), leaf1.start);
    assert_eq!(3, leaf1.len);
    assert_eq!(
        VimLAstNode::PlainIdentifier {
            ident: "lua".into(),
            scope: ExprVarScope::Missing
        },
        leaf1.ty
    );

    // BUG: why is it not deserializing the second leaf? Using a `Vec` instead
    // of a `BTreeSet` for the `children` field of `VimLExpressionAst` fixes
    // it.
    // assert_eq!(2, node.children.len()); // fails with `right = 1`
    assert_eq!((0, 9), node.start);
    assert_eq!(1, node.len);
    assert_eq!(VimLAstNode::Call, node.ty);

    let mut iter = node.children.into_iter();
    let leaf2 = iter.next().unwrap();
    // TODO: uncomment when bug gets resolved.
    // let leaf3 = iter.next().unwrap();

    assert!(
        leaf2.children.is_empty(),
        "tree has {} elements",
        leaf2.children.len()
    );
    assert_eq!((0, 3), leaf2.start);
    assert_eq!(6, leaf2.len);
    assert_eq!(
        VimLAstNode::PlainIdentifier {
            ident: "print".into(),
            scope: ExprVarScope::Missing
        },
        leaf2.ty
    );

    // TODO: uncomment when bug gets resolved.
    // assert!(
    //     leaf3.children.is_empty(),
    //     "tree has {} elements",
    //     leaf3.children.len()
    // );
    // assert_eq!((0, 10), leaf3.start);
    // assert_eq!(3, leaf3.len);
    // assert_eq!(VimLAstNode::SingleQuotedString("a".into()), leaf3.ty);

    let error = error.expect("error is set");
    assert_eq!("print('a')", error.arg);
    assert_eq!("E15: Missing operator: %.*s", error.message);

    assert_eq!(
        vec![
            (0, 0, 3, "NvimIdentifierName".into()),
            (0, 3, 4, "NvimInvalidSpacing".into()),
            (0, 4, 9, "NvimIdentifierName".into()),
            (0, 9, 10, "NvimCallingParenthesis".into()),
            (0, 10, 11, "NvimSingleQuote".into()),
            (0, 11, 12, "NvimSingleQuotedBody".into()),
            (0, 12, 13, "NvimSingleQuote".into()),
            (0, 13, 14, "NvimCallingParenthesis".into()),
        ],
        highlight
    );

    assert_eq!(14, len);
}
