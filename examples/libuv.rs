use std::thread;
use std::time::Duration;

use nvim_oxi as oxi;
use oxi::libuv::{AsyncHandle, TimerHandle};
use oxi::print;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::time;

#[oxi::module]
fn libuv() -> oxi::Result<()> {
    // --
    let mut n = 0;

    let callback = move |timer: &mut TimerHandle| {
        if n <= 10 {
            let i = n;
            oxi::schedule(move |_| Ok(print!("Callback called {i} times")));
            n += 1;
        } else {
            timer.stop().unwrap();
        }

        Ok::<_, oxi::Error>(())
    };

    let _handle = TimerHandle::start(
        Duration::from_millis(0),
        Duration::from_secs(1),
        callback,
    );

    // --
    let msg = String::from("Hey there!");

    let _handle = TimerHandle::once(Duration::from_secs(2), move || {
        oxi::schedule(move |_| Ok(print!("{msg}")));
        Ok::<_, oxi::Error>(())
    });

    // --
    let (sender, mut receiver) = mpsc::unbounded_channel::<i32>();

    let handle = AsyncHandle::new(move || {
        let i = receiver.blocking_recv().unwrap();
        oxi::schedule(move |_| {
            print!("Received number {i} from backround thread");
            Ok(())
        });
        Ok::<_, oxi::Error>(())
    })?;

    let _ = thread::spawn(move || send_numbers(handle, sender));

    Ok(())
}

#[tokio::main]
async fn send_numbers(handle: AsyncHandle, sender: UnboundedSender<i32>) {
    let mut i = 0;

    loop {
        sender.send(i).unwrap();
        handle.send().unwrap();
        i += 1;

        time::sleep(Duration::from_secs(1)).await;
    }
}
