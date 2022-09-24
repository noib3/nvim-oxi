use std::error::Error;
use std::time::Duration;

use libuv_sys2::{self as ffi, uv_timer_t};

use crate::{Handle, HandleCallbackMut};

pub struct TimerHandle {
    handle: Handle<uv_timer_t, HandleCallbackMut<Self>>,
}

impl TimerHandle {
    /// TODO: docs
    fn new() -> Result<Self, crate::Error> {
        let mut handle = Handle::new();

        if handle.is_null() {
            // TODO
            return Err(crate::Error::CouldntCreateAsyncHandle);
        }

        let retv = unsafe {
            crate::with_loop(|uv_loop| {
                ffi::uv_timer_init(uv_loop, handle.as_mut_ptr())
            })
        };

        if retv < 0 {
            // TODO
            return Err(crate::Error::CouldntCreateAsyncHandle);
        }

        Ok(Self { handle })
    }

    /// TODO: docs
    pub fn start<E, Cb>(
        timeout: &Duration,
        repeat: &Duration,
        mut callback: Cb,
    ) -> Result<Self, crate::Error>
    where
        E: Error + 'static,
        Cb: FnMut(&mut Self) -> Result<(), E> + 'static,
    {
        let mut timer = Self::new()?;

        let callback: HandleCallbackMut<Self> = Box::new(move |timer| {
            // Type erase the callback by boxing its error.
            callback(timer).map_err(|err| Box::new(err) as Box<dyn Error>)
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
            // TODO
            return Err(crate::Error::CouldntCreateAsyncHandle);
        }

        Ok(timer)
    }

    /// TODO: docs
    pub fn once<E, Cb>(
        timeout: &Duration,
        callback: Cb,
    ) -> Result<Self, crate::Error>
    where
        E: Error + 'static,
        Cb: FnOnce() -> Result<(), E> + 'static,
    {
        let mut callback = Some(callback);

        Self::start(timeout, &Duration::from_millis(0), move |timer| {
            let res = callback.take().unwrap()();
            timer.stop().unwrap();
            res
        })
    }

    /// TODO: docs
    pub fn stop(&mut self) -> Result<(), crate::Error> {
        let retv = unsafe { ffi::uv_timer_stop(self.handle.as_mut_ptr()) };

        if retv < 0 {
            // TODO
            return Err(crate::Error::CouldntCreateAsyncHandle);
        }

        Ok(())
    }
}

extern "C" fn timer_cb(handle: *mut uv_timer_t) {
    let handle: Handle<_, HandleCallbackMut<TimerHandle>> =
        unsafe { Handle::from_raw(handle) };

    let callback = unsafe { handle.get_data() };

    if !callback.is_null() {
        if let Err(_err) =
            unsafe { &mut *callback }(&mut TimerHandle { handle })
        {
            // TODO: what now?
        }
    }
}
