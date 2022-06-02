use std::mem::ManuallyDrop;
use std::string::String as StdString;

use nvim_types::object::{Object, ObjectType};
use serde::de::{self, IntoDeserializer};

use crate::Result;

/// A struct used for deserializing Neovim `Object`s into Rust values.
pub(super) struct Deserializer {
    pub(super) obj: Object,
}

impl<'de> de::Deserializer<'de> for Deserializer {
    type Error = crate::Error;

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
            bytes byte_buf unit unit_struct identifier ignored_any
    }

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let data = &self.obj.data;

        use ObjectType::*;
        match self.obj.r#type {
            kObjectTypeNil => visitor.visit_unit(),
            kObjectTypeBoolean => visitor.visit_bool(unsafe { data.boolean }),
            kObjectTypeInteger => visitor.visit_i64(unsafe { data.integer }),
            kObjectTypeFloat => visitor.visit_f64(unsafe { data.float }),

            kObjectTypeString => {
                let string =
                    ManuallyDrop::into_inner(unsafe { self.obj.data.string });

                match string.as_str() {
                    Ok(str) => visitor.visit_str(str),
                    _ => visitor.visit_bytes(string.as_bytes()),
                }
            },

            kObjectTypeArray => self.deserialize_seq(visitor),
            kObjectTypeDictionary => self.deserialize_map(visitor),
            kObjectTypeLuaRef => panic!("what to do here?"),
        }
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        use ObjectType::*;
        match self.obj.r#type {
            kObjectTypeNil => visitor.visit_none(),
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
        use ObjectType::*;
        let (variant, obj) = match self.obj.r#type {
            kObjectTypeDictionary => todo!(),

            kObjectTypeString => (
                ManuallyDrop::into_inner(unsafe { self.obj.data.string })
                    .into_string()?,
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
        use ObjectType::*;
        match self.obj.r#type {
            kObjectTypeArray => {
                let iter =
                    ManuallyDrop::into_inner(unsafe { self.obj.data.array })
                        .into_iter();
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
        use ObjectType::*;
        match self.obj.r#type {
            kObjectTypeDictionary => {
                let iter = ManuallyDrop::into_inner(unsafe {
                    self.obj.data.dictionary
                })
                .into_iter();
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
    iter: nvim_types::array::ArrayIter,
}

impl<'de> de::SeqAccess<'de> for SeqDeserializer {
    type Error = crate::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        while let Some(obj) = self.iter.next() {
            return seed.deserialize(Deserializer { obj }).map(Some);
        }

        Ok(None)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

struct MapDeserializer {
    iter: nvim_types::dictionary::DictIter,
    obj: Option<Object>,
}

impl<'de> de::MapAccess<'de> for MapDeserializer {
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        while let Some((name, obj)) = self.iter.next() {
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
    type Error = crate::Error;
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
    type Error = crate::Error;

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
