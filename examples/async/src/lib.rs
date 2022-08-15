use std::thread;
use std::time::Duration;

use nvim_oxi as oxi;
use nvim_oxi::r#loop::{self, AsyncHandle};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::time;

#[oxi::module]
fn async_print() -> oxi::Result<()> {
    let (sender, mut recv) = mpsc::unbounded_channel::<i32>();

    let handle = r#loop::new_async(move || {
        let i = recv.blocking_recv().unwrap();
        oxi::print!("Bonsoir {i}!");
        Ok(())
    })?;

    let _ = thread::spawn(move || hello(handle, sender));

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn hello(mut handle: AsyncHandle, sender: UnboundedSender<i32>) {
    let mut i = -1;

    loop {
        i += 1;
        sender.send(i).unwrap();
        let _ = handle.send();
        time::sleep(Duration::from_secs(1)).await;
    }
}
