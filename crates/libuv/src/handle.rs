use std::alloc::{self, Layout};
use std::ffi::c_void;
use std::marker::PhantomData;

use crate::{Error, Result, ffi};

/// TODO: docs
pub(crate) trait ProperLayout: Sized {}

/// TODO: docs
pub(crate) struct Handle<T, D: 'static> {
    ptr: *mut T,
    data: PhantomData<D>,
}

impl<T, D> Clone for Handle<T, D> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr, data: PhantomData }
    }
}

impl<T: ProperLayout, D> Handle<T, D> {
    /// TODO: docs
    pub(crate) fn new<I>(initializer: I) -> Result<Handle<T, D>>
    where
        I: FnOnce(*mut ffi::uv_loop_t, &mut Self) -> i32,
    {
        let layout = Layout::new::<T>();
        let ptr = unsafe { alloc::alloc(layout) as *mut T };

        if ptr.is_null() {
            return Err(Error::HandleMemAlloc);
        }

        let mut handle = Self { ptr, data: PhantomData };

        let retv = unsafe {
            crate::with_loop(|uv_loop| initializer(uv_loop, &mut handle))
        };

        if retv < 0 {
            unsafe { alloc::dealloc(ptr as *mut u8, layout) };
            return Err(Error::HandleInit);
        }

        Ok(handle)
    }

    pub(crate) fn as_ptr(&self) -> *const T {
        self.ptr.cast()
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    pub(crate) unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { ptr, data: PhantomData }
    }

    pub(crate) unsafe fn get_data(&self) -> *mut D {
        ffi::uv_handle_get_data(self.as_ptr() as *const ffi::uv_handle_t)
            as *mut D
    }

    pub(crate) unsafe fn set_data(&mut self, data: D) {
        let data = Box::into_raw(Box::new(data));

        ffi::uv_handle_set_data(
            self.as_mut_ptr() as *mut ffi::uv_handle_t,
            data as *mut c_void,
        )
    }
}
