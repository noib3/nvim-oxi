use std::borrow::Cow;
use std::ffi::c_int;
use std::fmt;
use std::mem::ManuallyDrop;

use lua::{ffi::*, LuaPoppable, LuaPushable};
use luajit_bindings as lua;

use crate::{
    Array,
    Boolean,
    Dictionary,
    Float,
    Function,
    Integer,
    LuaRef,
    NonOwning,
};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L109
//
/// Represents any valid Neovim type.
#[repr(C)]
pub struct Object {
    ty: ObjectKind,
    data: ObjectData,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L94
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum ObjectKind {
    Nil = 0,
    Boolean,
    Integer,
    Float,
    String,
    Array,
    Dictionary,
    LuaRef,
}

impl ObjectKind {
    pub fn as_static(&self) -> &'static str {
        match self {
            Self::Nil => "nil",
            Self::Boolean => "boolean",
            Self::Integer => "integer",
            Self::Float => "float",
            Self::String => "string",
            Self::Array => "array",
            Self::Dictionary => "dictionary",
            Self::LuaRef => "luaref",
        }
    }
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L111
#[repr(C)]
union ObjectData {
    boolean: Boolean,
    integer: Integer,
    float: Float,
    string: ManuallyDrop<crate::String>,
    array: ManuallyDrop<Array>,
    dictionary: ManuallyDrop<Dictionary>,
    luaref: LuaRef,
}

impl Default for Object {
    fn default() -> Object {
        Object::nil()
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ObjectKind::*;
        match self.ty {
            Nil => f.write_str("()"),
            Boolean => write!(f, "{}", unsafe { self.data.boolean }),
            Integer => write!(f, "{}", unsafe { self.data.integer }),
            Float => write!(f, "{}", unsafe { self.data.float }),
            String => write!(f, "\"{}\"", unsafe { &*self.data.string }),
            Array => write!(f, "{}", unsafe { &*self.data.array }),
            Dictionary => write!(f, "{}", unsafe { &*self.data.dictionary }),
            LuaRef => write!(f, "LuaRef({})", unsafe { self.data.luaref }),
        }
    }
}

impl Object {
    /// Returns a new nil object.
    #[inline]
    pub fn nil() -> Self {
        Self { ty: ObjectKind::Nil, data: ObjectData { integer: 0 } }
    }

    #[inline]
    pub fn is_nil(&self) -> bool {
        matches!(self.ty, ObjectKind::Nil)
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        !self.is_nil()
    }

    #[inline(always)]
    #[doc(hidden)]
    pub fn from_luaref(luaref: LuaRef) -> Self {
        Self { ty: ObjectKind::LuaRef, data: ObjectData { luaref } }
    }

    /// TODO: docs
    #[inline]
    pub fn kind(&self) -> ObjectKind {
        self.ty
    }

    /// Make a non-owning version of this `Object`.
    #[inline]
    #[doc(hidden)]
    pub fn non_owning(&self) -> NonOwning<'_, Self> {
        // Using ptr::read, because can't copy the union.
        unsafe { NonOwning::new(std::ptr::read(self)) }
    }

    /// TODO: docs
    #[inline(always)]
    pub unsafe fn as_boolean_unchecked(&self) -> bool {
        self.data.boolean
    }

    /// TODO: docs
    #[inline(always)]
    pub unsafe fn as_integer_unchecked(&self) -> Integer {
        self.data.integer
    }

    /// TODO: docs
    #[inline(always)]
    pub unsafe fn as_float_unchecked(&self) -> Float {
        self.data.float
    }

    /// TODO: docs
    #[inline(always)]
    #[doc(hidden)]
    pub unsafe fn as_luaref_unchecked(&self) -> LuaRef {
        self.data.luaref
    }

    /// Extracts the inner [`String`] from the object, without checking that
    /// the object actually represents a [`String`].
    pub unsafe fn into_string_unchecked(self) -> crate::String {
        let str = ManuallyDrop::new(self);
        crate::String { ..*str.data.string }
    }

    /// Extracts the inner [`Array`] from the object, without checking that the
    /// object actually represents an [`Array`].
    pub unsafe fn into_array_unchecked(self) -> Array {
        let array = ManuallyDrop::new(self);
        Array { ..*array.data.array }
    }

    /// Extracts the inner [`Dictionary`] from the object, without checking
    /// that the object actually represents a [`Dictionary`].
    pub unsafe fn into_dict_unchecked(self) -> Dictionary {
        let dict = ManuallyDrop::new(self);
        Dictionary { ..*dict.data.dictionary }
    }
}

macro_rules! clone_copy {
    ($self:expr, $field:ident) => {{
        Self {
            ty: $self.ty,
            data: ObjectData { $field: unsafe { $self.data.$field } },
        }
    }};
}

