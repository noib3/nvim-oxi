use std::cell::RefCell;
use std::rc::Rc;

use nvim_oxi::libuv::*;

#[nvim_oxi::test]
#[ignore = "the callback is never called"]
fn async_handle_0() {
    let num_called = Rc::new(RefCell::new(0));

    let also_num_called = num_called.clone();

    let handle = AsyncHandle::new(move || {
        *also_num_called.borrow_mut() += 1;
    })
    .unwrap();

    assert_eq!(*num_called.borrow(), 0);

    for i in 1..10 {
        handle.send().unwrap();

        let also_num_called = num_called.clone();

        // TODO: how do we wait for the callback to be executed without
        // blocking the main thread?
        nvim_oxi::schedule(move |_| {
            assert_eq!(*also_num_called.borrow(), i);
        });
    }
}
