use std::time::Duration;

use nvim_oxi as oxi;
use oxi::libuv::TimerHandle;
use oxi::print;

#[oxi::module]
fn libuv() -> oxi::Result<()> {
    let mut n = 0;

    let callback = move |timer: &mut TimerHandle| {
        let i = n;

        if i <= 10 {
            oxi::schedule(move |_| Ok(print!("Callback called {i} times")));
        } else {
            timer.stop();
        }

        n += 1;
        Ok::<_, oxi::Error>(())
    };

    let _handle = TimerHandle::start(
        &Duration::from_millis(0),
        &Duration::from_secs(1),
        callback,
    );

    let msg = String::from("Hey there!");

    let _handle = TimerHandle::once(&Duration::from_secs(2), move || {
        oxi::schedule(move |_| Ok(print!("{msg}")));
        Ok::<_, oxi::Error>(())
    });

    Ok(())
}