macro_rules! clone_drop {
    ($self:expr, $field:ident, $as_type:ty) => {{
        Self {
            ty: $self.ty,
            data: ObjectData {
                $field: ManuallyDrop::new(
                    unsafe { &$self.data.$field as &$as_type }.clone(),
                ),
            },
        }
    }};
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self.ty {
            ObjectKind::Nil => Self::nil(),
            ObjectKind::Boolean => clone_copy!(self, boolean),
            ObjectKind::Integer => clone_copy!(self, integer),
            ObjectKind::Float => clone_copy!(self, float),
            ObjectKind::String => clone_drop!(self, string, crate::String),
            ObjectKind::Array => clone_drop!(self, array, Array),
            ObjectKind::Dictionary => {
                clone_drop!(self, dictionary, Dictionary)
            },
            ObjectKind::LuaRef => clone_copy!(self, luaref),
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        use ObjectKind::*;
        match self.ty {
            String => unsafe { ManuallyDrop::drop(&mut self.data.string) },

            Array => unsafe { ManuallyDrop::drop(&mut self.data.array) },

            Dictionary => unsafe {
                ManuallyDrop::drop(&mut self.data.dictionary)
            },

            _ => {},
        }
    }
}

impl PartialEq<Self> for Object {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.ty != other.ty {
            return false;
        };

        let (lhs, rhs) = (&self.data, &other.data);

        unsafe {
            use ObjectKind::*;
            match self.ty {
                Nil => true,
                Boolean => lhs.boolean == rhs.boolean,
                Integer => lhs.boolean == rhs.boolean,
                Float => lhs.float == rhs.float,
                String => lhs.string == rhs.string,
                Array => lhs.array == rhs.array,
                Dictionary => lhs.dictionary == rhs.dictionary,
                LuaRef => lhs.luaref == rhs.luaref,
            }
        }
    }
}

impl From<()> for Object {
    fn from(_: ()) -> Self {
        Self::nil()
    }
}

// Implements `From<..>` for primitive `Copy` types.
macro_rules! from_copy {
    ($type:ident, $variant:ident, $data:ident) => {
        impl From<$type> for Object {
            #[inline(always)]
            fn from($data: $type) -> Self {
                Object { ty: ObjectKind::$variant, data: ObjectData { $data } }
            }
        }
    };
}

from_copy!(Boolean, Boolean, boolean);
from_copy!(Integer, Integer, integer);
from_copy!(Float, Float, float);

/// Implements `From<..>` for primitive `ManuallyDrop` types.
macro_rules! from_man_drop {
    ($type:ty, $variant:ident, $data:ident) => {
        impl From<$type> for Object {
            #[inline(always)]
            fn from($data: $type) -> Self {
                Object {
                    ty: ObjectKind::$variant,
                    data: ObjectData { $data: ManuallyDrop::new($data) },
                }
            }
        }
    };
}

from_man_drop!(crate::String, String, string);
from_man_drop!(Array, Array, array);
from_man_drop!(Dictionary, Dictionary, dictionary);

/// Implements `From<..>` for integer types convertible to `Integer`.
macro_rules! from_int {
    ($type:ident) => {
        impl From<$type> for Object {
            #[inline(always)]
            fn from(i: $type) -> Self {
                Integer::from(i).into()
            }
        }
    };
}

from_int!(i8);
from_int!(u8);
from_int!(i16);
from_int!(u16);
from_int!(i32);
from_int!(u32);

impl From<f32> for Object {
    #[inline(always)]
    fn from(n: f32) -> Self {
        Float::from(n).into()
    }
}

impl From<String> for Object {
    #[inline(always)]
    fn from(s: String) -> Self {
        crate::String::from(s).into()
    }
}

impl From<&str> for Object {
    #[inline(always)]
    fn from(s: &str) -> Self {
        crate::String::from(s).into()
    }
}

impl From<char> for Object {
    #[inline(always)]
    fn from(ch: char) -> Self {
        crate::String::from(ch).into()
    }
}

impl<T> From<Option<T>> for Object
where
    Object: From<T>,
{
    #[inline(always)]
    fn from(maybe: Option<T>) -> Self {
        maybe.map(Into::into).unwrap_or_default()
    }
}

impl<T> From<Box<T>> for Object
where
    Object: From<T>,
{
    #[inline(always)]
    fn from(boxed: Box<T>) -> Self {
        (*boxed).into()
    }
}

impl<T> From<Cow<'_, T>> for Object
where
    T: Clone,
    Object: From<T>,
{
    #[inline(always)]
    fn from(moo: Cow<'_, T>) -> Self {
        moo.into_owned().into()
    }
}

impl<T> FromIterator<T> for Object
where
    T: Into<Object>,
{
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Array::from_iter(iter).into()
    }
}

impl<K, V> FromIterator<(K, V)> for Object
where
    crate::String: From<K>,
    Object: From<V>,
{
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Dictionary::from_iter(iter).into()
    }
}

