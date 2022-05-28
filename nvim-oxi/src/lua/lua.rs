use std::ffi::CStr;
use std::string::String as StdString;
use std::{mem, ptr};

use libc::{c_char, c_int};
use nvim_types::{BufHandle, LuaRef, Object};
use once_cell::unsync::OnceCell;

use super::ffi::{self, lua_Integer, lua_State};
use crate::Result;

thread_local! {
    static LUA: OnceCell<*mut lua_State> = OnceCell::new();
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

/// Pretty prints the contents of the Lua stack to the Neovim message area.
#[allow(dead_code)]
pub(crate) unsafe fn debug_stack(lstate: *mut lua_State) {
    let height = ffi::lua_gettop(lstate);

    let stack_pp = (1..height + 1)
        .map(|n| {
            let idx = height + 1 - n;
            let typename = CStr::from_ptr(ffi::luaL_typename(lstate, -n));
            format!("{idx}: {}", typename.to_string_lossy())
        })
        .collect::<Vec<String>>()
        .join("\n");

    crate::print!("{stack_pp}");
}

// pub(crate) type CallbackMut<A, R> = Box<dyn FnMut(A) -> Result<R> + 'static>;
// pub(crate) type LuaOnce<A, R> = Box<dyn FnOnce(A) -> Result<R> + 'static>;

// /// TODO: docs
// pub(crate) fn to_ref_mut<R>(
//     fun: CallbackMut<(), R>,
//     _nargs: c_int,
// ) -> Result<c_int> {
//     unsafe extern "C" fn test<R>(lstate: *mut lua_State) -> c_int {
//         let fun = {
//             let idx = ffi::lua_upvalueindex(1);
//             let upvalue =
//                 ffi::lua_touserdata(lstate, idx) as *mut CallbackMut<(), R>;
//             &mut **upvalue
//         };
//         let result = Box::new(fun(()));
//         ffi::lua_pushlightuserdata(
//             lstate,
//             Box::leak(result) as *mut Result<R> as *mut c_void,
//         );
//         let r#ref = ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX);
//         ffi::lua_pushinteger(lstate, r#ref as lua_Integer);
//         1
//     }

//     let r#ref = self::with_state(move |lstate| unsafe {
//         let ud =
//             ffi::lua_newuserdata(lstate, mem::size_of::<CallbackMut<(), R>>())
//                 as *mut CallbackMut<(), R>;
//         ptr::write(ud, fun);
//         ffi::lua_pushcclosure(lstate, test::<R>, 1);
//         ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX)
//     });

//     Ok(r#ref)
// }

// /// TODO: docs
// pub(crate) fn to_ref_once<R>(
//     fun: LuaOnce<(), R>,
//     _nargs: c_int,
// ) -> Result<c_int> {
//     unsafe extern "C" fn test<R>(lstate: *mut lua_State) -> c_int {
//         let fun = {
//             let idx = ffi::lua_upvalueindex(1);
//             let upvalue =
//                 ffi::lua_touserdata(lstate, idx) as *mut LuaOnce<(), R>;
//             Box::from_raw(&mut **upvalue as *mut (dyn FnOnce(()) -> Result<R>))
//         };
//         let result = Box::new(fun(()));
//         ffi::lua_pushlightuserdata(
//             lstate,
//             Box::leak(result) as *mut Result<R> as *mut c_void,
//         );
//         let r#ref = ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX);
//         ffi::lua_pushinteger(lstate, r#ref as lua_Integer);
//         1
//     }

//     let r#ref = self::with_state(move |lstate| unsafe {
//         let ud = ffi::lua_newuserdata(lstate, mem::size_of::<LuaOnce<(), R>>())
//             as *mut LuaOnce<(), R>;
//         ptr::write(ud, fun);
//         ffi::lua_pushcclosure(lstate, test::<R>, 1);
//         ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX)
//     });

//     Ok(r#ref)
// }

// pub(crate) fn to_result<R>(obj: Object) -> Result<R> {
//     // `obj` is usually the result of calling a Neovim C function and should
//     // contain a number which is the index in the Lua registry where the
//     // pointer pointing to the results to retrieve is stored.
//     let r#ref: LuaRef = Integer::try_from(obj)?.try_into()?;
//     self::with_state(|lstate| unsafe {
//         ffi::lua_rawgeti(lstate, ffi::LUA_REGISTRYINDEX, r#ref);
//         let ud = ffi::lua_touserdata(lstate, -1) as *mut Result<R>;
//         ffi::lua_pop(lstate, 1);
//         let results = (*Box::from_raw(ud))?;
//         ffi::luaL_unref(lstate, ffi::LUA_REGISTRYINDEX, r#ref);
//         Ok(results)
//     })
// }

type LuaFnMut = Box<dyn FnMut(*mut lua_State) -> Result<c_int> + 'static>;
type LuaFnOnce = Box<dyn FnOnce(*mut lua_State) -> Result<c_int> + 'static>;

pub(crate) fn mut_to_luaref<A, R, F>(mut fun: F) -> LuaRef
where
    A: LuaPoppable,
    R: LuaPushable,
    F: FnMut(A) -> Result<R> + 'static,
{
    unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
        let fun = {
            let idx = ffi::lua_upvalueindex(1);
            let upv = ffi::lua_touserdata(lstate, idx) as *mut LuaFnMut;
            &mut **upv
        };

        fun(lstate).unwrap_or_else(|_err| {
            // TODO
            panic!("what to do here?");
        })
    }

    self::with_state(move |lstate| unsafe {
        let fun = Box::new(move |lstate| fun(A::pop(lstate)?)?.push(lstate));
        let ud = ffi::lua_newuserdata(lstate, mem::size_of::<LuaFnMut>());
        ptr::write(ud as *mut LuaFnMut, fun);
        ffi::lua_pushcclosure(lstate, c_fun, 1);
        ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX)
    })
}

