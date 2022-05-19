use std::fmt;
use std::mem::ManuallyDrop;

use super::array::Array;
use super::dictionary::Dictionary;
use super::string::String;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L115
#[repr(C)]
pub struct Object {
    pub(crate) r#type: ObjectType,
    pub(crate) data: ObjectData,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L100
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum ObjectType {
    kObjectTypeNil = 0,
    kObjectTypeBoolean,
    kObjectTypeInteger,
    kObjectTypeFloat,
    kObjectTypeString,
    kObjectTypeArray,
    kObjectTypeDictionary,
    kObjectTypeLuaRef,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L117
#[repr(C)]
pub union ObjectData {
    pub(crate) boolean: bool,
    pub(crate) integer: i64,
    pub(crate) float: f64,
    pub(crate) string: ManuallyDrop<String>,
    pub(crate) array: ManuallyDrop<Array>,
    pub(crate) dictionary: ManuallyDrop<Dictionary>,
    pub(crate) luaref: isize,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ObjectType::*;

        let mut dbg = f.debug_struct("Object");
        dbg.field("type", &self.r#type);

        match self.r#type {
            kObjectTypeNil => dbg.field("data", &"nil"),

            kObjectTypeBoolean => {
                dbg.field("data", &unsafe { self.data.boolean })
            },

            kObjectTypeInteger => {
                dbg.field("data", &unsafe { self.data.integer })
            },

            kObjectTypeFloat => dbg.field("data", &unsafe { self.data.float }),

            kObjectTypeString => {
                // dbg.field("data", &unsafe { self.data.string })
                dbg.field("data", &"todo")
            },

            kObjectTypeArray => {
                // dbg.field("data", &unsafe { self.data.array })
                dbg.field("data", &"todo")
            },

            kObjectTypeDictionary => {
                // dbg.field("data", &unsafe { self.data.dictionary })
                dbg.field("data", &"todo")
            },

            kObjectTypeLuaRef => {
                // dbg.field("data", &unsafe { self.data.luaref })
                dbg.field("data", &"todo")
            },
        };

        dbg.finish()
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        use ObjectType::*;
        match self.r#type {
            kObjectTypeString => unsafe {
                ManuallyDrop::drop(&mut self.data.string)
            },

            kObjectTypeArray => unsafe {
                ManuallyDrop::drop(&mut self.data.array)
            },

            kObjectTypeDictionary => unsafe {
                ManuallyDrop::drop(&mut self.data.dictionary)
            },

            _ => {},
        }
    }
}

macro_rules! impl_from_copy_type {
    ($type:ident, $variant:ident, $data:ident) => {
        impl From<$type> for Object {
            fn from($data: $type) -> Self {
                Self {
                    r#type: ObjectType::$variant,
                    data: ObjectData { $data },
                }
            }
        }
    };
}

macro_rules! impl_from_int {
    ($type:ident) => {
        impl From<$type> for Object {
            fn from(i: $type) -> Self {
                Self::from(i64::from(i))
            }
        }
    };
}

impl_from_copy_type!(bool, kObjectTypeBoolean, boolean);
impl_from_copy_type!(i64, kObjectTypeInteger, integer);

impl_from_int!(i8);
impl_from_int!(u8);
impl_from_int!(i16);
impl_from_int!(u16);
impl_from_int!(i32);
impl_from_int!(u32);

// impl From<bool> for Object {
//     fn from(boolean: bool) -> Self {
//         from_copy_type!(kObjectTypeBoolean, boolean)
//     }
// }

impl From<std::string::String> for Object {
    fn from(string: std::string::String) -> Self {
        String::from_c_string(std::ffi::CString::new(string).unwrap()).into()
    }
}

impl From<String> for Object {
    fn from(string: String) -> Self {
        Self {
            r#type: ObjectType::kObjectTypeString,
            data: ObjectData { string: ManuallyDrop::new(string) },
        }
    }
}

impl<T: Into<Object>> From<Option<T>> for Object {
    fn from(maybe: Option<T>) -> Self {
        match maybe {
            Some(obj) => obj.into(),
            None => Self {
                r#type: ObjectType::kObjectTypeNil,
                data: ObjectData { integer: 0 },
            },
        }
    }
}

impl<T: Into<Object>> From<Vec<T>> for Object {
    fn from(vec: Vec<T>) -> Self {
        Self {
            r#type: ObjectType::kObjectTypeArray,
            data: ObjectData { array: ManuallyDrop::new(Array::from(vec)) },
        }
    }
}
