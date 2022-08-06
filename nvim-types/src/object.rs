use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::mem::ManuallyDrop;
use std::result::Result as StdResult;
use std::string::String as StdString;

use crate::{
    Array,
    Boolean,
    Dictionary,
    Float,
    Integer,
    LuaRef,
    NonOwning,
    String as NvimString,
};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L115
//
/// Represents any type of Neovim object.
#[repr(C)]
pub struct Object {
    r#type: ObjectKind,
    data: ObjectData,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L100
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L117
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

impl Object {
    /// Returns a new nil object.
    #[inline]
    pub const fn nil() -> Self {
        Self { r#type: ObjectKind::Nil, data: ObjectData { integer: 0 } }
    }

    #[inline]
    pub const fn is_nil(&self) -> bool {
        matches!(self.r#type, ObjectKind::Nil)
    }

    #[inline]
    pub const fn is_some(&self) -> bool {
        !self.is_nil()
    }

    #[inline(always)]
    #[doc(hidden)]
    pub fn new_luaref(luaref: LuaRef) -> Self {
        Self { r#type: ObjectKind::LuaRef, data: ObjectData { luaref } }
    }

    /// TODO: docs
    #[inline]
    pub fn kind(&self) -> ObjectKind {
        self.r#type
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

    /// Extracts the inner [`String`] from the object, without checking that the
    /// object actually represents a [`String`].
    #[inline(always)]
    pub unsafe fn into_string_unchecked(self) -> NvimString {
        let str = ManuallyDrop::new(self);
        NvimString { ..*str.data.string }
    }

    /// Extracts the inner [`Array`] from the object, without checking that the
    /// object actually represents an [`Array`].
    #[inline(always)]
    pub unsafe fn into_array_unchecked(self) -> Array {
        let array = ManuallyDrop::new(self);
        Array { ..*array.data.array }
    }

    /// Extracts the inner [`Dictionary`] from the object, without checking that
    /// the object actually represents a [`Dictionary`].
    #[inline(always)]
    pub unsafe fn into_dict_unchecked(self) -> Dictionary {
        let dict = ManuallyDrop::new(self);
        Dictionary { ..*dict.data.dictionary }
    }
}

impl Default for Object {
    #[inline]
    fn default() -> Self {
        Self::nil()
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
        match self.r#type {
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

macro_rules! clone_copy {
    ($self:expr, $field:ident) => {{
        Self {
            r#type: $self.r#type,
            data: ObjectData { $field: unsafe { $self.data.$field } },
        }
    }};
}

macro_rules! clone_drop {
    ($self:expr, $field:ident, $as_type:ident) => {{
        Self {
            r#type: $self.r#type,
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
        match self.r#type {
            ObjectKind::Nil => Self::nil(),
            ObjectKind::Boolean => clone_copy!(self, boolean),
            ObjectKind::Integer => clone_copy!(self, integer),
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
        use ObjectKind::*;
        match self.r#type {
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
        if self.r#type != other.r#type {
            return false;
        };

        let (lhs, rhs) = (&self.data, &other.data);

        unsafe {
            use ObjectKind::*;
            match self.r#type {
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
                Object {
                    r#type: ObjectKind::$variant,
                    data: ObjectData { $data },
                }
            }
        }
    };
}

from_copy!(Boolean, Boolean, boolean);
from_copy!(Integer, Integer, integer);
from_copy!(Float, Float, float);

/// Implements `From<..>` for primitive `ManuallyDrop` types.
macro_rules! from_man_drop {
    ($type:ident, $variant:ident, $data:ident) => {
        impl From<$type> for Object {
            #[inline(always)]
            fn from($data: $type) -> Self {
                Object {
                    r#type: ObjectKind::$variant,
                    data: ObjectData { $data: ManuallyDrop::new($data) },
                }
            }
        }
    };
}

from_man_drop!(NvimString, String, string);
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

impl From<StdString> for Object {
    #[inline(always)]
    fn from(s: StdString) -> Self {
        NvimString::from(s).into()
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

#[derive(thiserror::Error, Debug)]
pub enum FromObjectError {
    /// Raised when implementing `TryFrom<Object>` for one of the "primitive"
    /// data types, i.e. a field of the `ObjectData` union.
    #[error("Was expecting a \"{expected:?}\", got \"{actual:?}\" instead")]
    Primitive { expected: ObjectKind, actual: ObjectKind },

    /// Raised when implementing `TryFrom<Object>` for a type that implements
    /// `TryFrom<{type}>`, where `{type}` is a primitive data type. For
    /// example, `TryFrom<StdString>` or `TryFrom<usize>`.
    #[error("Error converting {into} into {primitive:?}: {source}")]
    Secondary {
        primitive: ObjectKind,
        into: &'static str,
        source: Box<dyn StdError + Send + Sync>,
    },
}

impl PartialEq<Self> for FromObjectError {
    fn eq(&self, other: &Self) -> bool {
        use FromObjectError::*;
        match (self, other) {
            (
                Primitive { expected: e1, actual: a1 },
                Primitive { expected: e2, actual: a2 },
            ) => (e1 == e2) && (a1 == a2),

            (
                Secondary { primitive: p1, into: i1, source: _ },
                Secondary { primitive: p2, into: i2, source: _ },
            ) => (p1 == p2) && (i1 == i2),

            _ => false,
        }
    }
}

impl Eq for FromObjectError {}

impl FromObjectError {
    pub fn secondary<E, T>(primitive: ObjectKind, err: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::Secondary {
            primitive,
            into: std::any::type_name::<T>(),
            source: Box::new(err),
        }
    }
}

impl TryFrom<Object> for () {
    type Error = FromObjectError;

    fn try_from(obj: Object) -> StdResult<Self, Self::Error> {
        (matches!(obj.r#type, ObjectKind::Nil)).then_some(()).ok_or_else(
            || FromObjectError::Primitive {
                expected: ObjectKind::Nil,
                actual: obj.r#type,
            },
        )
    }
}

/// Implements `TryFrom<Object>` for primitive `Copy` types.
macro_rules! try_from_copy {
    ($type:ident, $variant:ident, $data:ident) => {
        impl TryFrom<Object> for $type {
            type Error = FromObjectError;

            fn try_from(obj: Object) -> StdResult<Self, Self::Error> {
                (matches!(obj.r#type, ObjectKind::$variant))
                    .then_some(unsafe { obj.data.$data })
                    .ok_or_else(|| FromObjectError::Primitive {
                        expected: ObjectKind::$variant,
                        actual: obj.r#type,
                    })
            }
        }
    };
}

try_from_copy!(Boolean, Boolean, boolean);
try_from_copy!(Integer, Integer, integer);
try_from_copy!(Float, Float, float);

/// Implements `TryFrom<Object>` for primitive `ManuallyDrop` types.
macro_rules! try_from_man_drop {
    ($type:ident, $variant:ident, $into_inner:ident) => {
        impl TryFrom<Object> for $type {
            type Error = FromObjectError;

            fn try_from(obj: Object) -> StdResult<Self, Self::Error> {
                let ty = obj.r#type;
                (matches!(ty, ObjectKind::$variant))
                    .then_some(unsafe { obj.$into_inner() })
                    .ok_or_else(|| FromObjectError::Primitive {
                        expected: ObjectKind::$variant,
                        actual: ty,
                    })
            }
        }
    };
}

try_from_man_drop!(NvimString, String, into_string_unchecked);
try_from_man_drop!(Array, Array, into_array_unchecked);
try_from_man_drop!(Dictionary, Dictionary, into_dict_unchecked);

/// Implements `TryFrom<Object>` for a type that implements `TryFrom<{prim}>`,
/// where `{prim}` is one of the primitive data types.
macro_rules! try_from_prim {
    ($orig:ident, $type:ty, $variant:ident) => {
        impl TryFrom<Object> for $type {
            type Error = FromObjectError;

            fn try_from(obj: Object) -> StdResult<Self, Self::Error> {
                $orig::try_from(obj)?.try_into().map_err(|err| {
                    FromObjectError::secondary::<_, $type>(
                        ObjectKind::$variant,
                        err,
                    )
                })
            }
        }
    };
}

try_from_prim!(Integer, i8, Integer);
try_from_prim!(Integer, u8, Integer);
try_from_prim!(Integer, i16, Integer);
try_from_prim!(Integer, u16, Integer);
try_from_prim!(Integer, i32, Integer);
try_from_prim!(Integer, u32, Integer);
try_from_prim!(Integer, u64, Integer);
try_from_prim!(Integer, i128, Integer);
try_from_prim!(Integer, u128, Integer);
try_from_prim!(Integer, isize, Integer);
try_from_prim!(Integer, usize, Integer);

try_from_prim!(NvimString, StdString, String);

#[cfg(feature = "serde")]
use serde::de;

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Object {
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
                Ok(NvimString::from_bytes(b.to_owned()).into())
            }

            fn visit_f32<E>(self, f: f32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Object::new_luaref(f as LuaRef))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
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

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
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
            // visit_into!(visit_u64, u64);
            visit_into!(visit_str, &str);
            visit_into!(visit_f64, f64);
        }

        deserializer.deserialize_any(ObjectVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_string_to_obj_and_back() {
        let str = StdString::from("foo");
        let obj = Object::from(str.clone());
        let str_again = StdString::try_from(obj);
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
        let obj = Object::new_luaref(42);
        assert_eq!("LuaRef(42)", &format!("{obj:?}"));
        assert_eq!("LuaRef(42)", &format!("{obj}"));
    }
}
