use std::cell::RefCell;
use std::convert::Infallible;
use std::rc::Rc;

use nvim_oxi::{self as nvim, libuv::*};

#[nvim::test]
fn async_handle_0() {
    let num_called = Rc::new(RefCell::new(0));

    let also_num_called = num_called.clone();

    let handle = AsyncHandle::new(move || {
        *also_num_called.borrow_mut() += 1;
        Ok::<_, Infallible>(())
    })
    .unwrap();

    assert_eq!(*num_called.borrow(), 0);

    for i in 1..10 {
        handle.send().unwrap();

        let also_num_called = num_called.clone();

        // This is not actually executing, we can panic inside the callback and
        // the test will still pass.
        //
        // TODO: how do we wait for the callback to be executed without
        // blocking the main thread?
        nvim::schedule(move |_| {
            assert_eq!(*also_num_called.borrow(), i);
            Ok(())
        });
    }
}
