//! [`Deserialize`](serde::Deserialize) implementations for various types
//! defined in this crate.

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor};

use crate::{Array, Dictionary, Function, Integer, LuaRef, Object};

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

        deserializer.deserialize_f32(FunctionVisitor(PhantomData, PhantomData))
    }
}

impl<'de> Deserialize<'de> for crate::String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringVisitor;

        impl<'de> Visitor<'de> for StringVisitor {
            type Value = crate::String;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("either a string of a byte vector")
            }

            fn visit_bytes<E>(self, b: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(crate::String::from_bytes(b.to_owned()))
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(crate::String::from(s))
            }
        }

        deserializer.deserialize_str(StringVisitor)
    }
}

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
                Ok(crate::String::from_bytes(b.to_owned()).into())
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
                let mut vec = Vec::<(crate::String, Object)>::with_capacity(
                    map.size_hint().unwrap_or_default(),
                );

                while let Some(pair) =
                    map.next_entry::<crate::String, Object>()?
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
