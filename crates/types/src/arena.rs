use core::ffi::c_char;
use core::ptr;

use libc::size_t;
use once_cell::unsync::OnceCell;

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
/// This should be called exactly once during the lifetime of the plugin and as
/// soon as it is loaded.
///
/// # Panics
///
/// Panics if this function was already called.
#[doc(hidden)]
#[inline]
pub fn arena_init() {
    ARENA.with(|arena| {
        if arena.set(Arena::new()).is_err() {
            panic!("Arena is already initialized");
        }
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
    ARENA.with(|arena| {
        let Some(arena) = arena.get() else {
            panic!("Arena is not initialized")
        };
        arena as *const _ as *mut _
    })
}
