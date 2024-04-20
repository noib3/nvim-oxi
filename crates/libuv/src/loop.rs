use core::cell::OnceCell;

use luajit::ffi::lua_State;

use crate::ffi;

thread_local! {
    static LOOP: OnceCell<*mut ffi::uv_loop_t> = const { OnceCell::new() };
}

/// Initializes the loop.
///
/// NOTE: this function **must** be called before calling any other function
/// exposed by this crate or there will be segfaults.
#[doc(hidden)]
pub unsafe fn init(lua_state: *mut lua_State) {
    LOOP.with(|uv_loop| uv_loop.set(ffi::luv_loop(lua_state)))
        .unwrap_unchecked();
}

/// Executes a function with access to the libuv loop.
///
/// NOTE: this will segfault if the loop has not been initialized by calling
/// [init].
pub(crate) unsafe fn with_loop<F, R>(fun: F) -> R
where
    F: FnOnce(*mut ffi::uv_loop_t) -> R,
{
    LOOP.with(move |uv_loop| fun(*uv_loop.get().unwrap()))
}
