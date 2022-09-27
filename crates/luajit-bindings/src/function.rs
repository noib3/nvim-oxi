use std::error::Error;
use std::ffi::{c_int, CStr};
use std::mem;
use std::ptr;

use crate::ffi::{self, lua_State};
use crate::{utils, LuaPoppable, LuaPushable};

/// Stores a function in the Lua registry, returning its ref.
pub fn store<F, A, R, E>(fun: F) -> c_int
where
    F: Fn(A) -> Result<R, E> + 'static,
    A: LuaPoppable,
    R: LuaPushable,
    E: Error + 'static,
{
    type Callback =
        Box<dyn Fn(*mut lua_State) -> Result<c_int, crate::Error> + 'static>;

    unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
        let fun = {
            let idx = ffi::lua_upvalueindex(1);
            let upv = ffi::lua_touserdata(lstate, idx) as *mut Callback;
            &**upv
        };

        fun(lstate).unwrap_or_else(|err| utils::handle_error(lstate, &err))
    }

    unsafe {
        crate::with_state(move |lstate| {
            let fun = move |lstate| {
                let args = A::pop(lstate)?;
                let ret =
                    fun(args).map_err(crate::Error::push_ciao::<R, _>)?;
                ret.push(lstate)
            };

            let ud = ffi::lua_newuserdata(lstate, mem::size_of::<Callback>());
            ptr::write(ud as *mut Callback, Box::new(fun));

            ffi::lua_pushcclosure(lstate, c_fun, 1);
            ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX)
        })
    }
}

/// Calls a function previously stored in the Lua registry via [store_fn].
pub fn call<A, R>(lua_ref: c_int, args: A) -> Result<R, crate::Error>
where
    A: LuaPushable,
    R: LuaPoppable,
{
    unsafe {
        crate::with_state(move |lstate| {
            ffi::lua_rawgeti(lstate, ffi::LUA_REGISTRYINDEX, lua_ref);
            let nargs = args.push(lstate).unwrap()/* TODO */;

            match ffi::lua_pcall(
                lstate,
                nargs,
                R::N,
                0, /* <- errorfunc */
            ) {
                ffi::LUA_OK => Ok(R::pop(lstate).unwrap()), /* TODO */

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