pub(crate) fn once_to_luaref<A, R, F>(fun: F) -> LuaRef
where
    A: LuaPoppable,
    R: LuaPushable,
    F: FnOnce(A) -> Result<R> + 'static,
{
    unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
        let fun = {
            let idx = ffi::lua_upvalueindex(1);
            let upv = ffi::lua_touserdata(lstate, idx) as *mut LuaFnOnce;
            Box::from_raw(&mut **upv)
        };

        fun(lstate).unwrap_or_else(|_err| {
            // TODO
            panic!("what to do here?");
        })
    }

    self::with_state(move |lstate| unsafe {
        let fun = Box::new(move |lstate| fun(A::pop(lstate)?)?.push(lstate));
        let ud = ffi::lua_newuserdata(lstate, mem::size_of::<LuaFnOnce>());
        ptr::write(ud as *mut LuaFnOnce, fun);
        ffi::lua_pushcclosure(lstate, c_fun, 1);
        ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX)
    })
}

// macro_rules! closure_to_luaref {
//     ($name:ident, $closure:ident, $type:ident, $fun_from_upv:ident) => {
//         pub(crate) fn $name<A, R, F>(fun: F) -> LuaRef
//         where
//             A: LuaPoppable,
//             R: LuaPushable,
//             F: $closure(A) -> Result<R> + 'static,
//         {
//             unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
//                 let fun = {
//                     let idx = ffi::lua_upvalueindex(1);
//                     let upv = ffi::lua_touserdata(lstate, idx) as *mut $type;
//                     $fun_from_upv
//                 };

//                 fun(lstate).expect("what to do here?")
//             }

//             self::with_state(move |lstate| unsafe {
//                 let fun =
//                     Box::new(move |lstate| fun(A::pop(lstate)?)?.push(lstate));
//                 let ud = ffi::lua_newuserdata(lstate, mem::size_of::<$type>());
//                 ptr::write(ud as *mut $type, fun);
//                 ffi::lua_pushcclosure(lstate, c_fun, 1);
//                 ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX)
//             })
//         }
//     };
// }

// closure_to_luaref!(
//     once_to_luaref,
//     FnOnce,
//     LuaFnOnce,
//     Box::from_raw(&mut **upv)
// );

