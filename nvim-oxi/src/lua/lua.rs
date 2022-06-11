use std::ffi::CStr;
use std::mem;

use once_cell::unsync::OnceCell;

use super::ffi::*;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L41
const INTERNAL_CALL_MASK: u64 = 1u64 << (mem::size_of::<u64>() * 8 - 1);

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L44
const VIML_INTERNAL_CALL: u64 = INTERNAL_CALL_MASK;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L47
pub(crate) const LUA_INTERNAL_CALL: u64 = VIML_INTERNAL_CALL + 1;

thread_local! {
    static LUA: OnceCell<*mut lua_State> = OnceCell::new();
}

/// Initializes the Lua state. It's only called once when the module is loaded,
/// and calling it more than once is ub.
#[inline(always)]
unsafe fn init_state(lstate: *mut lua_State) {
    LUA.with(|lua| lua.set(lstate).unwrap_unchecked());
}

/// TODO: docs
#[doc(hidden)]
#[inline(always)]
pub unsafe fn module_entrypoint<F, R>(
    lstate: *mut lua_State,
    body: F,
) -> libc::c_int
where
    R: super::LuaPushable,
    F: FnOnce() -> crate::Result<R> + 'static,
{
    self::init_state(lstate);
    body().unwrap().push(lstate).unwrap()
}

/// Runs a piece of code with access to the raw Lua state. Calling this before
/// the state has been initialized is ub.
#[inline(always)]
pub(crate) fn with_state<F, R>(fun: F) -> R
where
    F: FnOnce(*mut lua_State) -> R,
{
    LUA.with(move |lua| unsafe { fun(*(lua.get().unwrap_unchecked())) })
}

/// Pretty prints the contents of the Lua stack to the Neovim message area.
#[allow(dead_code)]
pub(crate) unsafe fn debug_stack(lstate: *mut lua_State) {
    let height = lua_gettop(lstate);

    let stack_pp = (1..height + 1)
        .map(|n| {
            let idx = height + 1 - n;
            let typename = CStr::from_ptr(luaL_typename(lstate, -n));
            format!("{idx}: {typename:?}")
        })
        .collect::<Vec<String>>()
        .join("\n");

    crate::print!("{stack_pp}");
}
