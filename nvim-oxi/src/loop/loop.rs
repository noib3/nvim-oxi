use libuv_sys2::uv_loop_t;
use once_cell::unsync::OnceCell;

use crate::lua;

extern "C" {
    // https://github.com/luvit/luv/blob/master/src/luv.c#L751
    fn luv_loop(L: *mut lua::lua_State) -> *mut uv_loop_t;
}

thread_local! {
    static LOOP: OnceCell<*mut uv_loop_t> = OnceCell::new();
}

#[inline]
pub(crate) unsafe fn init_loop(lstate: *mut lua::lua_State) {
    LOOP.with(|main_loop| main_loop.set(luv_loop(lstate)).unwrap_unchecked());
}

#[inline]
pub(crate) unsafe fn with_loop<F, R>(fun: F) -> R
where
    F: FnOnce(*mut uv_loop_t) -> R,
{
    LOOP.with(move |main_loop| fun(*(main_loop.get().unwrap_unchecked())))
}
