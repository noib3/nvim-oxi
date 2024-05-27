use std::error::Error as StdError;

use crate::{ffi, Error, Handle, IntoResult};

type Callback = Box<dyn FnMut() -> Result<(), Box<dyn StdError>> + 'static>;

/// Binding to libuv's [Async handle][1] used to trigger the execution of a
/// callback in the Neovim thread.
///
/// [1]: http://docs.libuv.org/en/v1.x/async.html
#[derive(Clone)]
pub struct AsyncHandle {
    handle: Handle<ffi::uv_async_t, Callback>,
}

unsafe impl Send for AsyncHandle {}
unsafe impl Sync for AsyncHandle {}

impl AsyncHandle {
    /// Registers a new callback on the Neovim event loop, returning an
    /// [`AsyncHandle`] which can be used to execute the callback from any
    /// thread. The callback will always be executed on the main thread.
    pub fn new<Cb, R>(mut callback: Cb) -> Result<Self, Error>
    where
        Cb: FnMut() -> R + 'static,
        R: IntoResult<()>,
        R::Error: StdError + 'static,
    {
        let mut handle = Handle::new(|uv_loop, handle| unsafe {
            ffi::uv_async_init(
                uv_loop,
                handle.as_mut_ptr(),
                Some(async_cb as _),
            )
        })?;

        let callback: Callback = Box::new(move || {
            // Type erase the callback by boxing its error.
            callback()
                .into_result()
                .map_err(|err| Box::new(err) as Box<dyn StdError>)
        });

        unsafe { handle.set_data(callback) };

        Ok(Self { handle })
    }

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
    pub fn send(&self) -> Result<(), Error> {
        let retv =
            unsafe { ffi::uv_async_send(self.handle.as_ptr() as *mut _) };

        if retv < 0 {
            return Err(Error::AsyncTrigger);
        }

        Ok(())
    }
}

extern "C" fn async_cb(ptr: *mut ffi::uv_async_t) {
    let handle: Handle<_, Callback> = unsafe { Handle::from_raw(ptr) };

    let callback = unsafe { handle.get_data() };

    if !callback.is_null() {
        let callback = unsafe { &mut *callback };

        if let Err(_err) = callback() {
            // TODO: what now?
        }
    }
}
