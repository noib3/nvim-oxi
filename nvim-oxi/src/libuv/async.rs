use std::alloc::{self, Layout};
use std::os::raw::c_void;
use std::ptr;

use libuv_sys2 as libuv;

type Cb = Box<dyn FnMut() -> crate::Result<()> + 'static>;

/// Callback registered w/ `libuv::uv_async_init` and triggered when
/// [`AsyncHandle::send`] is called.
unsafe extern "C" fn async_cb(handle: *mut libuv::uv_async_t) {
    let callback_ptr = libuv::uv_handle_get_data(handle as _) as *mut Cb;

    if callback_ptr.is_null() {
        return;
    }

    if let Err(_err) = (&mut *callback_ptr)() {
        // TODO: how to handle errors?
    }
}

/// Registers a new callback on the Neovim event loop, returning an
/// [`AsyncHandle`] which can be used to execute the callback from any thread.
/// The callback will always be executed on the main thread.
pub fn new_async<F>(fun: F) -> super::Result<AsyncHandle>
where
    F: FnMut() -> crate::Result<()> + 'static,
{
    let layout = Layout::new::<libuv::uv_async_t>();
    let handle = unsafe { alloc::alloc(layout) as *mut libuv::uv_async_t };

    let callback_ptr = Box::into_raw(Box::new(Box::new(fun) as Cb));

    unsafe {
        libuv::uv_handle_set_data(handle as _, callback_ptr as *mut c_void)
    };

    let retv = unsafe {
        super::with_loop(|main_loop| {
            libuv::uv_async_init(main_loop, handle, Some(async_cb as _))
        })
    };

    if retv < 0 {
        // Free the callback.
        drop(unsafe { Box::from_raw(callback_ptr) });
        unsafe { libuv::uv_handle_set_data(handle as _, ptr::null_mut()) };

        // Free the handle.
        unsafe { alloc::dealloc(handle as *mut u8, layout) };

        return Err(super::Error::CouldntCreateAsyncHandle);
    }

    Ok(AsyncHandle { handle })
}

#[derive(Clone)]
pub struct AsyncHandle {
    // We should probably call `uv_close` when all the handles are dropped.
    //
    // TODO: Wrap the pointer in an Arc and implement Clone and Drop?
    handle: *mut libuv::uv_async_t,
}

unsafe impl Send for AsyncHandle {}
unsafe impl Sync for AsyncHandle {}

impl AsyncHandle {
    /// Wakes up the Neovim event loop and executes the callback associated to
    /// this handle. It is safe to call this function from any thread. The
    /// callback will be called on the main thread.
    ///
    /// NOTE: [libuv] will coalesce calls to [`AsyncHandle::send`], that is,
    /// not every call to it will yield an execution of the callback. For
    /// example: if [`AsyncHandle::send`] is called 5 times in a row before the
    /// callback is called, the callback will only be called once. If
    /// [`AsyncHandle::send`] is called again after the callback was called, it
    /// will be called again.
    ///
    /// [libuv]: https://libuv.org/
    pub fn send(&self) -> super::Result<()> {
        match unsafe { libuv::uv_async_send(self.handle) } {
            ret if ret < 0 => Err(super::Error::CouldntTriggerAsyncHandle),
            _ => Ok(()),
        }
    }
}
