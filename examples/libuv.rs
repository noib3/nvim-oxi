use std::thread;
use std::time::Duration;

use nvim_oxi::libuv::{AsyncHandle, TimerHandle};
use nvim_oxi::{print, schedule, Error, Result};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::time;

#[nvim_oxi::plugin]
fn libuv() -> Result<()> {
    // --
    let mut n = 0;

    let callback = move |timer: &mut TimerHandle| {
        if n <= 10 {
            let i = n;
            schedule(move |_| Ok(print!("Callback called {i} times")));
            n += 1;
        } else {
            timer.stop().unwrap();
        }
    };

    let _handle = TimerHandle::start(
        Duration::from_millis(0),
        Duration::from_secs(1),
        callback,
    );

    // --
    let msg = String::from("Hey there!");

    let _handle = TimerHandle::once(Duration::from_secs(2), move || {
        schedule(move |_| Ok(print!("{msg}")));
    });

    // --
    let (sender, mut receiver) = mpsc::unbounded_channel::<i32>();

    let handle = AsyncHandle::new(move || {
        let i = receiver.blocking_recv().unwrap();
        schedule(move |_| {
            print!("Received number {i} from backround thread");
        });
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
