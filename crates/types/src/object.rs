use std::borrow::Cow;
use std::ffi::c_int;
use std::mem::ManuallyDrop;

use lua::{ffi::*, Poppable, Pushable};
use luajit as lua;

use crate::{
    Array,
    Boolean,
    Dictionary,
    Float,
    Function,
    Integer,
    LuaRef,
    NonOwning,
    NvimStr,
    String as NvimString,
};

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L109-L120
//
/// Binding to a Neovim object.
///
/// Represents any valid Neovim type.
#[repr(C)]
pub struct Object {
    ty: ObjectKind,
    data: ObjectData,
}

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L94-L107
//
/// Specifies the kind of a Neovim [`Object`].
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
    Buffer,
    Window,
    TabPage,
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
            Self::Buffer => "buffer",
            Self::Window => "window",
            Self::TabPage => "tabpage",
        }
    }
}

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L111-L119
#[repr(C)]
union ObjectData {
    boolean: Boolean,
    integer: Integer,
    float: Float,
    string: ManuallyDrop<NvimString>,
    array: ManuallyDrop<Array>,
    dictionary: ManuallyDrop<Dictionary>,
    luaref: LuaRef,
}

impl Default for Object {
    fn default() -> Object {
        Object::nil()
    }
}

impl core::fmt::Debug for Object {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let field: &dyn core::fmt::Debug = match self.ty {
            ObjectKind::Nil => return f.write_str("nil"),

            ObjectKind::Boolean => unsafe { &self.data.boolean },

            ObjectKind::Integer
            | ObjectKind::Buffer
            | ObjectKind::Window
            | ObjectKind::TabPage => unsafe { &self.data.integer },

            ObjectKind::Float => unsafe { &self.data.float },

            ObjectKind::String => unsafe { &*self.data.string },

            ObjectKind::Array => unsafe { &*self.data.array },

            ObjectKind::Dictionary => unsafe { &*self.data.dictionary },

            ObjectKind::LuaRef => {
                let luaref = unsafe { self.data.luaref };
                return write!(f, "Object(LuaRef({luaref}))");
            },
        };

        field.fmt(f)
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
    pub fn from_luaref(luaref: LuaRef) -> Self {
        Self { ty: ObjectKind::LuaRef, data: ObjectData { luaref } }
    }

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

    /// Returns the boolean stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying value
    /// without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a
    /// [`Boolean`][ObjectKind::Boolean]. Calling this method on an `Object`
    /// with any other kind may result in undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline(always)]
    pub unsafe fn as_boolean_unchecked(&self) -> bool {
        debug_assert!(self.ty == ObjectKind::Boolean, "{:?}", self.ty);
        self.data.boolean
    }

    /// Returns a mutable reference to the boolean stored in this
    /// [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying
    /// value without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a
    /// [`Boolean`][ObjectKind::Boolean]. Calling this method on an `Object`
    /// with any other kind may result in undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline(always)]
    pub unsafe fn as_boolean_unchecked_mut(&mut self) -> &mut bool {
        debug_assert!(self.ty == ObjectKind::Boolean, "{:?}", self.ty);
        &mut self.data.boolean
    }

    /// Returns the integer stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying
    /// value without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be one of
    /// [`Integer`][ObjectKind::Integer], [`Buffer`][ObjectKind::Buffer],
    /// [`Window`][ObjectKind::Window], or [`TabPage`][ObjectKind::TabPage].
    /// Calling this method on an `Object` with any other kind may result in
    /// undefined
    /// behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline(always)]
    pub unsafe fn as_integer_unchecked(&self) -> Integer {
        debug_assert!(
            matches!(
                self.ty,
                ObjectKind::Integer
                    | ObjectKind::Buffer
                    | ObjectKind::Window
                    | ObjectKind::TabPage
            ),
            "{:?}",
            self.ty
        );
        self.data.integer
    }