impl<A, R> From<Function<A, R>> for Object {
    fn from(fun: Function<A, R>) -> Self {
        Self::from_luaref(fun.lua_ref)
    }
}

impl LuaPushable for Object {
    unsafe fn push(self, lstate: *mut lua_State) -> Result<c_int, lua::Error> {
        match self.kind() {
            ObjectKind::Nil => ().push(lstate),
            ObjectKind::Boolean => self.as_boolean_unchecked().push(lstate),
            ObjectKind::Integer => self.as_integer_unchecked().push(lstate),
            ObjectKind::Float => self.as_float_unchecked().push(lstate),
            ObjectKind::String => self.into_string_unchecked().push(lstate),
            ObjectKind::Array => self.into_array_unchecked().push(lstate),
            ObjectKind::Dictionary => self.into_dict_unchecked().push(lstate),

            ObjectKind::LuaRef => {
                let index = lua::ffi::LUA_REGISTRYINDEX;
                let lua_ref = self.as_luaref_unchecked();
                lua::ffi::lua_rawgeti(lstate, index, lua_ref);
                Ok(1)
            },
        }
    }
}

impl LuaPoppable for Object {
    const N: c_int = 1;

    unsafe fn pop(lstate: *mut lua_State) -> Result<Self, lua::Error> {
        match lua_type(lstate, -1) {
            LUA_TNIL => <()>::pop(lstate).map(Into::into),

            LUA_TBOOLEAN => <bool>::pop(lstate).map(Into::into),

            LUA_TNUMBER => {
                let n = <lua_Number>::pop(lstate)?;

                // This checks that the number is in the range (i32::MIN,
                // i32::MAX) andd that it has no fractional component.
                if n == (n as c_int) as lua_Number {
                    Ok(Object::from(n as c_int))
                } else {
                    Ok(Object::from(n))
                }
            },

            LUA_TSTRING => crate::String::pop(lstate).map(Into::into),

            LUA_TTABLE => {
                if lua::utils::is_table_array(lstate, -1) {
                    Array::pop(lstate).map(Into::into)
                } else {
                    Dictionary::pop(lstate).map(Into::into)
                }
            },

            LUA_TFUNCTION => {
                let luaref = luaL_ref(lstate, LUA_REGISTRYINDEX);
                Ok(Object::from_luaref(luaref))
            },

            LUA_TNONE => todo!(),

            LUA_TLIGHTUSERDATA | LUA_TUSERDATA | LUA_TTHREAD => {
                let typename = lua::utils::debug_type(lstate, -1);
                lua_pop(lstate, 1);
                todo!()
            },

            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FromObject;

    #[test]
    fn std_string_to_obj_and_back() {
        let str = String::from("foo");
        let obj = Object::from(str.clone());
        let str_again = String::from_obj(obj);
        assert!(str_again.is_ok());
        assert_eq!(str, str_again.unwrap());
    }

    #[test]
    fn print_nil() {
        let obj = Object::nil();
        assert_eq!("()", &format!("{obj:?}"));
        assert_eq!("()", &format!("{obj}"));
    }

    #[test]
    fn print_boolean() {
        let obj = Object::from(true);
        assert_eq!("true", &format!("{obj:?}"));
        assert_eq!("true", &format!("{obj}"));
    }

    #[test]
    fn print_integer() {
        let obj = Object::from(42);
        assert_eq!("42", &format!("{obj:?}"));
        assert_eq!("42", &format!("{obj}"));
    }

    #[test]
    fn print_float() {
        let obj = Object::from(42.1);
        assert_eq!("42.1", &format!("{obj:?}"));
        assert_eq!("42.1", &format!("{obj}"));
    }

    #[test]
    fn print_string() {
        let obj = Object::from("foobar");
        assert_eq!("\"foobar\"", &format!("{obj:?}"));
        assert_eq!("\"foobar\"", &format!("{obj}"));
    }

    #[test]
    fn print_array() {
        let obj = Object::from(Array::from((42.1, true, "foo")));
        assert_eq!("[42.1, true, \"foo\"]", &format!("{obj:?}"));
        assert_eq!("[42.1, true, \"foo\"]", &format!("{obj}"));
    }

    #[test]
    fn print_dict() {
        let obj = Object::from(Dictionary::from_iter([
            ("foo", Object::from("bar")),
            ("baz", Object::from(19)),
        ]));
        assert_eq!("{foo: \"bar\", baz: 19}", &format!("{obj:?}"));
        assert_eq!("{foo: \"bar\", baz: 19}", &format!("{obj}"));
    }

    #[test]
    fn print_luaref() {
        let obj = Object::from_luaref(42);
        assert_eq!("LuaRef(42)", &format!("{obj:?}"));
        assert_eq!("LuaRef(42)", &format!("{obj}"));
    }
}
