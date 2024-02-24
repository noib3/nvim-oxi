use std::convert::Infallible;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

use nvim_oxi::{self as nvim, libuv::*};

#[nvim::test]
fn timer_handle_0() {
    let (tx, rx) = mpsc::channel();

    let timeout = Duration::from_millis(100);

    let repeat = Duration::from_millis(100);

    let mut handle = TimerHandle::start(timeout, repeat, move |_| {
        tx.send(()).unwrap();
        Ok::<_, Infallible>(())
    })
    .unwrap();

    // TODO: how do sleep without blocking the main thread?

    // sleep(timeout / 2);
    //
    // assert_eq!(rx.try_recv().unwrap(), ());
    //
    // sleep(timeout / 2);
    //
    // for _ in 0..10 {
    //     assert_eq!(rx.try_recv().unwrap(), ());
    //     assert_eq!(rx.try_recv().unwrap_err(), mpsc::TryRecvError::Empty);
    //     sleep(repeat);
    // }

    let _ = rx.try_recv();

    handle.stop().unwrap();

    sleep(repeat * 2);

    assert_eq!(rx.try_recv().unwrap_err(), mpsc::TryRecvError::Empty);
}
