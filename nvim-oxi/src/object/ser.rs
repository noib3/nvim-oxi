use nvim_types::{
    collection::Collection,
    dictionary::KeyValuePair,
    object::Object,
};
use serde::ser::{self, Impossible};

use crate::{Error, Result};

/// A struct for serializing Rust values into Neovim `Object`s.
#[derive(Debug)]
pub(super) struct Serializer;

macro_rules! unsupported {
    ($name:literal) => {
        crate::Error::SerializeError(
            concat!($name, " not supported").to_string(),
        )
    };
}

pub(super) struct ArraySerializer {
    inner: Vec<Object>,
}

impl ser::SerializeSeq for ArraySerializer {
    type Error = Error;
    type Ok = Object;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.inner.push(value);
        Ok(())
    }

    fn end(self) -> Result<Object> {
        let collection = Collection::from(self.inner);
        Ok(Object::from(collection))
    }
}

impl ser::SerializeTuple for ArraySerializer {
    type Error = Error;
    type Ok = Object;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.inner.push(value);
        Ok(())
    }

    fn end(self) -> Result<Object> {
        let collection = Collection::from(self.inner);
        Ok(Object::from(collection))
    }
}

pub(super) struct DictionarySerializer {
    inner: Vec<KeyValuePair>,
}

impl ser::SerializeMap for DictionarySerializer {
    type Error = Error;
    type Ok = Object;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(unsupported!("SerializeMap::serialize_key"))
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(unsupported!("SerializeMap::serialize_value"))
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<()>
    where
        K: serde::Serialize,
        V: serde::Serialize,
    {
        let key = key.serialize(Serializer)?;
        let key = String::try_from(key).map_err(|_| unsupported!("non-string keys"))?;
        let value = value.serialize(Serializer)?;
        self.inner.push(KeyValuePair::from((key, value)));
        Ok(())
    }

    fn end(self) -> Result<Object> {
        let collection = Collection::from(self.inner);
        Ok(Object::from(collection))
    }
}

impl ser::SerializeStruct for DictionarySerializer {
    type Error = Error;
    type Ok = Object;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.inner.push(KeyValuePair::from((key, value)));
        Ok(())
    }

    fn end(self) -> Result<Object> {
        let collection = Collection::from(self.inner);
        Ok(Object::from(collection))
    }
}

impl ser::Serializer for Serializer {
    type Error = Error;
    type Ok = Object;
    type SerializeMap = DictionarySerializer;
    type SerializeSeq = ArraySerializer;
    type SerializeStruct = DictionarySerializer;
    type SerializeStructVariant = Impossible<Object, Error>;
    type SerializeTuple = ArraySerializer;
    type SerializeTupleStruct = Impossible<Object, Error>;
    type SerializeTupleVariant = Impossible<Object, Error>;

    fn serialize_bool(self, v: bool) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_u64(self, _v: u64) -> Result<Object> {
        Err(unsupported!("u64"))
    }

    fn serialize_f32(self, v: f32) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_char(self, v: char) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_str(self, v: &str) -> Result<Object> {
        Ok(Object::from(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Object> {
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for b in v {
            ser::SerializeSeq::serialize_element(&mut seq, b)?;
        }
        ser::SerializeSeq::end(seq)
    }

    fn serialize_none(self) -> Result<Object> {
        Ok(Object::nil())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Object>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Object> {
        // maybe just serialize to nil?
        Err(unsupported!("unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Object> {
        Err(unsupported!("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Object> {
        Err(unsupported!("unit variant"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Object>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Object>
    where
        T: serde::Serialize,
    {
        Err(unsupported!("newtype variant"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let inner = if let Some(len) = len {
            Vec::with_capacity(len)
        } else {
            Vec::new()
        };
        Ok(ArraySerializer { inner })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(ArraySerializer { inner: Vec::with_capacity(len) })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        // reconsider?
        Err(unsupported!("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        // reconsider?
        Err(unsupported!("tuple variant"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let inner = if let Some(len) = len {
            Vec::with_capacity(len)
        } else {
            Vec::new()
        };
        Ok(DictionarySerializer { inner })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(DictionarySerializer { inner: Vec::with_capacity(len) })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(unsupported!("struct variant"))
    }
}
