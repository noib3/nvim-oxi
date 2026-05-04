use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::{cell::LazyCell, error::Error as StdError};
use std::{convert, fmt};

use mlua::{ExternalResult, FromLuaMulti, IntoLuaMulti};
use oximlua as olua;

use crate::LuaRef;

thread_local! {
    static CACHE: LazyCell<BTreeMap<LuaRef, CacheEntry>> = LazyCell::new(|| BTreeMap::new());
}

struct CacheEntry {
    ref_count: usize,
    fun: Option<mlua::Function>,
    reg: Option<mlua::RegistryKey>,
}

impl CacheEntry {
    fn from_ref(lua_ref: LuaRef) -> Self {
        let value = olua::get_registry_value(lua_ref).unwrap();
        let fun = value
            .as_function()
            .ok_or_else(mlua::Error::ToLuaConversionError {
                from: format!("{{LuaRef({lua_ref})}}"),
                to: "mlua::Value::Function",
                message: Some(format!(
                    "Lua registry contained <{}>({:?}) instead",
                    std::any::type_name_of_val(&value),
                    value
                )),
            })
            .unwrap();

        Self { ref_count: 1, fun: Some(fun.to_owned()), reg: None }
    }

    fn from_mlua(fun: mlua::Function, reg: mlua::RegistryKey) -> Self {
        Self { ref_count: 1, fun: Some(fun), reg: Some(reg) }
    }
}

/// A wrapper around a Lua reference to a function stored in the Lua registry.
#[derive(Eq, PartialEq, Hash)]
pub struct Function<A, R>
where
    A: IntoLuaMulti,
    R: FromLuaMulti,
{
    pub(crate) lua_ref: LuaRef,
    _pd: (PhantomData<A>, PhantomData<R>),
}

impl<A, R> Clone for Function<A, R>
where
    A: IntoLuaMulti,
    R: FromLuaMulti,
{
    fn clone(&self) -> Self {
        Self::from_ref(self.lua_ref)
    }
}

impl<A, R> fmt::Debug for Function<A, R>
where
    A: IntoLuaMulti,
    R: FromLuaMulti,
{
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

impl<A, R> Drop for Function<A, R> {
    fn drop(&mut self) {
        unsafe {
            Function::remove_cache_ref_count(self.lua_ref);
        }
    }
}

impl<A, R, F> From<F> for Function<A, R>
where
    F: Fn(A) -> R + mlua::MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
{
    fn from(fun: F) -> Function<A, R> {
        Function::from_fn_mut(fun)
    }
}

impl<A, R> From<mlua::Function<A, R>> for Function<A, R> {
    fn from(value: mlua::Function<A, R>) -> Self {
        let reg = olua::get_lua().create_registry_value(value).unwrap();
        let lua_ref = reg.id();
        CACHE
            .with(convert::identity)
            .insert(lua_ref, CacheEntry::from_mlua(value, reg));

        Self { lua_ref, _pd: (PhantomData, PhantomData) }
    }
}

impl<A, R> From<Function<A, R>> for mlua::Function<A, R> {
    fn from(value: Function<A, R>) -> Self {
        CACHE
            .with(convert::identity)
            .get(&value.lua_ref)
            .expect("cached function entry should exist")
            .fun
            .clone()
            .expect("mlua Function should exist and be cloneable")
    }
}

impl<A, R> mlua::FromLua for Function<A, R> {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::Function(fun) = value {
            Ok(Function::from(fun))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: std::any::type_name_of_val(&value),
                to: std::any::type_name::<Self>().to_string(),
                message: Some(
                    "expected `<mlua::Value::Function>`".to_string(),
                ),
            })
        }
    }
}

impl<A, R> mlua::IntoLua for Function<A, R> {
    fn into_lua(self, _: &mlua::Lua) -> mlua::Result<mlua::Value> {
        CACHE
            .with(convert::identity)
            .get(&self.lua_ref)
            .expect("cached function entry should exist")
            .fun
            .clone()
            .into()
    }
}

impl<A, R> Function<A, R>
where
    A: IntoLuaMulti,
    R: FromLuaMulti,
{
    pub(crate) fn from_ref(lua_ref: LuaRef) -> Self {
        CACHE
            .with(convert::identity)
            .entry(lua_ref)
            .and_modify(|entry| entry.ref_count += 1)
            .or_insert_with_key(CacheEntry::from_ref);

        Self { lua_ref, _pd: (PhantomData, PhantomData) }
    }

    #[doc(hidden)]
    pub fn lua_ref(&self) -> LuaRef {
        self.lua_ref
    }

    pub fn from_fn<F, R>(fun: F) -> Self
    where
        F: Fn(A) -> R + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        Self::from(mlua::Function::wrap(fun))
    }

    pub fn from_fn_mut<F, R>(fun: F) -> Self
    where
        F: Fn(A) -> R + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        Self::from(mlua::Function::wrap_mut(fun))
    }

    pub fn from_fn_once<F, R>(fun: F) -> Self
    where
        F: Fn(A) -> R + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        let fun = RefCell::new(Some(fun));

        Self::from_fn(move |args| {
            let fun = fun
                .try_borrow_mut()
                .map_err(mlua::Error::RecursiveMutCallback)?
                .take()
                .ok_or_else(|| {
                    Err("Cannot call function twice").into_lua_err()
                })?;

            fun(args)
        })
    }

    pub fn call(&self, args: A) -> mlua::Result<R>
    where
        A: IntoLuaMulti,
        R: FromLuaMulti,
    {
        mlua::Function::from(self).call(args)
    }

    pub unsafe fn remove_cache_ref_count(lua_ref: LuaRef) {
        let entry = CACHE
            .with(convert::identity)
            .remove(&lua_ref)
            .expect("cached function entry should exist");

        // TODO: Consider again if this case is likely to be rare
        //       thus justifying removing and reinserting the entry instead of
        //       getting the entry and only removing it if this check is false.
        if entry.ref_count > 1 {
            entry.ref_count -= 1;
            let entry = CACHE.with(convert::identity).insert(lua_ref, entry);
            return;
        }

        if let Some(reg) = entry.reg.take() {
            olua::get_lua().remove_registry_value(reg).unwrap()
        };
    }

    pub unsafe fn add_cache_ref_count(lua_ref: LuaRef) {
        let entry = CACHE
            .with(convert::identity)
            .get_mut(&lua_ref)
            .expect("cached function entry should exist");
        entry.ref_count += 1;
    }
}

#[cfg(feature = "serde")]
mod serde {
    use std::fmt;

    use mlua::{FromLuaMulti, IntoLuaMulti};
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use serde::ser::{Serialize, Serializer};

    use super::Function;
    use crate::LuaRef;

    impl<A, R> Serialize for Function<A, R>
    where
        A: IntoLuaMulti,
        R: FromLuaMulti,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_f32(self.lua_ref as f32)
        }
    }

    impl<'de, A, R> Deserialize<'de> for Function<A, R>
    where
        A: IntoLuaMulti,
        R: FromLuaMulti,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use std::marker::PhantomData;

            struct FunctionVisitor<A, R>(PhantomData<A>, PhantomData<R>);

            impl<A, R> Visitor<'_> for FunctionVisitor<A, R>
            where
                A: IntoLuaMulti,
                R: FromLuaMulti,
            {
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
