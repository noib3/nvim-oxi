use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

use luajit_bindings::{self as lua, LuaPoppable, LuaPushable};

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
    A: LuaPoppable,
    R: LuaPushable,
    E: Error + 'static,
{
    fn from(fun: F) -> Function<A, R> {
        Function::from_fn_mut(fun)
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
        A: LuaPoppable,
        R: LuaPushable,
        E: Error + 'static,
    {
        Self::from_ref(lua::function::store(fun))
    }

    pub fn from_fn_mut<F, E>(fun: F) -> Self
    where
        F: FnMut(A) -> Result<R, E> + 'static,
        A: LuaPoppable,
        R: LuaPushable,
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
        A: LuaPoppable,
        R: LuaPushable,
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
                .ok_or_else(|| crate::Error::new() /* TODO */)?;

            fun(args).map_err(crate::Error::from_err)
        })
    }

    pub fn call(&self, args: A) -> R
    where
        A: LuaPushable,
        R: LuaPoppable,
    {
        lua::function::call(self.lua_ref, args).unwrap() /* TODO */
    }

    /// Consumes the `Function`, removing the reference stored in the Lua
    /// registry.
    #[doc(hidden)]
    pub fn remove_from_lua_registry(self) {
        lua::function::remove(self.lua_ref)
    }
}
