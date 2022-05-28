use std::fmt;
use std::mem::ManuallyDrop;

use super::array::Array;
use super::dictionary::Dictionary;
use super::error::ConversionError;
use super::string::NvimString;
use crate::LuaRef;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L115
#[repr(C)]
pub struct Object {
    pub r#type: ObjectType,
    pub data: ObjectData,
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
    pub boolean: bool,
    pub integer: i64,
    pub float: f64,
    pub string: ManuallyDrop<NvimString>,
    pub array: ManuallyDrop<Array>,
    pub dictionary: ManuallyDrop<Dictionary>,
    pub luaref: LuaRef,
}

impl Object {
    #[inline]
    const fn nil() -> Self {
        Self {
            r#type: ObjectType::kObjectTypeNil,
            data: ObjectData { integer: 0 },
        }
    }

    #[inline]
    pub fn is_nil(&self) -> bool {
        matches!(self.r#type, ObjectType::kObjectTypeNil)
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dbg = f.debug_struct("Object");
        dbg.field("type", &self.r#type);

        use ObjectType::*;
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
                dbg.field("data", unsafe { &self.data.string })
            },

            kObjectTypeArray => dbg.field("data", unsafe { &self.data.array }),

            kObjectTypeDictionary => {
                dbg.field("data", unsafe { &self.data.dictionary })
            },

            kObjectTypeLuaRef => dbg.field(
                "data",
                &format!("Function({})", unsafe { self.data.luaref }),
            ),
        };

        dbg.finish()
    }
}

macro_rules! impl_clone_for_copy {
    ($self:expr, $field:ident) => {{
        Self {
            r#type: $self.r#type,
            data: ObjectData { $field: unsafe { $self.data.$field } },
        }
    }};
}

macro_rules! impl_clone_for_clone {
    ($self:expr, $field:ident) => {{
        Self {
            r#type: $self.r#type,
            data: ObjectData {
                $field: ManuallyDrop::new(
                    unsafe { &$self.data.$field }.clone(),
                ),
            },
        }
    }};
}

// impl Clone for Object {
//     fn clone(&self) -> Self {
//         use ObjectType::*;
//         match self.r#type {
//             kObjectTypeNil => Self::nil(),
//             kObjectTypeBoolean => impl_clone_for_copy!(self, boolean),
//             kObjectTypeInteger => impl_clone_for_copy!(self, integer),
//             kObjectTypeFloat => impl_clone_for_copy!(self, float),
//             // kObjectTypeString => impl_clone_for_clone!(self, string),
//             // kObjectTypeArray => impl_clone_for_clone!(self, array),
//             // kObjectTypeDictionary => impl_clone_for_clone!(self, dictionary),
//             kObjectTypeString => {
//                 let value: &NvimString = unsafe { &self.data.string };
//                 Self {
//                     r#type: self.r#type,
//                     data: ObjectData {
//                         string: ManuallyDrop::new(value.clone()),
//                     },
//                 }
//             },

//             kObjectTypeArray => {
//                 let value: &Array = unsafe { &self.data.array };
//                 Self {
//                     r#type: self.r#type,
//                     data: ObjectData {
//                         array: ManuallyDrop::new(value.clone()),
//                     },
//                 }
//             },

//             kObjectTypeDictionary => {
//                 let value: &Dictionary = unsafe { &self.data.dictionary };
//                 Self {
//                     r#type: self.r#type,
//                     data: ObjectData {
//                         dictionary: ManuallyDrop::new(value.clone()),
//                     },
//                 }
//             },

//             kObjectTypeLuaRef => impl_clone_for_copy!(self, luaref),
//         }
//     }
// }

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
// impl_from_int!(i32);
impl_from_int!(u32);

// impl PartialEq for Object {
//     fn eq(&self, other: &Self) -> bool {
//         self.r#type == other.r#type
//             && unsafe {
//                 let (sd, od) = (&self.data, &other.data);
//                 use ObjectType::*;
//                 match self.r#type {
//                     kObjectTypeNil => true,
//                     kObjectTypeBoolean => sd.boolean == od.boolean,
//                     kObjectTypeInteger => sd.integer == od.integer,
//                     kObjectTypeFloat => sd.float == od.float,
//                     kObjectTypeString => sd.string == od.string,
//                     kObjectTypeArray => sd.array == od.array,
//                     kObjectTypeDictionary => sd.dictionary == od.dictionary,
//                     kObjectTypeLuaref => sd.luaref == od.luaref,
//                 }
//             }
//     }
// }

impl From<()> for Object {
    fn from(_unit: ()) -> Self {
        Self::nil()
    }
}

impl From<LuaRef> for Object {
    fn from(luaref: LuaRef) -> Self {
        Self {
            r#type: ObjectType::kObjectTypeLuaRef,
            data: ObjectData { luaref },
        }
    }
}

impl From<std::string::String> for Object {
    fn from(string: std::string::String) -> Self {
        NvimString::from_c_string(std::ffi::CString::new(string).unwrap())
            .into()
    }
}

impl From<NvimString> for Object {
    fn from(string: NvimString) -> Self {
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

impl<T: Into<Object>> From<Box<T>> for Object {
    fn from(boxed: Box<T>) -> Self {
        (*boxed).into()
    }
}

impl<T: Into<Object>> From<Vec<T>> for Object {
    fn from(vec: Vec<T>) -> Self {
        Self {
            r#type: ObjectType::kObjectTypeArray,
            data: ObjectData {
                array: ManuallyDrop::new(Array::from_iter(vec)),
            },
        }
    }
}

impl From<Array> for Object {
    fn from(array: Array) -> Self {
        Self {
            r#type: ObjectType::kObjectTypeArray,
            data: ObjectData { array: ManuallyDrop::new(array) },
        }
    }
}

impl TryFrom<Object> for bool {
    type Error = super::error::ConversionError;

    #[inline]
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        (matches!(obj.r#type, ObjectType::kObjectTypeBoolean))
            .then(|| unsafe { obj.data.boolean })
            .ok_or_else(|| ConversionError::Primitive {
                expected: ObjectType::kObjectTypeBoolean,
                got: obj.r#type,
            })
    }
}

impl TryFrom<Object> for LuaRef {
    type Error = super::error::ConversionError;

    #[inline]
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        (matches!(obj.r#type, ObjectType::kObjectTypeLuaRef))
            .then(|| unsafe { obj.data.luaref })
            .ok_or_else(|| ConversionError::Primitive {
                expected: ObjectType::kObjectTypeLuaRef,
                got: obj.r#type,
            })
    }
}

impl TryFrom<Object> for i64 {
    type Error = super::error::ConversionError;

    #[inline]
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        (matches!(obj.r#type, ObjectType::kObjectTypeInteger))
            .then(|| unsafe { obj.data.integer })
            .ok_or_else(|| ConversionError::Primitive {
                expected: ObjectType::kObjectTypeInteger,
                got: obj.r#type,
            })
    }
}

impl TryFrom<Object> for () {
    type Error = super::error::ConversionError;

    #[inline]
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        (matches!(obj.r#type, ObjectType::kObjectTypeNil))
            .then(|| ())
            .ok_or_else(|| ConversionError::Primitive {
                expected: ObjectType::kObjectTypeNil,
                got: obj.r#type,
            })
    }
}
