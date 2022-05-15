use std::ffi::CStr;
use std::fmt;
use std::os::raw::c_char;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Exception(String),

    #[error("{0}")]
    Validation(String),
}

impl TryFrom<NvimError> for Error {
    type Error = &'static str;

    fn try_from(err: NvimError) -> Result<Self, Self::Error> {
        use ErrorType::*;
        match err.r#type {
            kErrorTypeException => Ok(Self::Exception(err.to_string())),
            kErrorTypeValidation => Ok(Self::Validation(err.to_string())),
            kErrorTypeNone => Err("not an error!"),
        }
    }
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L62
#[repr(C)]
pub(crate) struct NvimError {
    r#type: ErrorType,
    msg: *mut c_char,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L26
#[allow(non_camel_case_types)]
#[repr(C)]
enum ErrorType {
    kErrorTypeNone = -1,
    kErrorTypeException,
    kErrorTypeValidation,
}

impl Default for NvimError {
    fn default() -> Self {
        Self { r#type: ErrorType::kErrorTypeNone, msg: std::ptr::null_mut() }
    }
}

impl fmt::Display for NvimError {
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

impl NvimError {
    /// TODO: docs
    pub(crate) fn into_err_or<T, F>(self, f: F) -> Result<T, Error>
    where
        F: FnOnce() -> T,
    {
        if self.is_err() {
            Err(self.try_into().unwrap())
        } else {
            Ok(f())
        }
    }

    /// TODO: docs
    const fn is_err(&self) -> bool {
        !matches!(self.r#type, ErrorType::kErrorTypeNone)
    }
}
