#![allow(non_camel_case_types)]

use libuv_sys2::uv_loop_t;
use once_cell::unsync::OnceCell;

thread_local! {
    static LOOP: OnceCell<*mut uv_loop_t> = OnceCell::new();
}

extern "C" {
    // https://github.com/luvit/luv/blob/master/src/luv.c#L751
    fn luv_loop(lua_state: *mut std::ffi::c_void) -> *mut uv_loop_t;
}

/// TODO: docs
pub unsafe fn init(lua_state: *mut std::ffi::c_void) {
    LOOP.with(|uv_loop| uv_loop.set(luv_loop(lua_state))).unwrap_unchecked();
}

/// Executes a function with access to the libuv loop.
pub(crate) unsafe fn with_loop<F, R>(fun: F) -> R
where
    F: FnOnce(*mut uv_loop_t) -> R,
{
    LOOP.with(move |uv_loop| fun(*uv_loop.get().unwrap_unchecked()))
}
