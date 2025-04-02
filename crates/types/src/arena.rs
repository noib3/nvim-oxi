use core::cell::OnceCell;
use core::ffi::c_char;
use core::ptr;

use libc::size_t;

thread_local! {
    static ARENA: OnceCell<Arena> = const { OnceCell::new() };
}

/// A memory arena that is passed to the C API for allocation.
#[allow(dead_code)]
#[repr(C)]
pub struct Arena {
    current_block: *mut c_char,
    pos: size_t,
    size: size_t,
}

impl Arena {
    #[inline]
    fn new() -> Self {
        Self { current_block: ptr::null_mut(), pos: 0, size: 0 }
    }
}

/// Initializes the [`Arena`].
///
/// This should be called as soon as the plugin is loaded.
#[doc(hidden)]
#[inline]
pub fn arena_init() {
    ARENA.with(|arena| {
        let _ = arena.set(Arena::new());
    });
}

/// Returns a pointer to the [`Arena`] that can be passed to the C API.
///
/// # Panics
///
/// Panics if the [`Arena`] wasn't initialized by calling [`arena_init`].
#[doc(hidden)]
#[inline]
pub fn arena() -> *mut Arena {
    ptr::null_mut()
    // ARENA.with(|arena| {
    //     let Some(arena) = arena.get() else {
    //         panic!("Arena is not initialized")
    //     };
    //     arena as *const _ as *mut _
    // })
}
