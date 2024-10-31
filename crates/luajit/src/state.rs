use core::cell::OnceCell;

use crate::ffi::State;

thread_local! {
    static LUA: OnceCell<*mut State> = const { OnceCell::new() };
}

/// Initializes the Lua state.
///
/// NOTE: this function **must** be called before calling any other function
/// exposed by this crate or there will be segfaults.
pub unsafe fn init(lstate: *mut State) {
    LUA.with(|lua| lua.set(lstate).unwrap_unchecked());
}

/// Executes a function with access to the Lua state.
///
/// NOTE: this will segfault if the Lua state has not been initialized by
/// calling [`init`].
pub unsafe fn with_state<F, R>(fun: F) -> R
where
    F: FnOnce(*mut State) -> R,
{
    LUA.with(move |lstate| fun(*lstate.get().unwrap()))
}
