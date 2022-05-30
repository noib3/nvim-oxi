use std::error::Error as StdError;
use std::ffi::CStr;
use std::fmt;
use std::result::Result as StdResult;

use libc::c_char;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L62
#[derive(thiserror::Error)]
#[repr(C)]
pub struct Error {
    pub r#type: ErrorType,
    pub msg: *mut c_char,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L26
#[allow(dead_code, non_camel_case_types)]
#[repr(C)]
pub enum ErrorType {
    kErrorTypeNone = -1,
    kErrorTypeException,
    kErrorTypeValidation,
}

impl Error {
    pub const fn new() -> Self {
        Self { r#type: ErrorType::kErrorTypeNone, msg: std::ptr::null_mut() }
    }
}

impl Default for Error {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
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
    pub fn into_err_or_else<Ok, Err, F>(self, f: F) -> StdResult<Ok, Err>
    where
        Err: StdError + From<self::Error>,
        F: FnOnce() -> Ok,
    {
        (!self.is_err()).then(f).ok_or_else(|| self.into())
    }

    pub fn into_err_or_flatten<Ok, Err, F>(self, f: F) -> StdResult<Ok, Err>
    where
        Err: StdError + From<self::Error>,
        F: FnOnce() -> StdResult<Ok, Err>,
    {
        (!self.is_err()).then(f).ok_or_else(|| self.into())?
    }

    #[inline]
    pub const fn is_err(&self) -> bool {
        !matches!(self.r#type, ErrorType::kErrorTypeNone)
    }
}