    /// Returns a mutable reference to the integer stored in this
    /// [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying
    /// value without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be one of
    /// [`Integer`][ObjectKind::Integer], [`Buffer`][ObjectKind::Buffer],
    /// [`Window`][ObjectKind::Window], or [`TabPage`][ObjectKind::TabPage].
    /// Calling this method on an `Object` with any other kind may result in
    /// undefined
    /// behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline(always)]
    pub unsafe fn as_integer_unchecked_mut(&mut self) -> &mut Integer {
        debug_assert!(
            matches!(
                self.ty,
                ObjectKind::Integer
                    | ObjectKind::Buffer
                    | ObjectKind::Window
                    | ObjectKind::TabPage
            ),
            "{:?}",
            self.ty
        );
        &mut self.data.integer
    }

    /// Returns the float stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying
    /// value without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a [`Float`][ObjectKind::Float].
    /// Calling this method on an `Object` with any other kind may result in
    /// undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline(always)]
    pub unsafe fn as_float_unchecked(&self) -> Float {
        debug_assert!(self.ty == ObjectKind::Float, "{:?}", self.ty);
        self.data.float
    }

    /// Returns a mutable reference to the float stored in this
    /// [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying
    /// value without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a [`Float`][ObjectKind::Float].
    /// Calling this method on an `Object` with any other kind may result in
    /// undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline(always)]
    pub unsafe fn as_float_unchecked_mut(&mut self) -> &mut Float {
        debug_assert!(self.ty == ObjectKind::Float, "{:?}", self.ty);
        &mut self.data.float
    }

    /// Returns the Lua reference stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying
    /// value without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a
    /// [`LuaRef`][ObjectKind::LuaRef]. Calling this method on an `Object` with
    /// any other kind may result in undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline(always)]
    pub unsafe fn as_luaref_unchecked(&self) -> LuaRef {
        debug_assert!(self.ty == ObjectKind::LuaRef, "{:?}", self.ty);
        self.data.luaref
    }

    /// Returns a mutable reference to the Lua reference stored in this
    /// [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying
    /// value without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a
    /// [`LuaRef`][ObjectKind::LuaRef]. Calling this method on an `Object` with
    /// any other kind may result in undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline(always)]
    pub unsafe fn as_luaref_unchecked_mut(&mut self) -> &mut LuaRef {
        debug_assert!(self.ty == ObjectKind::LuaRef, "{:?}", self.ty);
        &mut self.data.luaref
    }

