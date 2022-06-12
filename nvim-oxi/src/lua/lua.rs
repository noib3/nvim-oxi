use std::ffi::CStr;
use std::{fmt, mem};

use libc::c_int;
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

/// The entrypoint of the plugin. Initializes the Lua state, executes the
/// entrypoint function and pushes the result on the stack.
#[doc(hidden)]
#[inline(always)]
pub unsafe fn module_entrypoint<F, R>(
    lstate: *mut lua_State,
    body: F,
) -> libc::c_int
where
    F: FnOnce() -> crate::Result<R> + 'static,
    R: super::LuaPushable,
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

/// No-op if the stack is already taller than `n`, grows the stack to `n` by
/// adding `nil`s if it's not.
pub(crate) fn grow_stack(lstate: *mut lua_State, n: c_int) {
    unsafe {
        if lua_gettop(lstate) < n {
            lua_settop(lstate, n);
        }
    }
}

/// Pretty prints the contents of the Lua stack to the Neovim message area.
#[allow(dead_code)]
pub(crate) unsafe fn debug_stack(lstate: *mut lua_State) {
    let height = lua_gettop(lstate);

    let stack_pp = (1..height + 1)
        .map(|n| {
            let idx = height + 1 - n;
            let value = debug_value(lstate, -n);
            let typename = debug_type(lstate, -n);
            format!("{idx}: {value} ({typename})")
        })
        .collect::<Vec<String>>()
        .join("\n");

    crate::print!("{stack_pp}");
}

/// Returns the string representation of the Lua value at a given stack index.
pub(crate) unsafe fn debug_value(lstate: *mut lua_State, n: c_int) -> String {
    match lua_type(lstate, n) {
        LUA_TNONE | LUA_TNIL => "()".to_string(),

        LUA_TBOOLEAN => (lua_toboolean(lstate, n) == 1).to_string(),

        LUA_TSTRING => {
            format!("{:?}", CStr::from_ptr(lua_tostring(lstate, n)))
        },

        LUA_TNUMBER => lua_tonumber(lstate, n).to_string(),

        _ => "other".to_string(),
    }
}

/// Returns the type of the Lua value at a given stack index.
pub(crate) unsafe fn debug_type(
    lstate: *mut lua_State,
    n: c_int,
) -> impl fmt::Display {
    CStr::from_ptr(luaL_typename(lstate, n)).to_string_lossy()
}
