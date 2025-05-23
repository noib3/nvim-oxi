use serde::ser::{self, Error};

use super::SerializeError;
use crate::Object;
use crate::conversion::FromObject;

/// A struct for serializing Rust values into Neovim `Object`s.
#[non_exhaustive]
#[derive(Debug)]
pub struct Serializer;

impl Serializer {
    #[allow(dead_code)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

macro_rules! serialize_into {
    ($name:ident, $type:ty) => {
        #[inline]
        fn $name(self, value: $type) -> Result<Object, Self::Error> {
            Ok(value.into())
        }
    };
}

macro_rules! serialize_big_int {
    ($name:ident, $type:ty) => {
        #[inline]
        fn $name(self, value: $type) -> Result<Object, Self::Error> {
            i64::try_from(value).map_err(Self::Error::custom).map(Object::from)
        }
    };
}

macro_rules! serialize_nil {
    ($name:ident) => {
        #[inline]
        fn $name(self) -> Result<Object, Self::Error> {
            Ok(Object::nil())
        }
    };
}

impl ser::Serializer for Serializer {
    type Ok = Object;
    type Error = SerializeError;

    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeSeq;
    type SerializeTupleStruct = SerializeSeq;
    type SerializeTupleVariant = SerializeSeq;

    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeMap;

    serialize_into!(serialize_bool, bool);
    serialize_into!(serialize_i8, i8);
    serialize_into!(serialize_u8, u8);
    serialize_into!(serialize_i16, i16);
    serialize_into!(serialize_u16, u16);
    serialize_into!(serialize_i32, i32);
    serialize_into!(serialize_u32, u32);
    serialize_into!(serialize_i64, i64);
    serialize_into!(serialize_f64, f64);
    serialize_into!(serialize_char, char);
    serialize_into!(serialize_str, &str);

    serialize_big_int!(serialize_u64, u64);
    serialize_big_int!(serialize_i128, i128);
    serialize_big_int!(serialize_u128, u128);

    serialize_nil!(serialize_none);
    serialize_nil!(serialize_unit);

    // 32bit floats are serialized into references to Lua functions, as
    // described in `super::de`.
    #[inline]
    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Object::from_luaref(value as i32))
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(crate::String::from_bytes(value).into())
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Object, Self::Error> {
        Ok(Object::nil())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.unwrap_or_default();
        Ok(SerializeSeq { items: Vec::with_capacity(len) })
    }

    #[inline]
    fn serialize_tuple(
        self,
        len: usize,
    ) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        let len = len.unwrap_or_default();
        Ok(SerializeMap { key: None, pairs: Vec::with_capacity(len) })
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_map(Some(len))
    }
}

pub struct SerializeSeq {
    items: Vec<Object>,
}

impl ser::SerializeSeq for SerializeSeq {
    type Ok = Object;
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        self.items.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Object::from_iter(self.items))
    }
}

macro_rules! serialize_seq {
    ($trait:ident, $fn:ident) => {
        impl ser::$trait for SerializeSeq {
            type Ok = Object;
            type Error = SerializeError;

            fn $fn<T>(&mut self, value: &T) -> Result<(), Self::Error>
            where
                T: ser::Serialize + ?Sized,
            {
                ser::SerializeSeq::serialize_element(self, value)
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                ser::SerializeSeq::end(self)
            }
        }
    };
}

serialize_seq!(SerializeTuple, serialize_element);
serialize_seq!(SerializeTupleStruct, serialize_field);
serialize_seq!(SerializeTupleVariant, serialize_field);

pub struct SerializeMap {
    key: Option<crate::String>,
    pairs: Vec<(crate::String, Object)>,
}

impl ser::SerializeMap for SerializeMap {
    type Ok = Object;
    type Error = SerializeError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        let a = key.serialize(Serializer)?;
        // TODO: don't unwrap
        self.key = Some(
            crate::String::from_object(a).map_err(SerializeError::custom)?,
        );
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        let key = self.key.take().expect("value serialized before key");
        let obj = value.serialize(Serializer)?;
        self.pairs.push((key, obj));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Object::from_iter(self.pairs))
    }
}

macro_rules! serialize_map {
    ($trait:ident) => {
        impl ser::$trait for SerializeMap {
            type Ok = Object;
            type Error = SerializeError;

            fn serialize_field<T>(
                &mut self,
                key: &'static str,
                value: &T,
            ) -> Result<(), Self::Error>
            where
                T: ser::Serialize + ?Sized,
            {
                ser::SerializeMap::serialize_key(self, key)?;
                ser::SerializeMap::serialize_value(self, value)
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                ser::SerializeMap::end(self)
            }
        }
    };
}

serialize_map!(SerializeStruct);
serialize_map!(SerializeStructVariant);
