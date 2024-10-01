use std::ffi::c_int;

use thiserror::Error as ThisError;

use crate::utils;

#[derive(Clone, Debug, Eq, PartialEq, ThisError, Hash)]
pub enum Error {
    #[error(
        "Value of type {ty} couldn't be popped from the stack{}.",
        message.as_ref().map(|msg| format!(": {msg}")).unwrap_or_default()
    )]
    PopError { ty: &'static str, message: Option<String> },

    #[error(
        "Value of type {ty} couldn't be pushed on the stack{}.",
        message.as_ref().map(|msg| format!(": {msg}")).unwrap_or_default()
    )]
    PushError { ty: &'static str, message: Option<String> },

    #[error("Lua runtime error: {0}")]
    RuntimeError(String),

    #[error("Lua memory error: {0}")]
    MemoryError(String),

    #[error("TODO")]
    PopEmptyStack,
}

impl Error {
    pub fn pop_error<M: Into<String>>(ty: &'static str, message: M) -> Self {
        Self::PopError { ty, message: Some(message.into()) }
    }

    pub fn pop_error_from_err<T, E: std::error::Error>(err: E) -> Self {
        Self::PopError {
            ty: std::any::type_name::<T>(),
            message: Some(err.to_string()),
        }
    }

    pub fn pop_wrong_type<T>(expected: c_int, found: c_int) -> Self {
        let ty = std::any::type_name::<T>();
        let expected = utils::type_name(expected);
        let found = utils::type_name(found);
        Self::PopError {
            ty,
            message: Some(format!(
                "expected a {}, found a {} instead",
                expected, found
            )),
        }
    }

    pub unsafe fn pop_wrong_type_at_idx<T>(
        lstate: *mut crate::ffi::State,
        idx: std::ffi::c_int,
    ) -> Self {
        let expected = std::any::type_name::<T>();
        let got = crate::utils::debug_type(lstate, idx);

        Self::PopError {
            ty: expected,
            message: Some(format!(
                "expected {}, got {} instead",
                expected, got
            )),
        }
    }

    pub fn push_error<M: Into<String>>(ty: &'static str, message: M) -> Self {
        Self::PushError { ty, message: Some(message.into()) }
    }

    pub fn push_error_from_err<T, E: std::error::Error>(err: E) -> Self {
        Self::PushError {
            ty: std::any::type_name::<T>(),
            message: Some(err.to_string()),
        }
    }
}
