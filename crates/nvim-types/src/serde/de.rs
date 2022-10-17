use std::string::String as StdString;

use serde::de::{self, IntoDeserializer};

use super::Result;
use crate::{Object, ObjectKind};

/// A struct used for deserializing Neovim `Object`s into Rust values.
pub struct Deserializer {
    obj: Object,
}

impl Deserializer {
    pub fn new(obj: Object) -> Self {
        Self { obj }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer {
    type Error = super::Error;

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
            bytes byte_buf unit unit_struct identifier ignored_any
    }

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        use ObjectKind::*;
        match self.obj.kind() {
            Nil => visitor.visit_unit(),

            Boolean => {
                visitor.visit_bool(unsafe { self.obj.as_boolean_unchecked() })
            },

            Integer | Buffer | Window | TabPage => {
                visitor.visit_i64(unsafe { self.obj.as_integer_unchecked() })
            },

            Float => unsafe {
                visitor.visit_f64(self.obj.as_float_unchecked())
            },

            String => {
                let string = unsafe { self.obj.into_string_unchecked() };
                match string.as_str() {
                    Ok(str) => visitor.visit_str(str),
                    _ => visitor.visit_bytes(string.as_bytes()),
                }
            },

            Array => self.deserialize_seq(visitor),

            Dictionary => self.deserialize_map(visitor),

            // We map the ref representing the index of the lua function in the
            // Lua registry to `f32`. It's definitely a hack, but Neovim rarely
            // returns a float so it should a good place to store it to avoid
            // collisions.
            LuaRef => unsafe {
                visitor.visit_f32(self.obj.as_luaref_unchecked() as f32)
            },
        }
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.obj.kind() {
            ObjectKind::Nil => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let (variant, obj) = match self.obj.kind() {
            ObjectKind::Dictionary => {
                let mut iter =
                    unsafe { self.obj.into_dict_unchecked() }.into_iter();

                let (variant, value) = match iter.len() {
                    1 => iter.next().expect("checked length"),
                    _ => {
                        return Err(de::Error::invalid_value(
                            de::Unexpected::Map,
                            &"dictionary with a single key-value pair",
                        ))
                    },
                };

                (variant.into_string()?, Some(value))
            },

            ObjectKind::String => (
                unsafe { self.obj.into_string_unchecked() }.into_string()?,
                None,
            ),

            _ => return Err(de::Error::custom("bad enum value")),
        };

        visitor.visit_enum(EnumDeserializer { variant, obj })
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.obj.kind() {
            ObjectKind::Array => {
                let iter =
                    unsafe { self.obj.into_array_unchecked() }.into_iter();
                let mut deserializer = SeqDeserializer { iter };
                visitor.visit_seq(&mut deserializer)
            },

            ty => Err(de::Error::invalid_type(
                de::Unexpected::Other(&format!("{ty:?}")),
                &"array",
            )),
        }
    }

    #[inline]
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.obj.kind() {
            ObjectKind::Dictionary => {
                let iter =
                    unsafe { self.obj.into_dict_unchecked() }.into_iter();
                let mut deserializer = MapDeserializer { iter, obj: None };
                visitor.visit_map(&mut deserializer)
            },

            ty => Err(de::Error::invalid_type(
                de::Unexpected::Other(&format!("{ty:?}")),
                &"dictionary",
            )),
        }
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
}

struct SeqDeserializer {
    iter: crate::ArrayIterator,
}

impl<'de> de::SeqAccess<'de> for SeqDeserializer {
    type Error = super::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(obj) = self.iter.next() {
            return seed.deserialize(Deserializer { obj }).map(Some);
        }

        Ok(None)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

struct MapDeserializer {
    iter: crate::DictIterator,
    obj: Option<Object>,
}

impl<'de> de::MapAccess<'de> for MapDeserializer {
    type Error = super::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some((name, obj)) = self.iter.next() {
            self.obj = Some(obj);
            return seed
                .deserialize(Deserializer { obj: name.into() })
                .map(Some);
        }

        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.obj.take() {
            Some(obj) => seed.deserialize(Deserializer { obj }),
            _ => Err(de::Error::custom("object is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

struct EnumDeserializer {
    variant: StdString,
    obj: Option<Object>,
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer {
    type Error = super::Error;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let deserializer = VariantDeserializer { obj: self.obj };
        seed.deserialize(variant).map(|v| (v, deserializer))
    }
}

struct VariantDeserializer {
    obj: Option<Object>,
}

impl<'de> de::VariantAccess<'de> for VariantDeserializer {
    type Error = super::Error;

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.obj {
            Some(obj) => seed.deserialize(Deserializer { obj }),

            _ => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.obj {
            Some(obj) => de::Deserializer::deserialize_map(
                Deserializer { obj },
                visitor,
            ),

            _ => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.obj {
            Some(obj) => de::Deserializer::deserialize_seq(
                Deserializer { obj },
                visitor,
            ),

            _ => Err(de::Error::invalid_type(
                de::Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn unit_variant(self) -> Result<()> {
        match self.obj {
            None => Ok(()),

            _ => Err(de::Error::invalid_type(
                de::Unexpected::NewtypeVariant,
                &"unit variant",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;
    use crate::{Array, Dictionary};

    fn d(value: impl Into<Object>) -> Result<Object> {
        Object::deserialize(Deserializer::new(value.into()))
            .map_err(Into::into)
    }

    #[test]
    fn deserialize_unit() {
        assert_eq!(Ok(Object::nil()), d(()));
    }

    #[test]
    fn deserialize_bool() {
        assert_eq!(Ok(Object::from(true)), d(true));
    }

    #[test]
    fn deserialize_num() {
        assert_eq!(Ok(Object::from(42)), d(42i32));
        assert_eq!(Ok(Object::from(42)), d(42u8));
        assert_eq!(Ok(Object::from(42)), d(42u32));
    }

    #[test]
    fn deserialize_str() {
        assert_eq!(Ok(Object::from("foobar")), d("foobar"));
        assert_eq!(Ok(Object::from("barfoo")), d(String::from("barfoo")));
    }

    #[test]
    fn deserialize_seq() {
        let arr = Array::from((1, 2, "foo", false));
        assert_eq!(Ok(Object::from(arr.clone())), d(arr));
    }

    #[test]
    fn deserialize_map() {
        let map = Dictionary::from_iter([
            ("foo", Object::from("foo")),
            ("bar", Object::from(false)),
            ("baz", Object::from(Array::from(("foo", "bar", false)))),
        ]);
        assert_eq!(Ok(Object::from(map.clone())), d(map));
    }
}
