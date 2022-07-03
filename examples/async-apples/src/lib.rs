use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use async_std::task;
use nvim_oxi::{self as nvim, print};
use rand::{thread_rng, Rng};

const MAX_WAIT: usize = 7;

const UPDATE_MSGS: &[&str] = &[
    "Started counting apples!",
    "Counted a few",
    "Still counting",
    "Not quite done yet",
    "Umhh, this might take a while",
    "Not sure if I'll finish in time",
    "Almost done",
];

#[nvim::module]
fn async_apples() -> nvim::Result<()> {
    let wait_time: usize = thread_rng().gen_range(4..10);

    let i = Rc::new(RefCell::new(0));
    let done = Rc::new(RefCell::new(false));

    let i_cl = Rc::clone(&i);
    let done_cl = Rc::clone(&done);
    let task = task::spawn_local(async move {
        let i = i_cl;
        while *i.try_borrow().unwrap() != wait_time {
            let mut j = i.try_borrow_mut().unwrap();
            print!("{}", UPDATE_MSGS[*j]);
            task::sleep(Duration::from_secs(1)).await;
            *j += 1;
        }
        let apples = thread_rng().gen_range(0..101);
        print!("Done! You have {apples} apples!");
        let mut done = done_cl.try_borrow_mut().unwrap();
        *done = true;
    });

    let _ = task::spawn_local(async move {
        let mut stopped = false;

        while !*done.try_borrow().unwrap() {
            if *i.try_borrow().unwrap() == MAX_WAIT {
                task.cancel().await;
                stopped = true;
                break;
            }
            task::sleep(Duration::from_secs(1)).await;
        }

        if stopped {
            print!("I've had enough of these damn apples!");
        }
    });

    // Great..except we have no executor to run these tasks on w/o blocking the
    // thread :/
    //
    // TODO: create something like a new `Task` object that takes an async
    // `FnMut(()) -> impl Future<Output = R> + 'static` and drives it to
    // completion using libuv.

    Ok(())
}
