use std::ffi::CStr;
use std::fmt;

use libc::c_char;

use crate::object::ObjectType;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L62
#[derive(Debug, thiserror::Error)]
#[repr(C)]
pub struct Error {
    pub r#type: ErrorType,
    pub msg: *mut c_char,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L26
#[derive(Debug)]
#[allow(dead_code, non_camel_case_types)]
#[repr(C)]
pub enum ErrorType {
    kErrorTypeNone = -1,
    kErrorTypeException,
    kErrorTypeValidation,
}

impl Default for Error {
    fn default() -> Self {
        Self { r#type: ErrorType::kErrorTypeNone, msg: std::ptr::null_mut() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.msg.is_null() {
            let msg = unsafe { CStr::from_ptr(self.msg) }.to_string_lossy();
            write!(f, "{}", msg)
        } else {
            use ErrorType::*;
            let msg = match self.r#type {
                kErrorTypeNone => return Ok(()),
                kErrorTypeException => "exception",
                kErrorTypeValidation => "validation",
            };
            write!(f, "{}", msg)
        }
    }
}

impl Error {
    /// Returns `Ok(f())` if it's not actually an error, or moves into a
    /// generic `std::error::Error` if it is.
    pub fn into_err_or_else<F, Ok, Err>(
        self,
        f: F,
    ) -> std::result::Result<Ok, Err>
    where
        F: FnOnce() -> Ok,
        Err: std::error::Error + From<self::Error>,
    {
        (!self.is_err()).then(f).ok_or_else(|| self.into())
    }

    #[inline]
    pub const fn is_err(&self) -> bool {
        !matches!(self.r#type, ErrorType::kErrorTypeNone)
    }
}

/// Error that occurs when the conversion of an `Object` into another type
/// fails.
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error(
        "Object type expected to be \"{expected:?}\", but was \"{got:?}\""
    )]
    Primitive { expected: ObjectType, got: ObjectType },
}
