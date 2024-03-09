//! FFI bindings to libuv.
//!
//! We were previously using the `libuv-sys2` crate for this, but since we
//! don't need to provide bindings to the entire libuv API we can remove a
//! direct dependency and ~30 indirect build dependencies by only writing the
//! bindings we need manually.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_uint, c_void};
use core::marker::{PhantomData, PhantomPinned};

pub(crate) type uv_timer_cb =
    Option<unsafe extern "C" fn(handle: *mut uv_timer_t)>;

pub(crate) type uv_async_cb =
    Option<unsafe extern "C" fn(handle: *mut uv_async_t)>;

#[repr(C)]
struct handle {
    data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub(crate) struct uv_loop_t(handle);

#[repr(C)]
pub(crate) struct uv_handle_t(handle);

#[repr(C)]
pub(crate) struct uv_async_t(uv_async_s);

impl crate::ProperLayout for uv_async_t {}

#[repr(C)]
pub(crate) struct uv_timer_t(uv_timer_s);

impl crate::ProperLayout for uv_timer_t {}

#[link(name = "nvim", kind = "raw-dylib")]
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

#[repr(C)]
struct uv_async_s {
    data: *mut ::std::os::raw::c_void,
    loop_: *mut uv_loop_t,
    type_: uv_handle_type,
    close_cb: uv_close_cb,
    handle_queue: uv__queue,
    u: uv_async_s__bindgen_ty_1,
    next_closing: *mut uv_handle_t,
    flags: c_uint,
    async_cb: uv_async_cb,
    queue: uv__queue,
    pending: c_int,
}

#[repr(C)]
struct uv_timer_s {
    data: *mut ::std::os::raw::c_void,
    loop_: *mut uv_loop_t,
    type_: uv_handle_type,
    close_cb: uv_close_cb,
    handle_queue: uv__queue,
    u: uv_timer_s__bindgen_ty_1,
    next_closing: *mut uv_handle_t,
    flags: ::std::os::raw::c_uint,
    timer_cb: uv_timer_cb,
    node: uv_timer_s__bindgen_ty_2,
    timeout: u64,
    repeat: u64,
    start_id: u64,
}

type uv_handle_type = c_uint;

type uv_close_cb = Option<unsafe extern "C" fn(handle: *mut uv_handle_t)>;

#[repr(C)]
#[derive(Copy, Clone)]
struct uv__queue {
    next: *mut uv__queue,
    prev: *mut uv__queue,
}

#[repr(C)]
union uv_async_s__bindgen_ty_1 {
    fd: c_int,
    reserved: [*mut c_void; 4usize],
}

#[repr(C)]
union uv_timer_s__bindgen_ty_1 {
    fd: c_int,
    reserved: [*mut c_void; 4usize],
}

#[repr(C)]
union uv_timer_s__bindgen_ty_2 {
    heap: [*mut c_void; 3usize],
    queue: uv__queue,
}
