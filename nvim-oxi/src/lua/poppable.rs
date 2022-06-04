use std::ptr;
use std::string::String as StdString;

use nvim_types::BufHandle;

use super::ffi::*;
use crate::api::opts;
use crate::Result;

#[doc(hidden)]
pub trait LuaPoppable: Sized {
    /// Assembles itself by popping values off the stack. Fails if there aren't
    /// enough values or if they are of the wrong type.
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self>;
}

impl LuaPoppable for () {
    unsafe fn pop(_lstate: *mut lua_State) -> Result<Self> {
        Ok(())
    }
}

impl LuaPoppable for lua_Integer {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        let int = lua_tointeger(lstate, -1);
        lua_pop(lstate, 1);
        Ok(int)
    }
}

impl LuaPoppable for u32 {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        Ok(lua_Integer::pop(lstate)?.try_into()?)
    }
}

impl LuaPoppable for BufHandle {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        Ok(lua_Integer::pop(lstate)?.try_into()?)
    }
}

impl LuaPoppable for usize {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        Ok(lua_Integer::pop(lstate)?.try_into()?)
    }
}

impl LuaPoppable for StdString {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        // TODO: check type and return err?
        let mut size = 0;
        let ptr = lua_tolstring(lstate, -1, &mut size);
        let mut str = StdString::with_capacity(size);
        ptr::copy(ptr as *const u8, str.as_mut_ptr(), size);
        str.as_mut_vec().set_len(size);
        lua_pop(lstate, 1);
        Ok(str)
    }
}

impl<T: LuaPoppable> LuaPoppable for Option<T> {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        let ltp = lua_type(lstate, -1);
        crate::print!("{ltp}, {}", ltp != LUA_TNIL && ltp != LUA_TNONE);

        (ltp != LUA_TNIL && ltp != LUA_TNONE)
            .then(|| T::pop(lstate))
            .transpose()
    }
}

impl LuaPoppable for opts::OnLinesArgs {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        let (h, i) = if lua_gettop(lstate) == 9 {
            let h = usize::pop(lstate)?;
            let i = usize::pop(lstate)?;
            (Some(h), Some(i))
        } else {
            (None, None)
        };
        let g = usize::pop(lstate)?;
        let f = usize::pop(lstate)?;
        let e = usize::pop(lstate)?;
        let d = usize::pop(lstate)?;
        let c = u32::pop(lstate)?;
        let b = BufHandle::pop(lstate)?;
        let a = <StdString as LuaPoppable>::pop(lstate)?;

        Ok((a, b.into(), c, d, e, f, g, h, i))
    }
}

impl LuaPoppable for opts::OnBytesArgs {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        // TODO: Check that nargs is 12?
        // let nargs = lua_gettop(lstate);

        let n = usize::pop(lstate)?;
        let m = usize::pop(lstate)?;
        let l = usize::pop(lstate)?;
        let i = usize::pop(lstate)?;
        let h = usize::pop(lstate)?;
        let g = usize::pop(lstate)?;
        let f = usize::pop(lstate)?;
        let e = usize::pop(lstate)?;
        let d = usize::pop(lstate)?;
        let c = u32::pop(lstate)?;
        let b = BufHandle::pop(lstate)?;
        let a = <StdString as LuaPoppable>::pop(lstate)?;

        Ok((a, b.into(), c, d, e, f, g, h, i, l, m, n))
    }
}

impl LuaPoppable for opts::OnChangedtickArgs {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        let c = u32::pop(lstate)?;
        let b = BufHandle::pop(lstate)?;
        let a = <StdString as LuaPoppable>::pop(lstate)?;

        Ok((a, b.into(), c))
    }
}

impl LuaPoppable for opts::OnDetachArgs {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        let b = BufHandle::pop(lstate)?;
        let a = <StdString as LuaPoppable>::pop(lstate)?;

        Ok((a, b.into()))
    }
}

impl LuaPoppable for (StdString, StdString, usize) {
    unsafe fn pop(lstate: *mut lua_State) -> Result<Self> {
        let c = usize::pop(lstate)?;
        let b = <StdString as LuaPoppable>::pop(lstate)?;
        let a = <StdString as LuaPoppable>::pop(lstate)?;

        Ok((a, b, c))
    }
}
