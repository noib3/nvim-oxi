#[cfg(feature = "libuv-rs")]
use libuv::r#loop::Loop as LibuvRsLoop;
use libuv_sys2::uv_loop_t;

// https://github.com/neovim/neovim/blob/master/src/nvim/event/loop.h#L44
#[repr(C)]
struct Loop {
    uv: uv_loop_t,
}

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/main.c#L107
    #[link_name = "main_loop"]
    static MAIN_LOOP: Loop;
}

thread_local! {
    /// Handle to the main [libuv event loop] used by Neovim to schedule its
    /// asynchronous tasks.
    ///
    /// [libuv event loop]: http://docs.libuv.org/en/v1.x/loop.html
    pub static LOOP: uv_loop_t = unsafe { MAIN_LOOP.uv };

    /// TODO: docs
    #[cfg(feature = "libuv-rs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "libuv-rs")))]
    pub static LIBUV_RS_LOOP: LibuvRsLoop = LibuvRsLoop::from_ptr(&LOOP as *const _ as *mut _);
}

#[cfg(feature = "libuv-rs")]
trait FromPtr {
    fn from_ptr(ptr: *mut uv_loop_t) -> LibuvRsLoop;
}

#[cfg(feature = "libuv-rs")]
impl FromPtr for LibuvRsLoop {
    fn from_ptr(ptr: *mut uv_loop_t) -> LibuvRsLoop {
        todo!()
    }
}
