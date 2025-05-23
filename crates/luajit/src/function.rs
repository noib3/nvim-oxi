use core::error::Error;
use core::ffi::{CStr, c_int};
use core::mem;
use core::ptr;

use crate::ffi::{self, State};
use crate::{IntoResult, Poppable, Pushable, utils};

/// Stores a function in the Lua registry, returning its ref.
pub fn store<F, A, R, O>(fun: F) -> c_int
where
    F: Fn(A) -> R + 'static,
    A: Poppable,
    R: IntoResult<O>,
    O: Pushable,
    R::Error: Error + 'static,
{
    type Callback =
        Box<dyn Fn(*mut State) -> Result<c_int, crate::Error> + 'static>;

    unsafe extern "C" fn c_fun(lstate: *mut State) -> c_int {
        let fun = {
            let idx = ffi::lua_upvalueindex(1);
            let upv = ffi::lua_touserdata(lstate, idx) as *mut Callback;
            &**upv
        };

        fun(lstate).unwrap_or_else(|err| utils::push_error(&err, lstate))
    }

    unsafe {
        crate::with_state(move |lstate| {
            let fun = move |lstate| {
                let args = A::pop(lstate)?;
                let ret = fun(args)
                    .into_result()
                    .map_err(crate::Error::push_error_from_err::<R, _>)?;
                ret.push(lstate)
            };

            let ud = ffi::lua_newuserdata(lstate, mem::size_of::<Callback>());
            ptr::write(ud as *mut Callback, Box::new(fun));

            ffi::lua_pushcclosure(lstate, c_fun, 1);
            ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX)
        })
    }
}

/// Calls a function previously stored in the Lua registry via [store].
pub fn call<A, R>(lua_ref: c_int, args: A) -> Result<R, crate::Error>
where
    A: Pushable,
    R: Poppable,
{
    unsafe {
        crate::with_state(move |lstate| {
            ffi::lua_rawgeti(lstate, ffi::LUA_REGISTRYINDEX, lua_ref);
            let nargs = args.push(lstate)?;

            match ffi::lua_pcall(lstate, nargs, -1, 0 /* <- errorfunc */) {
                ffi::LUA_OK => R::pop(lstate),

                err_code => {
                    let msg = CStr::from_ptr(ffi::lua_tostring(lstate, -1))
                        .to_string_lossy()
                        .to_string();

                    ffi::lua_pop(lstate, 1);

                    match err_code {
                        ffi::LUA_ERRRUN => {
                            Err(crate::Error::RuntimeError(msg))
                        },

                        ffi::LUA_ERRMEM => Err(crate::Error::MemoryError(msg)),

                        ffi::LUA_ERRERR => {
                            unreachable!("errorfunc is 0, this never happens!")
                        },

                        _ => unreachable!(),
                    }
                },
            }
        })
    }
}

/// Removes the function reference stored in the Lua registry
pub fn remove(lua_ref: c_int) {
    unsafe {
        crate::with_state(|lstate| {
            ffi::luaL_unref(lstate, ffi::LUA_REGISTRYINDEX, lua_ref)
        })
    }
}