    /// Returns a reference to the string stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying
    /// value without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a
    /// [`String`][ObjectKind::String]. Calling this method on an `Object` with
    /// any other kind may result in undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn as_nvim_str_unchecked(&self) -> NvimStr<'_> {
        debug_assert!(self.ty == ObjectKind::String, "{:?}", self.ty);
        self.data.string.as_nvim_str()
    }

    /// Returns the string stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying value
    /// without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a
    /// [`String`][ObjectKind::String]. Calling this method on an `Object` with
    /// any other kind may result in undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn into_string_unchecked(self) -> NvimString {
        debug_assert!(self.ty == ObjectKind::String, "{:?}", self.ty);
        let string = &self.data.string;
        let string = unsafe {
            NvimString::from_raw_parts(string.as_ptr(), string.len())
        };
        core::mem::forget(self);
        string
    }

    /// Returns a reference to the array stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying value
    /// without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be an [`Array`][ObjectKind::Array].
    /// Calling this method on an `Object` with any other kind may result in
    /// undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn as_array_unchecked(&self) -> &Array {
        debug_assert!(self.ty == ObjectKind::Array, "{:?}", self.ty);
        &self.data.array
    }

    /// Returns a mutable reference to the array stored in this
    /// [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying value
    /// without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be an [`Array`][ObjectKind::Array].
    /// Calling this method on an `Object` with any other kind may result in
    /// undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn as_array_unchecked_mut(&mut self) -> &mut Array {
        debug_assert!(self.ty == ObjectKind::Array, "{:?}", self.ty);
        &mut self.data.array
    }

    /// Returns the array stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying value
    /// without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be an [`Array`][ObjectKind::Array].
    /// Calling this method on an `Object` with any other kind may result in
    /// undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn into_array_unchecked(self) -> Array {
        debug_assert!(self.ty == ObjectKind::Array, "{:?}", self.ty);
        #[allow(clippy::unnecessary_struct_initialization)]
        let array = Array(crate::kvec::KVec { ..self.data.array.0 });
        core::mem::forget(self);
        array
    }

    /// Returns a reference to the dictionary stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying value
    /// without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a
    /// [`Dictionary`][ObjectKind::Dictionary]. Calling this method on an
    /// `Object` with any other kind may result in undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn as_dictionary_unchecked(&self) -> &Dictionary {
        debug_assert!(self.ty == ObjectKind::Dictionary, "{:?}", self.ty);
        &self.data.dictionary
    }

    /// Returns a mutable reference to the dictionary stored in this
    /// [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying value
    /// without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a
    /// [`Dictionary`][ObjectKind::Dictionary]. Calling this method on an
    /// `Object` with any other kind may result in undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn as_dictionary_unchecked_mut(&mut self) -> &mut Dictionary {
        debug_assert!(self.ty == ObjectKind::Dictionary, "{:?}", self.ty);
        &mut self.data.dictionary
    }

    /// Returns the dictionary stored in this [`Object`].
    ///
    /// This is a zero-cost method that directly accesses the underlying value
    /// without performing any runtime checks.
    ///
    /// # Safety
    ///
    /// This `Object`'s [`ObjectKind`] must be a
    /// [`Dictionary`][ObjectKind::Dictionary]. Calling this method on an
    /// `Object` with any other kind may result in undefined behavior.
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn into_dictionary_unchecked(self) -> Dictionary {
        debug_assert!(self.ty == ObjectKind::Dictionary, "{:?}", self.ty);
        #[allow(clippy::unnecessary_struct_initialization)]
        let dict = Dictionary(crate::kvec::KVec { ..self.data.dictionary.0 });
        core::mem::forget(self);
        dict
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
            ObjectKind::Integer
            | ObjectKind::Buffer
            | ObjectKind::Window
            | ObjectKind::TabPage => clone_copy!(self, integer),
            ObjectKind::Float => clone_copy!(self, float),
            ObjectKind::String => clone_drop!(self, string, NvimString),
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
        match self.ty {
            ObjectKind::String => unsafe {
                ManuallyDrop::drop(&mut self.data.string)
            },

            ObjectKind::Array => unsafe {
                ManuallyDrop::drop(&mut self.data.array)
            },

            ObjectKind::Dictionary => unsafe {
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
                Integer | Buffer | Window | TabPage => {
                    lhs.integer == rhs.integer
                },
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

from_man_drop!(NvimString, String, string);
from_man_drop!(Array, Array, array);
from_man_drop!(Dictionary, Dictionary, dictionary);

impl<A, R> From<Function<A, R>> for Object {
    fn from(fun: Function<A, R>) -> Self {
        Self::from_luaref(fun.lua_ref)
    }
}

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
        NvimString::from(s.as_str()).into()
    }
}

impl From<&str> for Object {
    #[inline(always)]
    fn from(s: &str) -> Self {
        NvimString::from(s).into()
    }
}

impl From<char> for Object {
    #[inline(always)]
    fn from(ch: char) -> Self {
        NvimString::from(ch).into()
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

impl From<Cow<'_, str>> for Object {
    fn from(moo: Cow<'_, str>) -> Self {
        NvimString::from(moo.as_ref()).into()
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
    NvimString: From<K>,
    Object: From<V>,
{
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Dictionary::from_iter(iter).into()
    }
}

impl Pushable for Object {
    unsafe fn push(self, lstate: *mut State) -> Result<c_int, lua::Error> {
        match self.kind() {
            ObjectKind::Nil => ().push(lstate),
            ObjectKind::Boolean => self.as_boolean_unchecked().push(lstate),
            ObjectKind::Integer
            | ObjectKind::Buffer
            | ObjectKind::Window
            | ObjectKind::TabPage => self.as_integer_unchecked().push(lstate),
            ObjectKind::Float => self.as_float_unchecked().push(lstate),
            ObjectKind::String => self.into_string_unchecked().push(lstate),
            ObjectKind::Array => self.into_array_unchecked().push(lstate),
            ObjectKind::Dictionary => {
                self.into_dictionary_unchecked().push(lstate)
            },
            ObjectKind::LuaRef => {
                Function::<(), ()>::from_ref(self.as_luaref_unchecked())
                    .push(lstate)
            },
        }
    }
}

impl Poppable for Object {
    unsafe fn pop(lstate: *mut State) -> Result<Self, lua::Error> {
        if lua_gettop(lstate) == 0 {
            return Ok(Self::nil());
        }

        match lua_type(lstate, -1) {
            LUA_TNIL => <()>::pop(lstate).map(Into::into),

            LUA_TBOOLEAN => bool::pop(lstate).map(Into::into),

            LUA_TNUMBER => {
                let n = Number::pop(lstate)?;

                // This checks that the number is in the range (i32::MIN,
                // i32::MAX) andd that it has no fractional component.
                if n == (n as c_int) as Number {
                    Ok(Object::from(n as c_int))
                } else {
                    Ok(Object::from(n))
                }
            },

            LUA_TSTRING => NvimString::pop(lstate).map(Into::into),

            LUA_TTABLE => {
                if lua::utils::is_table_array(lstate, -1) {
                    Array::pop(lstate).map(Into::into)
                } else {
                    Dictionary::pop(lstate).map(Into::into)
                }
            },

            LUA_TFUNCTION => Function::<(), ()>::pop(lstate).map(Into::into),

            LUA_TNONE => Err(lua::Error::PopEmptyStack),

            LUA_TLIGHTUSERDATA | LUA_TUSERDATA | LUA_TTHREAD => {
                let typename = lua::utils::debug_type(lstate, -1);
                lua_pop(lstate, 1);

                Err(lua::Error::pop_error(
                    "Object",
                    format!("unexpected value of type {typename}"),
                ))
            },

            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use std::fmt;

    use serde::de::{self, Deserialize};

    use crate::{
        Array,
        Dictionary,
        Integer,
        LuaRef,
        Object,
        String as NvimString,
    };

    impl<'de> Deserialize<'de> for Object {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct ObjectVisitor;

            macro_rules! visit_into {
                ($fn_name:ident, $ty:ty) => {
                    fn $fn_name<E>(self, value: $ty) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Ok(Object::from(value))
                    }
                };
            }

            impl<'de> de::Visitor<'de> for ObjectVisitor {
                type Value = Object;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str("either a string of a byte vector")
                }

                fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Object::nil())
                }

                fn visit_bytes<E>(self, b: &[u8]) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(NvimString::from_bytes(b).into())
                }

                fn visit_u64<E>(self, n: u64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Integer::try_from(n).map(Object::from).map_err(E::custom)
                }

                fn visit_f32<E>(self, f: f32) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Object::from_luaref(f as LuaRef))
                }

                fn visit_seq<A>(
                    self,
                    mut seq: A,
                ) -> Result<Self::Value, A::Error>
                where
                    A: de::SeqAccess<'de>,
                {
                    let mut vec = Vec::<Object>::with_capacity(
                        seq.size_hint().unwrap_or_default(),
                    );

                    while let Some(obj) = seq.next_element::<Object>()? {
                        vec.push(obj);
                    }

                    Ok(vec.into_iter().collect::<Array>().into())
                }

                fn visit_map<A>(
                    self,
                    mut map: A,
                ) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>,
                {
                    let mut vec = Vec::<(NvimString, Object)>::with_capacity(
                        map.size_hint().unwrap_or_default(),
                    );

                    while let Some(pair) =
                        map.next_entry::<NvimString, Object>()?
                    {
                        vec.push(pair);
                    }

                    Ok(vec.into_iter().collect::<Dictionary>().into())
                }

                visit_into!(visit_bool, bool);
                visit_into!(visit_i8, i8);
                visit_into!(visit_u8, u8);
                visit_into!(visit_i16, i16);
                visit_into!(visit_u16, u16);
                visit_into!(visit_i32, i32);
                visit_into!(visit_u32, u32);
                visit_into!(visit_i64, i64);
                visit_into!(visit_f64, f64);
                visit_into!(visit_str, &str);
            }

            deserializer.deserialize_any(ObjectVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversion::FromObject;

    #[test]
    fn debug_nil() {
        assert_eq!(format!("{:?}", Object::nil()), "nil");
    }

    #[test]
    fn std_string_to_obj_and_back() {
        let str = String::from("foo");
        let obj = Object::from(str.clone());
        let str_again = String::from_object(obj);
        assert!(str_again.is_ok());
        assert_eq!(str, str_again.unwrap());
    }
}
