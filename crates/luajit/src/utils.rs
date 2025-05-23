use core::ffi::{CStr, c_int};
use core::fmt::Display;

use crate::ffi::{self, State};

/// Does nothing if the stack is already taller than `n`, grows the stack
/// height to `n` by adding `nil`s if it's not.
pub unsafe fn grow_stack(lstate: *mut State, n: c_int) {
    unsafe {
        if ffi::lua_gettop(lstate) < n {
            ffi::lua_settop(lstate, n);
        }
    }
}

/// Returns a displayable representation of the Lua value at a given stack
/// index.
pub unsafe fn debug_value(lstate: *mut State, n: c_int) -> Box<dyn Display> {
    match ffi::lua_type(lstate, n) {
        ffi::LUA_TNONE | ffi::LUA_TNIL => Box::new("()"),

        ffi::LUA_TBOOLEAN => Box::new(ffi::lua_toboolean(lstate, n) == 1),

        ffi::LUA_TSTRING => Box::new(
            CStr::from_ptr(ffi::lua_tostring(lstate, n)).to_string_lossy(),
        ),

        ffi::LUA_TNUMBER => Box::new(ffi::lua_tonumber(lstate, n)),

        _ => Box::new("other"),
    }
}

/// Assumes that the value at index `index` is a table and returns whether it's
/// an array table (as opposed to a dictionary table).
pub unsafe fn is_table_array(lstate: *mut State, index: c_int) -> bool {
    ffi::lua_pushnil(lstate);

    if ffi::lua_next(lstate, index - 1) == 0 {
        // Empty table.
        if ffi::lua_getmetatable(lstate, index) == 0 {
            return true;
        }
        ffi::lua_pop(lstate, 1);
        return false;
    }

    let ty = ffi::lua_type(lstate, -2);
    ffi::lua_pop(lstate, 2);
    ty == ffi::LUA_TNUMBER
}

/// Returns the type of the Lua value at a given stack index.
pub unsafe fn debug_type(lstate: *mut State, n: c_int) -> impl Display {
    CStr::from_ptr(ffi::luaL_typename(lstate, n)).to_string_lossy()
}

/// Pretty prints the contents of the Lua stack to the Neovim message area.
pub unsafe fn debug_stack(lstate: *mut State) {
    let height = ffi::lua_gettop(lstate);

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

pub unsafe fn push_error<E: core::fmt::Display + ?Sized>(
    err: &E,
    lstate: *mut State,
) -> ! {
    let msg = err.to_string();
    ffi::lua_pushlstring(lstate, msg.as_ptr() as *const _, msg.len());
    drop(msg);
    ffi::lua_error(lstate);
}

pub fn type_name(ty: c_int) -> &'static str {
    match ty {
        ffi::LUA_TNONE => "empty stack",
        ffi::LUA_TNIL => "nil",
        ffi::LUA_TBOOLEAN => "boolean",
        ffi::LUA_TLIGHTUSERDATA => "light userdata",
        ffi::LUA_TNUMBER => "number",
        ffi::LUA_TSTRING => "string",
        ffi::LUA_TTABLE => "table",
        ffi::LUA_TFUNCTION => "function",
        ffi::LUA_TUSERDATA => "userdata",
        ffi::LUA_TTHREAD => "thread",
        _ => unreachable!(),
    }
}
