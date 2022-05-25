use std::marker::{PhantomData, PhantomPinned};
use std::{mem, ptr};

use libc::c_int;
use once_cell::unsync::OnceCell;

use super::ffi;

thread_local! {
    static LUA: OnceCell<*mut lua_State> = OnceCell::new();
}

pub(crate) type Callback = Box<dyn FnMut() + 'static>;

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct lua_State {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// TODO: docs
#[inline(always)]
pub(crate) fn init_state(lstate: *mut lua_State) {
    LUA.with(|lua| lua.set(lstate).expect("couldn't initialize Lua state"));
}

/// TODO: docs
#[inline(always)]
pub(crate) fn with_state<F, R>(fun: F) -> R
where
    F: FnOnce(*mut lua_State) -> R,
{
    LUA.with(move |lua| unsafe { fun(*(lua.get().unwrap_unchecked())) })
}

/// TODO: docs
pub(crate) fn closure_to_luaref<F>(
    fun: F,
    _nargs: c_int,
    _nresults: c_int,
) -> crate::Result<c_int>
where
    F: FnMut() + 'static,
{
    unsafe extern "C" fn test(lstate: *mut lua_State) -> c_int {
        let fun = {
            let index = ffi::lua_upvalueindex(1);
            let upvalue = ffi::lua_touserdata(lstate, index) as *mut Callback;
            &mut **upvalue
        };
        let _ = fun();
        0
    }

    self::with_state(move |lstate| unsafe {
        let userdata = ffi::lua_newuserdata(lstate, mem::size_of::<Callback>())
            as *mut Callback;
        ptr::write(userdata, Box::new(fun));
        ffi::lua_pushcclosure(lstate, test, 1);
        Ok(ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX))
    })
}
