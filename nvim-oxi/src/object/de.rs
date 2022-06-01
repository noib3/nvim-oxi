use std::mem::ManuallyDrop;
use std::string::String as StdString;

use nvim_types::object::{Object, ObjectType};
use serde::de;

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
            kObjectTypeString => match StdString::try_from(unsafe {
                ManuallyDrop::into_inner(self.obj.data.string)
            }) {
                Ok(str) => visitor.visit_str(&str),
                Err(_) => todo!(),
            },
            kObjectTypeArray => self.deserialize_seq(visitor),
            kObjectTypeDictionary => self.deserialize_map(visitor),
            kObjectTypeLuaRef => todo!(),
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
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        use ObjectType::*;
        match self.obj.r#type {
            kObjectTypeArray => visitor.visit_seq(&mut SeqDeserializer {
                iter: unsafe { ManuallyDrop::into_inner(self.obj.data.array) }
                    .into_iter(),
            }),

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
    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
