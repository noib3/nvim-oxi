use std::alloc::{self, Layout};
use std::error::Error;
use std::ffi::c_void;
use std::marker::PhantomData;

use libuv_sys2::{self as ffi, uv_handle_t};

pub(crate) type HandleCallbackMut<T> =
    Box<dyn FnMut(&mut T) -> Result<(), Box<dyn Error>> + 'static>;

pub(crate) struct Handle<T, D: 'static> {
    ptr: *mut T,
    data: PhantomData<D>,
}

impl<T, D> Handle<T, D> {
    pub(crate) fn new() -> Handle<T, D> {
        let layout = Layout::new::<T>();
        let ptr = unsafe { alloc::alloc(layout) as *mut T };
        Self { ptr, data: PhantomData }
    }

    pub(crate) fn as_ptr(&self) -> *const T {
        self.ptr as _
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    pub(crate) unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { ptr, data: PhantomData }
    }

    pub(crate) fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub(crate) unsafe fn get_data(&self) -> *mut D {
        ffi::uv_handle_get_data(self.as_ptr() as *const uv_handle_t) as *mut D
    }

    pub(crate) unsafe fn set_data(&mut self, data: D) {
        let data = Box::into_raw(Box::new(data));

        ffi::uv_handle_set_data(
            self.as_mut_ptr() as *mut uv_handle_t,
            data as *mut c_void,
        )
    }
}

// impl<T, D> Drop for Handle<T, D> {
//     fn drop(&mut self) {
//         let layout = Layout::new::<T>();
//         unsafe { alloc::dealloc(self.ptr as *mut u8, layout) };
//     }
// }
