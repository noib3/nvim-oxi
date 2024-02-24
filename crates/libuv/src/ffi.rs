#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};
use core::marker::{PhantomData, PhantomPinned};

pub type uv_timer_cb = Option<unsafe extern "C" fn(handle: *mut uv_timer_t)>;

pub type uv_async_cb = Option<unsafe extern "C" fn(handle: *mut uv_async_t)>;

#[repr(C)]
struct handle {
    data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub(crate) struct uv_async_t(handle);

#[repr(C)]
pub(crate) struct uv_handle_t(handle);

#[repr(C)]
pub(crate) struct uv_loop_t(handle);

#[repr(C)]
pub(crate) struct uv_timer_t(handle);

extern "C" {
    // https://github.com/luvit/luv/blob/master/src/luv.c#L751
    pub(crate) fn luv_loop(
        lua_state: *mut luajit::ffi::lua_State,
    ) -> *mut uv_loop_t;

    pub(crate) fn uv_async_init(
        loop_: *mut uv_loop_t,
        async_: *mut uv_async_t,
        cb: uv_async_cb,
    ) -> c_int;

    pub(crate) fn uv_async_send(async_: *mut uv_async_t) -> c_int;

    pub(crate) fn uv_handle_get_data(
        handle: *const uv_handle_t,
    ) -> *mut c_void;

    pub(crate) fn uv_handle_set_data(
        handle: *mut uv_handle_t,
        data: *mut c_void,
    );

    pub(crate) fn uv_timer_init(
        loop_: *mut uv_loop_t,
        handle: *mut uv_timer_t,
    ) -> c_int;

    pub(crate) fn uv_timer_start(
        handle: *mut uv_timer_t,
        cb: uv_timer_cb,
        timeout: u64,
        repeat: u64,
    ) -> c_int;

    pub(crate) fn uv_timer_stop(handle: *mut uv_timer_t) -> c_int;
}
