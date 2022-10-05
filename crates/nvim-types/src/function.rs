use std::cell::RefCell;
use std::error::Error;
use std::ffi::c_int;
use std::fmt;
use std::marker::PhantomData;

use luajit_bindings::{self as lua, ffi, Poppable, Pushable};

use crate::LuaRef;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Function<A, R> {
    pub(crate) lua_ref: LuaRef,
    _pd: (PhantomData<A>, PhantomData<R>),
}

impl<A, R> fmt::Debug for Function<A, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<function {}: {} -> {}>",
            self.lua_ref,
            std::any::type_name::<A>(),
            std::any::type_name::<R>()
        )
    }
}

impl<A, R, F, E> From<F> for Function<A, R>
where
    F: FnMut(A) -> Result<R, E> + 'static,
    A: Poppable,
    R: Pushable,
    E: Error + 'static,
{
    fn from(fun: F) -> Function<A, R> {
        Function::from_fn_mut(fun)
    }
}

impl<A, R> Poppable for Function<A, R> {
    unsafe fn pop(
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<Self, lua::Error> {
        match ffi::lua_type(lstate, -1) {
            ffi::LUA_TFUNCTION => {
                let lua_ref = ffi::luaL_ref(lstate, ffi::LUA_REGISTRYINDEX);
                // TODO: check `lua_ref`.
                Ok(Self::from_ref(lua_ref))
            },

            ffi::LUA_TNONE => Err(lua::Error::PopEmptyStack),

            other => Err(lua::Error::pop_wrong_type::<Self>(
                ffi::LUA_TFUNCTION,
                other,
            )),
        }
    }
}

impl<A, R> Pushable for Function<A, R> {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<c_int, lua::Error> {
        let index = lua::ffi::LUA_REGISTRYINDEX;
        lua::ffi::lua_rawgeti(lstate, index, self.lua_ref);
        Ok(1)
    }
}

impl<A, R> Function<A, R> {
    pub(crate) fn from_ref(lua_ref: LuaRef) -> Self {
        Self { lua_ref, _pd: (PhantomData, PhantomData) }
    }

    #[doc(hidden)]
    pub fn lua_ref(&self) -> LuaRef {
        self.lua_ref
    }

    pub fn from_fn<F, E>(fun: F) -> Self
    where
        F: Fn(A) -> Result<R, E> + 'static,
        A: Poppable,
        R: Pushable,
        E: Error + 'static,
    {
        Self::from_ref(lua::function::store(fun))
    }

    pub fn from_fn_mut<F, E>(fun: F) -> Self
    where
        F: FnMut(A) -> Result<R, E> + 'static,
        A: Poppable,
        R: Pushable,
        E: Error + 'static,
    {
        let fun = RefCell::new(fun);

        Self::from_fn(move |args| {
            let fun = &mut *fun.try_borrow_mut().map_err(|_| {
                crate::Error::new() /* TODO */
            })?;

            fun(args).map_err(crate::Error::from_err)
        })
    }

    pub fn from_fn_once<F, E>(fun: F) -> Self
    where
        F: FnOnce(A) -> Result<R, E> + 'static,
        A: Poppable,
        R: Pushable,
        E: Error + 'static,
    {
        let fun = RefCell::new(Some(fun));

        Self::from_fn(move |args| {
            let fun = fun
                .try_borrow_mut()
                .map_err(|_| {
                    crate::Error::new() /* TODO */
                })?
                .take()
                .ok_or_else(crate::Error::new /* TODO */)?;

            fun(args).map_err(crate::Error::from_err)
        })
    }

    pub fn call(&self, args: A) -> Result<R, lua::Error>
    where
        A: Pushable,
        R: Poppable,
    {
        lua::function::call(self.lua_ref, args)
    }

    /// Consumes the `Function`, removing the reference stored in the Lua
    /// registry.
    #[doc(hidden)]
    pub fn remove_from_lua_registry(self) {
        lua::function::remove(self.lua_ref)
    }
}

#[cfg(feature = "serde")]
mod serde {
    use std::fmt;

    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use serde::ser::{Serialize, Serializer};

    use super::Function;
    use crate::LuaRef;

    impl<A, R> Serialize for Function<A, R> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_f32(self.lua_ref as f32)
        }
    }

    impl<'de, A, R> Deserialize<'de> for Function<A, R> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use std::marker::PhantomData;

            struct FunctionVisitor<A, R>(PhantomData<A>, PhantomData<R>);

            impl<'de, A, R> Visitor<'de> for FunctionVisitor<A, R> {
                type Value = Function<A, R>;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str("an f32 representing a Lua reference")
                }

                fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Function::from_ref(value as LuaRef))
                }
            }

            deserializer
                .deserialize_f32(FunctionVisitor(PhantomData, PhantomData))
        }
    }
}
