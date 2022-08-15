use std::alloc::{self, Layout};
use std::cmp::Ordering;

use libuv_sys2::{
    uv_async_init,
    uv_async_send,
    uv_async_t,
    uv_close,
    uv_handle_get_data,
    uv_handle_set_data,
};

#[allow(unused_imports)]
use crate::lua;

#[derive(Clone)]
pub struct AsyncHandle {
    handle: *mut uv_async_t,
}

impl AsyncHandle {
    #[inline(always)]
    pub fn send(&mut self) -> super::Result<()> {
        let ret = unsafe { uv_async_send(self.handle) };

        match ret.cmp(&0) {
            Ordering::Less => Err(super::Error::CouldntTriggerAsyncHandle),
            _ => Ok(()),
        }
    }
}

unsafe impl Send for AsyncHandle {}
unsafe impl Sync for AsyncHandle {}

// unsafe extern "C" fn close_cb(handle: *mut uv_handle_t) {}

impl Drop for AsyncHandle {
    fn drop(&mut self) {
        unsafe { uv_close(self.handle as _, None) };

        if !self.handle.is_null() {
            let layout = Layout::new::<uv_async_t>();
            unsafe { std::alloc::dealloc(self.handle as _, layout) };
        }
    }
}

type Cb = Box<dyn FnMut() -> crate::Result<()> + 'static>;

unsafe extern "C" fn async_cb(handle: *mut uv_async_t) {
    let cb_ptr = uv_handle_get_data(handle as _) as *mut Cb;

    // TODO: use let chains once they are stable (1.65?).
    if !cb_ptr.is_null() {
        if let Err(_err) = (&mut *cb_ptr)() {
            // TODO: how to handle errors?
            //
            // lua::with_state(|lstate| lua::handle_error(lstate, err.into()))
        }
    }
}

pub fn new_async<F>(fun: F) -> super::Result<AsyncHandle>
where
    F: FnMut() -> crate::Result<()> + 'static,
{
    let layout = Layout::new::<uv_async_t>();
    let handle = unsafe { alloc::alloc(layout) as *mut uv_async_t };

    let cb_ptr = Box::into_raw(Box::new(Box::new(fun) as Cb));

    unsafe { uv_handle_set_data(handle as _, cb_ptr as _) };

    let ret = unsafe {
        super::with_loop(|main_loop| {
            uv_async_init(main_loop, handle, Some(async_cb as _))
        })
    };

    match ret.cmp(&0) {
        // A negative return value indicates an error.
        Ordering::Less => {
            unsafe { alloc::dealloc(handle as _, layout) };
            Err(super::Error::CouldntCreateAsyncHandle)
        },

        _ => Ok(AsyncHandle { handle }),
    }
}
