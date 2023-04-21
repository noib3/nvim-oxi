use libuv_sys2::uv_loop_t;
use once_cell::unsync::OnceCell;
use oxi_luajit::ffi::lua_State;

thread_local! {
    static LOOP: OnceCell<*mut uv_loop_t> = OnceCell::new();
}

extern "C" {
    // https://github.com/luvit/luv/blob/master/src/luv.c#L751
    fn luv_loop(lua_state: *mut lua_State) -> *mut uv_loop_t;
}

/// Initializes the loop.
///
/// NOTE: this function **must** be called before calling any other function
/// exposed by this crate or there will be segfaults.
#[doc(hidden)]
pub unsafe fn init(lua_state: *mut lua_State) {
    LOOP.with(|uv_loop| uv_loop.set(luv_loop(lua_state))).unwrap_unchecked();
}

/// Executes a function with access to the libuv loop.
///
/// NOTE: this will segfault if the loop has not been initialized by calling
/// [init].
pub(crate) unsafe fn with_loop<F, R>(fun: F) -> R
where
    F: FnOnce(*mut uv_loop_t) -> R,
{
    LOOP.with(move |uv_loop| fun(*uv_loop.get().unwrap()))
}
