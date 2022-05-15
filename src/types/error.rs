use std::os::raw::c_char;
use thiserror::Error;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L26
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum ErrorType {
    kErrorTypeNone = -1,
    kErrorTypeException,
    kErrorTypeValidation,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L62
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub(crate) struct Error {
    r#type: ErrorType,
    msg: *mut c_char,
}

impl Default for Error {
    fn default() -> Self {
        Self { r#type: ErrorType::kErrorTypeNone, msg: std::ptr::null_mut() }
    }
}
