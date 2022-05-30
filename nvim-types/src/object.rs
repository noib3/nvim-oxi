use std::fmt;
use std::mem::ManuallyDrop;

use crate::{array::Array, dictionary::Dictionary, string::String, LuaRef};

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
    pub string: ManuallyDrop<String>,
    pub array: ManuallyDrop<Array>,
    pub dictionary: ManuallyDrop<Dictionary>,
    pub luaref: LuaRef,
}

impl Object {
    #[inline]
    pub const fn nil() -> Self {
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
        use ObjectType::*;
        match self.r#type {
            kObjectTypeNil => Self::nil(),
            kObjectTypeBoolean => clone_copy!(self, boolean),
            kObjectTypeInteger => clone_copy!(self, integer),
            kObjectTypeFloat => clone_copy!(self, float),
            kObjectTypeString => clone_drop!(self, string, String),
            kObjectTypeArray => clone_drop!(self, array, Array),
            kObjectTypeDictionary => clone_drop!(self, dictionary, Dictionary),
            kObjectTypeLuaRef => clone_copy!(self, luaref),
        }
    }
}

// impl Drop for Object {
//     fn drop(&mut self) {
//         use ObjectType::*;
//         match self.r#type {
//             kObjectTypeString => unsafe {
//                 ManuallyDrop::drop(&mut self.data.string)
//             },

//             kObjectTypeArray => unsafe {
//                 ManuallyDrop::drop(&mut self.data.array)
//             },

//             kObjectTypeDictionary => unsafe {
//                 ManuallyDrop::drop(&mut self.data.dictionary)
//             },

//             _ => {},
//         }
//     }
// }
