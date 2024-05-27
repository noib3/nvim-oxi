use nvim_oxi::api::{self, opts::*, types::*, Window};
use nvim_oxi::{print, Dictionary, Function};

#[nvim_oxi::plugin]
fn api() -> nvim_oxi::Result<Dictionary> {
    // Create a new `Greetings` command.
    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("shows a greetings message")
        .nargs(CommandNArgs::ZeroOrOne)
        .build();

    let greetings = |args: CommandArgs| {
        let who = args.args.unwrap_or("from Rust".to_owned());
        let bang = if args.bang { "!" } else { "" };
        print!("Hello {}{}", who, bang);
    };

    api::create_user_command("Greetings", greetings, &opts)?;

    // Remaps `hi` to `hello` in insert mode.
    api::set_keymap(Mode::Insert, "hi", "hello", &Default::default())?;

    // Creates two functions `{open,close}_window` to open and close a
    // floating window.

    let buf = api::create_buf(false, true)?;

    use std::cell::RefCell;
    use std::rc::Rc;

    let win: Rc<RefCell<Option<Window>>> = Rc::default();

    let w = Rc::clone(&win);

    let open_window: Function<(), Result<(), api::Error>> =
        Function::from_fn(move |()| {
            if w.borrow().is_some() {
                api::err_writeln("Window is already open");
                return Ok(());
            }

            let config = WindowConfig::builder()
                .relative(WindowRelativeTo::Cursor)
                .height(5)
                .width(10)
                .row(1)
                .col(0)
                .build();

            let mut win = w.borrow_mut();
            *win = Some(api::open_win(&buf, false, &config)?);

            Ok(())
        });

    let close_window = Function::from_fn(move |()| {
        if win.borrow().is_none() {
            api::err_writeln("Window is already closed");
            return Ok(());
        }

        let win = win.borrow_mut().take().unwrap();
        win.close(false)
    });

    let api = Dictionary::from_iter([
        ("open_window", open_window),
        ("close_window", close_window),
    ]);

    Ok(api)
}
