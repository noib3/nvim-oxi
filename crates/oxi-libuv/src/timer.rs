use std::error::Error as StdError;
use std::time::Duration;

use libuv_sys2::{self as ffi, uv_timer_t};

use crate::{Error, Handle};

pub(crate) type Callback = Box<
    dyn FnMut(&mut TimerHandle) -> Result<(), Box<dyn StdError>> + 'static,
>;

/// Binding to libuv's [Timer handle][1] used to schedule callbacks to be
/// called in the future.
///
/// [1]: http://docs.libuv.org/en/v1.x/timer.html
pub struct TimerHandle {
    handle: Handle<uv_timer_t, Callback>,
}

impl TimerHandle {
    fn new() -> Result<Self, Error> {
        let handle = Handle::new(|uv_loop, handle| unsafe {
            ffi::uv_timer_init(uv_loop, handle.as_mut_ptr())
        })?;

        Ok(Self { handle })
    }

    /// Executes a callback every `repeat` interval starting after `timeout`.
    ///
    /// If the timeout is zero the callback will fire on the next event loop
    /// iteration.
    pub fn start<Cb, E>(
        timeout: Duration,
        repeat: Duration,
        mut callback: Cb,
    ) -> Result<Self, Error>
    where
        Cb: FnMut(&mut Self) -> Result<(), E> + 'static,
        E: StdError + 'static,
    {
        let mut timer = Self::new()?;

        let callback: Callback = Box::new(move |timer| {
            // Type erase the callback by boxing its error.
            callback(timer).map_err(|err| Box::new(err) as Box<dyn StdError>)
        });

        unsafe { timer.handle.set_data(callback) };

        let retv = unsafe {
            ffi::uv_timer_start(
                timer.handle.as_mut_ptr(),
                Some(timer_cb as _),
                timeout.as_millis() as u64,
                repeat.as_millis() as u64,
            )
        };

        if retv < 0 {
            return Err(Error::TimerStart);
        }

        Ok(timer)
    }

    /// Same as [`start()`](TimerHandle::start) but accepts a closure that
    /// will be called once before being automatically stopped.
    pub fn once<Cb, E>(timeout: Duration, callback: Cb) -> Result<Self, Error>
    where
        Cb: FnOnce() -> Result<(), E> + 'static,
        E: StdError + 'static,
    {
        let mut callback = Some(callback);

        Self::start(timeout, Duration::from_millis(0), move |timer| {
            let res = callback.take().unwrap()();
            timer.stop().unwrap();
            res
        })
    }

    /// Stops the timer.
    pub fn stop(&mut self) -> Result<(), Error> {
        let retv = unsafe { ffi::uv_timer_stop(self.handle.as_mut_ptr()) };

        if retv < 0 {
            return Err(Error::TimerStop);
        }

        Ok(())
    }
}

extern "C" fn timer_cb(ptr: *mut uv_timer_t) {
    let handle: Handle<_, Callback> = unsafe { Handle::from_raw(ptr) };

    let callback = unsafe { handle.get_data() };

    if !callback.is_null() {
        let mut handle = TimerHandle { handle };
        let callback = unsafe { &mut *callback };

        if let Err(_err) = callback(&mut handle) {
            // TODO: what now?
        }
    }
}