pub(crate) trait LuaPoppable: Sized {
    /// Assembles itself by popping all the values on the stack. Fails if there
    /// aren't enough values or if they are of the wrong type.
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self>;
}

pub(crate) trait LuaPushable {
    /// Pushes all its values on the Lua stack, returning the number of values
    /// that it pushed.
    unsafe fn push(self, lstate: *mut lua_State) -> crate::Result<c_int>;
}

impl LuaPoppable for () {
    unsafe fn pop(_lstate: *mut lua_State) -> crate::Result<Self> {
        Ok(())
    }
}

impl LuaPoppable for lua_Integer {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        let int = ffi::lua_tointeger(lstate, -1);
        ffi::lua_pop(lstate, 1);
        Ok(int)
    }
}

impl LuaPoppable for u32 {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        Ok(lua_Integer::pop(lstate)?.try_into()?)
    }
}

impl LuaPoppable for BufHandle {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        Ok(lua_Integer::pop(lstate)?.try_into()?)
    }
}

impl LuaPoppable for usize {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        Ok(lua_Integer::pop(lstate)?.try_into()?)
    }
}

impl LuaPoppable for StdString {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        // TODO: check type and return err?
        let mut size = 0;
        let ptr = ffi::lua_tolstring(lstate, -1, &mut size);
        let mut str = StdString::with_capacity(size);
        ptr::copy(ptr as *const u8, str.as_mut_ptr(), size);
        str.as_mut_vec().set_len(size);
        ffi::lua_pop(lstate, 1);
        Ok(str)
    }
}

impl<T: LuaPoppable> LuaPoppable for Option<T> {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        let ltp = ffi::lua_type(lstate, -1);
        crate::print!(
            "{ltp}, {}",
            ltp != ffi::LUA_TNIL && ltp != ffi::LUA_TNONE
        );

        (ltp != ffi::LUA_TNIL && ltp != ffi::LUA_TNONE)
            .then(|| T::pop(lstate))
            .transpose()
    }
}

impl LuaPoppable for crate::api::buffer::OnLinesArgs {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        // self::debug_stack(lstate);

        let (h, i) = if ffi::lua_gettop(lstate) == 9 {
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

impl LuaPoppable for crate::api::buffer::OnBytesArgs {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        // TODO: Check that nargs is 12?
        // let nargs = ffi::lua_gettop(lstate);

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

impl LuaPoppable for crate::api::buffer::OnChangedtickArgs {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        let c = u32::pop(lstate)?;
        let b = BufHandle::pop(lstate)?;
        let a = <StdString as LuaPoppable>::pop(lstate)?;

        Ok((a, b.into(), c))
    }
}

impl LuaPoppable for crate::api::buffer::OnDetachArgs {
    unsafe fn pop(lstate: *mut lua_State) -> crate::Result<Self> {
        let b = BufHandle::pop(lstate)?;
        let a = <StdString as LuaPoppable>::pop(lstate)?;

        Ok((a, b.into()))
    }
}

impl<T: Into<Object>> LuaPushable for T {
    unsafe fn push(self, lstate: *mut lua_State) -> crate::Result<c_int> {
        let obj = self.into();

        use nvim_types::ObjectType::*;
        match obj.r#type {
            kObjectTypeNil => ffi::lua_pushnil(lstate),

            kObjectTypeBoolean => {
                let n = if obj.data.boolean { 1 } else { 0 };
                ffi::lua_pushboolean(lstate, n);
            },

            kObjectTypeInteger => {
                let n = obj.data.integer.try_into()?;
                ffi::lua_pushinteger(lstate, n);
            },

            kObjectTypeFloat => {
                ffi::lua_pushnumber(lstate, obj.data.float);
            },

            kObjectTypeString => {
                let string = &obj.data.string;
                ffi::lua_pushlstring(
                    lstate,
                    string.data as *const c_char,
                    string.size,
                );
            },

            kObjectTypeArray => todo!(),

            kObjectTypeDictionary => todo!(),

            kObjectTypeLuaRef => todo!(),
        }

        Ok(1)
    }
}
