use std::ffi::CStr;
use std::fmt;

use libc::c_char;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L62
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
    pub fn into_err_or_else<Ok, F, Err, TryFromErr>(
        self,
        f: F,
    ) -> Result<Ok, Err>
    where
        F: FnOnce() -> Ok,
        Err: std::error::Error + TryFrom<self::Error, Error = TryFromErr>,
        TryFromErr: std::fmt::Debug,
    {
        if self.is_err() {
            Err(self.try_into().unwrap())
        } else {
            Ok(f())
        }
    }

    const fn is_err(&self) -> bool {
        !matches!(self.r#type, ErrorType::kErrorTypeNone)
    }
}
