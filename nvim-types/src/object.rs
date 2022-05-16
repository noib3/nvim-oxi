use std::mem::ManuallyDrop;

use super::array::Array;
use super::dictionary::Dictionary;
use super::string::String;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L115
#[repr(C)]
pub(crate) struct Object {
    r#type: ObjectType,
    data: ObjectData,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L100
#[allow(non_camel_case_types)]
#[repr(C)]
enum ObjectType {
    kObjectTypeNil = 0,
    kObjectTypeBoolean,
    kObjectTypeInteger,
    kObjectTypeFloat,
    kObjectTypeString,
    kObjectTypeArray,
    kObjectTypeDictionary,
    kObjectTypeLuaRef,
    // EXT types, cannot be split or reordered, see #EXT_OBJECT_TYPE_SHIFT
    kObjectTypeBuffer,
    kObjectTypeWindow,
    kObjectTypeTabpage,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L117
#[repr(C)]
pub union ObjectData {
    boolean: bool,
    integer: i64,
    float: f64,
    string: ManuallyDrop<String>,
    array: ManuallyDrop<Array>,
    dictionary: ManuallyDrop<Dictionary>,
    luaref: isize,
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
